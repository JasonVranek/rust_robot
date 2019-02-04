import socket
import json

class Message():
	def __init__(self, json_msg):
		self.dest = ("192.168.1.123", 5000)
		self.msg = json.dumps(json_msg).encode()

	def send(self):
		# create an INET, STREAMing socket
		s = socket.socket(socket.AF_INET, socket.SOCK_STREAM) 
		s.connect(self.dest)
		s.sendall(self.msg)
		s.close()

if __name__ == '__main__':
	msg = Message({ "order_type": "move", "device": "robot", "cmd": 1 })
	msg.send()