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
    while True:
        demo.loop()

        server_socket.listen()

        try:
            conn, addr = server_socket.accept()
        except socket.timeout:
            conn = None
            print("no one")

        if conn != None:
            print('Connected by', addr)
            conn.sendall("listening".encode())

            try:
                data = conn.recv(1024)
                print("received:", data)
            except socket.timeout:
                print("no message yet")


if __name__ == "__main__":
    server_socket = start_server()
    app_loop(server_socket)