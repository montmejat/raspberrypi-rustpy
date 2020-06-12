import socket
import demo
import os
import signal
import sys
import threading
import dill
import copy
from types import ModuleType


def signal_handler(sig, frame):
    print(' ** Closing Lamp Control ** ')

    try:
        demo.end()
    except AttributeError:
        pass

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


def start_server():
    host = '127.0.0.1'
    port = 10000

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


def app_loop(server_socket):
    pause = False

    while True:
        if not pause:
            demo.loop()

        server_socket.listen()

        try:
            conn, addr = server_socket.accept()
        except socket.timeout:
            conn = None

        if conn != None:
            print(' - Connected by', addr)
            conn.sendall('listening'.encode())

            try:
                data = conn.recv(1024)
                data = data.decode('utf-8')

                if data == 'pause':
                    print('   * CLIENT : Paused server *')
                    pause = True
                elif data == 'unpause':
                    print('   * CLIENT : Looping back up again *')
                    pause = False
                elif data == 'restart':
                    print('   * CLIENT : Restarting *')
                    pause = False
                    demo.start()
                elif 'save' in data:
                    if ':' in data:
                        _, filename = data.split(':')
                        print('   * CLIENT : Saving variables to', filename, '*')
                        save_variables(filename)
                    else:
                        print("   * CLIENT : Saving variables to 'demo_vars.pkl'")
                        save_variables()
                elif 'load' in data:
                    if ':' in data:
                        _, filename = data.split(':')
                        print('   * CLIENT : Loading variables from', filename, '*')
                        load_variables(filename)
                    else:
                        print("   * CLIENT : Loading variables from 'demo_vars.pkl' *")
                        load_variables()
                elif 'func:' in data:
                    function_name = data.replace('func:', '')
                    print('   * CLIENT : Calling', function_name)
                    getattr(demo, function_name)()
                elif 'var:' in data:
                    var_and_value = data.replace('var:', '')
                    var_name, value = var_and_value.split('=')
                    print('   * CLIENT : Modifying', var_name, 'to', value)
                    var_type = type(getattr(demo, var_name))
                    value = var_type(value)
                    setattr(demo, var_name, value)

            except socket.timeout:
                pass


if __name__ == '__main__':
    server_socket = start_server()
    app_loop(server_socket)