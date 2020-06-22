import cbor, zmq

context = zmq.Context()
socket = context.socket(zmq.REQ)

# send request
socket.connect("tcp://localhost:5555")
message = { 'type': 'action', 'value': 'restart', 'key': 'qsdfq' }
socket.send(cbor.dumps(message))

# get response 
print(socket.recv())