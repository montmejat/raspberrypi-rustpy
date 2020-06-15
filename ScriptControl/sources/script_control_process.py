import socket
import demo
import os
import signal
import sys
import threading
import dill
import copy
import threading
import queue
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
    host = '127.0.0.1'
    port = 10000

    if print_debug:
        print(' ** Lamp Control Started ** ')
    signal.signal(signal.SIGINT, signal_handler)

    socket.setdefaulttimeout(2)
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((host, port))

    try:
        demo.start()
    except AttributeError:
        print("Warning, 'start' function does not exists.")

    if not 'loop' in dir(demo):
        print("Error, demo.py needs a 'loop' function.")
        sys.exit(0)
    
    return server_socket


def thread_server(server_socket, pause_queue, print_debug=True, send_debug_to_client=True):
    pause = False

    while True:
        server_socket.listen()

        try:
            conn, addr = server_socket.accept()
        except socket.timeout:
            conn = None

        message = ''
        if conn != None:
            if print_debug:
                print(' - Connected by', addr)
            conn.sendall('listening'.encode())

            try:
                data = conn.recv(1024)
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

            except socket.timeout:
                pass
            
            if send_debug_to_client:
                conn.sendall(message.encode())
        
        if print_debug and message != '':
            print('   * CLIENT :', message, '*')
        
        pause_queue.put(pause)


def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
    pause_queue = queue.Queue()
    pause_queue.put(False)

    thread = threading.Thread(target=thread_server, args=(server_socket, pause_queue, print_debug, send_debug_to_client))
    thread.daemon = True
    thread.start()

    while True:
        pause = pause_queue.get()
        if not pause:
            demo.loop()


if __name__ == '__main__':
    server_socket = start_server(print_debug=True)
    app_loop(server_socket, print_debug=True)