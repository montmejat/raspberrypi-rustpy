import socket
import zmq

context = zmq.Context()

socket = context.socket(zmq.REQ)
socket.connect("tcp://localhost:5555")

while True:
    message = input("send:")

    socket.send(message.encode())

    data = socket.recv()
    print("received:", data)
        
    print()
