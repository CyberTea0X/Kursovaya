import socket, threading, pymysql, urllib, random, string
import datetime
import requests
import config

LOCALHOST = "192.168.31.46"
PORT = config.con_port

server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

server.bind((LOCALHOST, PORT))
print('Server started successfully')


class ClientThread(threading.Thread):
    def __init__(self, clientAddress, clientsocket):
        threading.Thread.__init__(self)
        self.csocket = clientsocket
        print(f'new connection: address: {clientAddress}')

    def run(self):
        msg = ''
        while True:
            data = self.csocket.recv(4096)
            msg = data.decode()
            # print(msg)
            msg = str(msg).split('℻')
            if msg[0] == '':
                print('diconnection')
                break

            elif msg[0] == '@auth':
                self.csocket.send(bytes('Происходит авторизация', 'UTF-8'))
                self.email = msg[1]
                self.password = msg[2]
                print(f'логин {self.email}pass {self.password}')

            elif msg[0] == '@registration':
                self.csocket.send(bytes('Происходит регистрация', 'UTF-8'))
                self.name = str(msg[1])
                self.surname = str(msg[2])
                self.password = str(msg[3])
                self.email = str(msg[4])
                self.logo = str(msg[5])
                self.about = str(msg[6])

                self.red_data = datetime.datetime.now().date()
                # self.csocket.send(bytes(f'логин {self.login}, pass {self.password}', 'UTF-8'))
                # print(f'мыло {self.email}\npass {self.password}')

                rand_text = [
                    random.choice(string.ascii_lowercase + string.digits if i != 5 else string.ascii_uppercase) for
                    i in range(32)]
                self.random_name = ''.join(rand_text)
                img_data = requests.get(self.logo).content
                with open(str(self.random_name) + '.png', 'wb') as handler:
                    handler.write(img_data)
                    print(f'Image saved successfully as: {self.random_name}.jpg')
                try:
                    connection = pymysql.connect(host=config.host,
                                                 port=config.dbport,
                                                 user=config.user,
                                                 password=config.password,
                                                 database=config.db,
                                                 cursorclass=pymysql.cursors.DictCursor
                                                 )
                    print('connected to db')

                    try:
                        with connection.cursor() as cursor:
                            # id_now = cursor.execute("SELECT `id` FROM `users WHERE MAX(id) FROM users`")
                            insert_query = f"INSERT INTO `users`(id, first_name, last_name, email, logo_id, " \
                                           "raiting, about_user, chats_folder, login, password, reg_date, " \
                                           f"is_online) VALUES ('','{self.name}','{self.surname}','{self.email}'," \
                                           f"'{self.random_name}.jpg','0','{self.about}','CHAT_FOLDER_TO_DO','?', " \
                                           f"'{self.password}','{self.red_data}','1');"
                            cursor.execute(insert_query)
                            connection.commit()
                    finally:
                        connection.close()

                except Exception as ex:
                    print(f'CONNECTION FAILED \n {ex}')


while True:
    server.listen(1)
    clientsock, clientAddress = server.accept()
    newthread = ClientThread(clientAddress, clientsock)
    newthread.start()
