import socket
import zmq

context = zmq.Context()

socket = context.socket(zmq.REQ)
socket.connect("ipc://demoserver")

while True:
    message = input("send:")

    socket.send(message.encode())

    data = socket.recv()
    print("received:", data)
        
    print()
