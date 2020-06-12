import socket

HOST = '127.0.0.1'
PORT = 10000

while True:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        message = input("send:")
        s.connect(('', PORT))

        data = s.recv(1024)
        print("received:", data)
        if data == "listening".encode():
            print("server is listening, sending message")
            s.send(message.encode())

            data = s.recv(1024)
            print("received:", data)
        
    print()
