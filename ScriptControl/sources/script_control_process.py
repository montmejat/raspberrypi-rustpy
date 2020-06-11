import socket
import demo
import os
import signal
import sys
import threading

HOST = '127.0.0.1'
PORT = 10000

def signal_handler(sig, frame):
    print(' ** Closing Lamp Control ** ')
    sys.exit(0)


def start_server():
    print(" ** Lamp Control Started ** ")
    signal.signal(signal.SIGINT, signal_handler)

    socket.setdefaulttimeout(2)
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((HOST, PORT))

    try:
        demo.start()
    except AttributeError:
        print("Warning, 'start' function does not exists.")

    if not "loop" in dir(demo):
        print("Error, demo.py needs a 'loop' function.")
    
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
            print(" - Connected by", addr)
            conn.sendall("listening".encode())

            try:
                data = conn.recv(1024)
                data = data.decode("utf-8")

                if data == "pause":
                    pause = True
                    print("   * CLIENT : Paused server *")
                elif data == "unpause":
                    pause = False
                    print("   * CLIENT : Loop back up again *")
                elif data == "restart":
                    pause = False
                    print("   * CLIENT : Restart *")
                    demo.start()
                elif "func:" in data:
                    function_name = data.replace("func:", "")
                    print("   * CLIENT : Calling", function_name)
                    getattr(demo, function_name)()
                elif "var:" in data:
                    var_and_value = data.replace("var:", "")
                    var_name, value = var_and_value.split("=")
                    print("   * CLIENT : Modifying", var_name, "to", value)
                    var_type = type(getattr(demo, var_name))
                    value = var_type(value)
                    setattr(demo, var_name, value)

            except socket.timeout:
                pass


if __name__ == "__main__":
    server_socket = start_server()
    app_loop(server_socket)