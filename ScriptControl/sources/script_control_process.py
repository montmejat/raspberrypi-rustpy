import demo
import os, signal, sys, threading, dill, copy, threading, queue, zmq
from types import ModuleType


def signal_handler(sig, frame):
    print(' CLOSING : ** Closing Script Control ** ')

    try:
        demo.end()
    except AttributeError:
        print("No end() method")

    sys.exit(0)


def save_variables(filename='demo_vars.pkl'):
    outfile = open(filename, 'wb')
    variable_names = [variable for variable in dir(demo) if not variable.startswith('__')]
    variables = {}

    for variable in variable_names:
        if not callable(getattr(demo, variable)) and not isinstance(getattr(demo, variable), ModuleType) and variable != 'led':
            try:
                copy = getattr(demo, variable)
                print("      Copying:", copy)
                variables[variable] = copy
            except TypeError:
                print("      Can't dill:", copy)

    dill.dump(variables, outfile)
    outfile.close()


def load_variables(filename='demo_vars.pkl'):
    infile = open(filename, 'rb')
    variables = dill.load(infile)
    print('      Variables:', variables)

    for var in variables:
        setattr(demo, var, variables[var])

    infile.close()


def start_server(print_debug=True):
    if print_debug:
        print(' ** Lamp Control Started ** ')
    signal.signal(signal.SIGINT, signal_handler)

    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind("tcp://*:5555")

    try:
        demo.start()
    except AttributeError:
        print("Warning, 'start' function does not exists.")

    if not 'loop' in dir(demo):
        print("Error, demo.py needs a 'loop' function.")
        sys.exit(0)
    
    return socket


def thread_server(socket, print_debug=True, send_debug_to_client=True):
    global pause

    while True:
        event = socket.poll(500)

        message = ''
        if event != 0:
            data = socket.recv()
            data = data.decode('utf-8')

            if data == 'pause':
                message = "Paused server"
                pause = True
            elif data == 'unpause':
                message = 'Looping back up again'
                pause = False
            elif data == 'restart':
                message = 'Restarting'
                pause = False
                demo.start()
            elif 'save' in data:
                if ':' in data:
                    _, filename = data.split(':')
                    message = 'Saving variables to' + filename
                    save_variables(filename)
                else:
                    message = 'Saving variables to demo_vars.pkl'
                    save_variables()
            elif 'load' in data:
                if ':' in data:
                    _, filename = data.split(':')
                    message = 'Loading variables from' + filename
                    load_variables(filename)
                else:
                    message = 'Loading variables from demo_vars.pkl'
                    load_variables()
            elif 'func:' in data:
                function_name = data.replace('func:', '')
                message = 'Calling' + function_name
                getattr(demo, function_name)()
            elif 'var:' in data:
                var_and_value = data.replace('var:', '')
                var_name, value = var_and_value.split('=')
                message = 'Modifying' + var_name + 'to' + value

                try:
                    if '(' and ')' in data:
                        value = value.replace('(', '').replace(')', '')

                        if 'int' in data:
                            var_type = 'int'
                            value = value.replace('int', '')
                            value = int(value)
                            setattr(demo, var_name, value)
                        elif 'float' in data:
                            var_type = 'float'
                            value = value.replace('float', '')
                            value = float(value)
                            setattr(demo, var_name, value)
                        elif 'str' in data:
                            var_type = 'str'
                            value = value.replace('str', '').replace("'", '').replace('"', '')
                            value = str(value)
                            setattr(demo, var_name, value)
                        elif 'bool' in data:
                            var_type = 'bool'
                            value = value.replace('bool', '')
                            value = bool(value)
                            setattr(demo, var_name, value)
                    else:
                        var_type = type(getattr(demo, var_name))
                        value = var_type(value)
                        setattr(demo, var_name, value)
                except ValueError:
                    message = '      Cannot modify' + var_name + 'to' + value, 'of the type' + var_type
            else:
                message = 'Command not correct'

            if send_debug_to_client:
                socket.send(message.encode())

        if print_debug and message != '':
            print('   * CLIENT :', message, '*')


def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
    thread = threading.Thread(target=thread_server, args=(server_socket, print_debug, send_debug_to_client))
    thread.daemon = True
    thread.start()

    while True:
        if not pause:
            demo.loop()

pause = False
if __name__ == '__main__':
    server_socket = start_server(print_debug=True)
    app_loop(server_socket, print_debug=True)