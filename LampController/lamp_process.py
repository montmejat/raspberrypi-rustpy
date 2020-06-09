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


def start():
    print(" ** Lamp Control Started ** ")
    signal.signal(signal.SIGINT, signal_handler)

    try:
        demo.start()
    except AttributeError:
        print("Warning, 'start' function does not exists.")

    if not "loop" in dir(demo):
        print("Error, demo.py needs a 'loop' function.")


def app_loop():
    while True:
        demo.loop()


def app():
    thread = threading.Thread(target=app_loop())
    thread.start()

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', PORT))
        s.listen()

        while True:
            conn, addr = s.accept()

            with conn:
                print('Connected by', addr)
                while True:
                    data = conn.recv(1024)
                    if not data:
                        break
                    conn.sendall(data)


start()
app()