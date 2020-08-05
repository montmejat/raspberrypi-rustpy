import os, signal, sys, cbor, zmq, hashlib, time, serial
from threading import Thread
from types import ModuleType

sys.path.insert(1, '/home/pi/raspberrypi-rustpy/ScriptControl/scriptcontrol/demo/')
import demo

class CommunicationsThread(Thread):
    def __init__(self, socket, print_debug=True, send_debug_to_client=True):
        super().__init__(daemon=True)

        self.socket = socket
        self.print_debug = print_debug
        self.send_debug_to_client = send_debug_to_client
        self.hash_key = hashlib.sha256('test'.encode()).hexdigest()
        self.leds_count = 64
        self.leds = demo.Led(self.leds_count) # TODO: make sure that this object stays in sync to the one at demo.py

        try:
            self.port = serial.Serial("/dev/rfcomm0", baudrate=9600)
        except Exception:
            self.port = None
            print("\nBLUETOOTH ERROR! Make sure you connected to the bluetooth device.\nTry using: 'sudo rfcomm connect hci0 hc05_addr'.\n")
        
        if self.port != None:
            self.port.write(b'#') # start message

            for i in range(64):
                led = self.leds.get(i)
                self.port.write(bytes(led.green))
                self.port.write(bytes(led.red))
                self.port.write(bytes(led.blue))
            
            self.port.write(b'?') # stop message

    def run(self):
        global pause

        while True:
            event_count = self.socket.poll(1000)

            message = ''
            if event_count != 0:
                data = self.socket.recv()
                data = cbor.loads(data)

                if data['type'] == 'action':
                    if data['value'] == 'pause':
                        message = "Paused server"
                        pause = True

                    elif data['value'] == 'unpause':
                        message = 'Looping back up again'
                        pause = False

                    elif data['value'] == 'restart':
                        key = data['key']
                        key = hashlib.sha256(key.encode()).hexdigest()
                        if key == self.hash_key:
                            message = 'Restarting'
                            pause = False
                            demo.start()
                        else:
                            message = 'Key not correct!'
                        
                    elif data['value'] == 'save':
                        if 'filename' in data.keys():
                            filename = data.split('filename')
                            message = 'Saving variables to' + filename
                            save_variables(filename)
                        else:
                            message = 'Saving variables to demo_vars.pkl'
                            save_variables()
                        
                    elif data['value'] == 'load':
                        if ':' in data:
                            _, filename = data.split(':')
                            message = 'Loading variables from' + filename
                            load_variables(filename)
                        else:
                            message = 'Loading variables from demo_vars.pkl'
                            load_variables()

                elif data['type'] == 'get':
                    if data['value'] == 'state':
                        paused = 'false'
                        if pause:
                            paused = 'true'

                        message = cbor.dumps({ 'paused': paused })
                        self.socket.send(message)
                        continue
                    
                    if data['value'] == 'settings':
                        variable_names = [variable for variable in dir(demo.param) if not (variable.startswith('__') or variable == 'SliderValue')]
                        variables = {}

                        for variable_name in variable_names:
                            variable = getattr(demo.param, variable_name)

                            if type(variable) == demo.Settings.SliderValue:
                                variables[variable_name] = str(str(variable.min) + ':' + str(variable.value) + ':' + str(variable.max))
                            else:
                                variables[variable_name] = str(variable)
                        
                        print("   * CLIENT : requested settings '", variables, " *")

                        message = cbor.dumps(variables)
                        self.socket.send(message)
                        continue

                elif data['type'] == 'call':
                    function_name = data['value']
                    message = 'Calling ' + function_name
                    getattr(demo, function_name)()

                elif data['type'] == 'set':
                    var_name = data['var']
                    value = data['value']
                    message = 'Modifying ' + var_name + ' to ' + value

                    if 'cast' in data.keys():
                        try:
                            if data['cast'] == 'int':
                                value = int(value)
                                setattr(demo.param, var_name, value)
                            elif data['cast'] == 'float':
                                value = float(value)
                                setattr(demo.param, var_name, value)
                            elif data['cast'] == 'str':
                                value = str(value)
                                setattr(demo.param, var_name, value)
                            elif data['cast'] == 'bool':
                                value = bool(value)
                                setattr(demo.param, var_name, value)
                        except ValueError:
                            message = '      Cannot modify' + var_name + 'to' + value, 'of the type' + data['cast']
                    else:
                        var_type = type(getattr(demo.param, var_name))
                        if var_type == demo.Settings.SliderValue:
                            slider = getattr(demo.param, var_name)
                            slider.value = value
                            setattr(demo.param, var_name, slider)
                        else:
                            value = var_type(value)
                            setattr(demo.param, var_name, value)
                else:
                    message = 'Command not correct'

                if self.send_debug_to_client:
                    self.socket.send(message.encode())

            if self.print_debug and message != '':
                print('   * CLIENT :', message, '*')


def signal_handler(sig, frame):
    print(' CLOSING : ** Closing Script Control ** ')

    try:
        demo.end()
    except AttributeError:
        print("No end() method")

    sys.exit(0)


def save_variables(filename='demo_vars'):
    outfile = open(filename, 'wb')
    variable_names = [variable for variable in dir(demo) if not variable.startswith('__')]
    variables = {}

    for variable_name in variable_names:
        variable = getattr(demo, variable_name)
        if not callable(variable) and not isinstance(variable, ModuleType) and (type(variable) in [ int, bool, str, str, dict, list ]):
            try:
                variables[variable_name] = variable
            except TypeError:
                print("      Can't encode with cbor:", copy)

    try: 
        cbor.dump(variables, outfile)
    except Exception:
        print("      Error during save...")
    
    outfile.close()


def load_variables(filename='demo_vars'):
    infile = open(filename, 'rb')
    variables = cbor.load(infile)

    for var in variables:
        setattr(demo, var, variables[var])

    infile.close()


def start_server(print_debug=True):
    global port

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


def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
    global pause
    pause = False

    thread = CommunicationsThread(server_socket, print_debug, send_debug_to_client)
    thread.start()

    while True:
        if not pause:
            demo.loop()