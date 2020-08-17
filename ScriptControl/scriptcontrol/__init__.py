import os, signal, sys, cbor, zmq, hashlib, time, serial, struct
from threading import Thread, Lock
from types import ModuleType

sys.path.insert(1, '/home/pi/raspberrypi-rustpy/ScriptControl/scriptcontrol/demo/')
import luminolib

mode = "demo"
mode_library = __import__(mode)

class CommunicationsThread(Thread):
    def __init__(self, socket, print_debug=True, send_debug_to_client=True):
        super().__init__(daemon=True)

        self.socket = socket
        self.print_debug = print_debug
        self.send_debug_to_client = send_debug_to_client
        self.hash_key = hashlib.sha256('test'.encode()).hexdigest()
        self.leds_count = 22
        self.leds = luminolib.Led(self.leds_count)

        try:
            self.port = serial.Serial("/dev/rfcomm0", baudrate=9600)
        except Exception:
            self.port = None
            print("\nBLUETOOTH ERROR! Make sure you connected to the bluetooth device.\nTry using: 'sudo rfcomm connect hci0 hc05_addr'.\n")
        
        if self.port != None:
            self.port.write(b'#') # start message
            self.port.write(b'&') # tell that its to send data

            for i in range(self.leds_count):
                led = self.leds.get(i)
                self.port.write(struct.pack('=B', led.green))
                self.port.write(struct.pack('=B', led.red))
                self.port.write(struct.pack('=B', led.blue))
            
            self.port.write(b'?') # stop message

            # TODO: try to flush the port

    def sync_leds(self, leds):
        if self.port == None:
            return

        lock = Lock()

        lock.acquire()
        self.leds = leds
        self.port.write(b'#')
        self.port.write(b'&') # tell that its to send data

        for i in range(self.leds_count):
            led = self.leds.get(i)
            self.port.write(struct.pack('=B', led.green))
            self.port.write(struct.pack('=B', led.red))
            self.port.write(struct.pack('=B', led.blue))

        self.port.write(b'?')
        lock.release()

    def run(self):
        global pause, mode, mode_library

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

                            mode_library.start()
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
                    elif data['value'] == 'settings':
                        variable_names = [variable for variable in dir(mode_library.param) if not (variable.startswith('__') or variable == 'SliderValue')]
                        variables = {}

                        for variable_name in variable_names:
                            variable = getattr(mode_library.param, variable_name)

                            if type(variable) == luminolib.Settings.SliderValue:
                                variables[variable_name] = str(str(variable.min) + ':' + str(variable.value) + ':' + str(variable.max))
                            else:
                                variables[variable_name] = str(variable)
                        
                        print("   * CLIENT : requested settings '", variables, " *")

                        message = cbor.dumps(variables)
                        self.socket.send(message)
                        continue
                    elif data['value'] == 'leds':
                        leds_as_dict = []
                        
                        for i in range(self.leds_count):
                            leds_as_dict.append({ 'led': str(i), 'green': str(self.leds.get(i).green), 'red': str(self.leds.get(i).red), 'blue': str(self.leds.get(i).blue) })

                        message = cbor.dumps(leds_as_dict)
                        self.socket.send(message)
                        continue
                    elif data['value'] == 'mode':
                        message = cbor.dumps(mode)
                        self.socket.send(message)
                        continue
                    
                elif data['type'] == 'call':
                    function_name = data['value']
                    message = 'Calling ' + function_name
                    getattr(mode_library, function_name)()

                elif data['type'] == 'set':
                    if 'leds' in data.keys():
                        leds_modified = '['
                        for led_data in data["leds"]:
                            led_number = led_data['var'].replace('led', '')
                            led = self.leds.get(int(led_number))
                            led.green = int(led_data['green'])
                            led.red = int(led_data['red'])
                            led.blue = int(led_data['blue'])
                            leds_modified = leds_modified + ' ' + str(led_number)
                        leds_modified = leds_modified + ' ]'

                        self.port.write(b'#')
                        self.port.write(b'&') # tell that its to send data
                        for i in range(self.leds_count):
                            led = self.leds.get(i)
                            self.port.write(struct.pack('=B', led.green))
                            self.port.write(struct.pack('=B', led.red))
                            self.port.write(struct.pack('=B', led.blue))
                        self.port.write(b'?')
                    elif 'var' in data.keys():
                        var_name = data['var']
                        value = data['value']
                        message = 'Modifying ' + var_name + ' to ' + value

                        if 'cast' in data.keys():
                            try:
                                if data['cast'] == 'int':
                                    value = int(value)
                                    setattr(mode_library.param, var_name, value)
                                elif data['cast'] == 'float':
                                    value = float(value)
                                    setattr(mode_library.param, var_name, value)
                                elif data['cast'] == 'str':
                                    value = str(value)
                                    setattr(mode_library.param, var_name, value)
                                elif data['cast'] == 'bool':
                                    value = bool(value)
                                    setattr(mode_library.param, var_name, value)
                            except ValueError:
                                message = '      Cannot modify' + var_name + 'to' + value, 'of the type' + data['cast']
                        else:
                            var_type = type(getattr(mode_library.param, var_name))
                            if var_type == mode_library.Settings.SliderValue:
                                slider = getattr(mode_library.param, var_name)
                                slider.value = value
                                setattr(mode_library.param, var_name, slider)
                            else:
                                value = var_type(value)
                                setattr(mode_library.param, var_name, value)
                    elif 'mode' in data.keys():
                        if mode == 'rainbow':
                            self.port.write(b'#') # stop looping
                        
                        if data['mode'] == 'rainbow':
                            self.port.write(b'#')
                            self.port.write(b'/') # rainbow code mode 

                        mode = data['mode']
                        mode_library = __import__(mode)
                        mode_library.start()
                        message = '      Mode changed to ' + mode
                        
                else:
                    message = 'Command not correct'

                if self.send_debug_to_client:
                    self.socket.send(message.encode())

            if self.print_debug and message != '':
                print('   * CLIENT :', message, '*')


def signal_handler(sig, frame):
    global mode

    print(' CLOSING : ** Closing Script Control ** ')
    
    try:
        mode_library.end()
    except AttributeError:
        print("No end() method")

    sys.exit(0)


def save_variables(filename='demo_vars'):
    global mode_library

    outfile = open(filename, 'wb')
    variable_names = [variable for variable in dir(mode_library) if not variable.startswith('__')]
    variables = {}

    for variable_name in variable_names:
        variable = getattr(mode_library, variable_name)
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
        setattr(mode_library, var, variables[var])

    infile.close()


def start_server(print_debug=True):
    global port, mode

    if print_debug:
        print(' ** Lamp Control Started ** ')
    signal.signal(signal.SIGINT, signal_handler)

    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind("tcp://*:5555")

    try:
        mode_library.start()
    except AttributeError:
        print("Warning, 'start' function does not exists.")

    if not 'loop' in dir(mode_library):
        print("Error, mode_library.py needs a 'loop' function.")
        sys.exit(0)
    
    return socket


def app_loop(server_socket, print_debug=True, send_debug_to_client=True):
    global pause, mode
    pause = False

    thread = CommunicationsThread(server_socket, print_debug, send_debug_to_client)
    thread.start()

    while True:
        if not pause:
            mode_library.loop()

            if mode != 'rainbow':
                thread.sync_leds(mode_library.led_matrix)