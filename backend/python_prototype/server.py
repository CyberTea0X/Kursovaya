import datetime
import pymysql
import random
import socket
import string
import threading
import os
import config
import requests
import time

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
    uuid = ''
    # ID работает криво, пока что нет идей как исправить. Прикол в том что он теперь берет id последнего подключившегося
    # юзера, что странно, поскольку при подключении пользователя класс с ним запихивается в отдельный поток

    def chatting(self):
        # self.csocket.send(bytes('TO_DO', 'UTF-8'))
        folder_id = f'id{ClientThread.uuid}'
        folder = f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}'
        if not os.path.exists(folder):

            self.csocket.send(bytes('Вы еще не начинали нискем диалог, создаю новую папку', 'UTF-8'))
            print(f'Creating new folder with dialogs to user {folder_id}')
            try:
                os.mkdir(f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}')
                print(f'Successfully created folder: {folder_id}')
                time.sleep(3)
                self.csocket.send(bytes('Успех', 'UTF-8'))

            except Exception as ex:
                print(f'FAIL:\n {ex}')
            print('flag')
            ClientThread.chatting(self)
        else:
            self.csocket.send(bytes('Кому напишем? использование: ', 'UTF-8'))
            pass


    def run(self):

        status = 'AUTHORIZED'
        while True:

            data = self.csocket.recv(4096)
            msg = data.decode()
            #℻
            if (msg.find('@registration') == 0) or (msg.find('@reauth') == 0) or (msg.find('@auth') == 0):
                msg = str(msg).split('℻')
            else:
                msg = str(msg).split(' ')
            print(msg)
            a = ''
            a = a.join(msg)
            a = a.split()

            # msg = str(msg).split('℻') if status == 'NON_AUTHORIZED' else print(msg)
            if msg[0] == '':
                print('diconnection')
                break

            elif msg[0] == '@auth':
                self.csocket.send(bytes('Происходит авторизация', 'UTF-8'))
                email = msg[1]
                password = msg[2]

                try:
                    connection = pymysql.connect(host=config.host,
                                                 port=config.dbport,
                                                 user=config.user,
                                                 password=config.password,
                                                 database=config.db,
                                                 cursorclass=pymysql.cursors.DictCursor
                                                 )
                    try:
                        with connection.cursor() as cursor:
                            client_ip = str(clientAddress[0])
                            cursor.execute(f"SELECT * FROM `users` WHERE email='{email}'")
                            result = cursor.fetchall()
                            mail = result[0]["email"]
                            pas = result[0]["password"]
                            ip = result[0]["ip"]
                            ClientThread.uuid = result[0]["id"]
                            print(f'Авторизация клиента id{ClientThread.uuid}')
                            print(f'Email {email}\npass {password}')
                            if (mail == email) and (pas == password) and (ip == client_ip):
                                self.csocket.send(bytes(f'ACCESS GRANTED\n', 'UTF-8'))
                                cursor.execute(f"UPDATE `users` SET authorized = 'AUTHORIZED' WHERE ip='{client_ip}'")
                                connection.commit()
                                cursor.execute(f"SELECT * FROM `users` WHERE ip='{client_ip}'")
                                result = cursor.fetchall()
                                status = result[0]["authorized"]
                                self.csocket.send(
                                    bytes(
                                        "-" * 40 + '\nКуда отправимся)? \n1. Профиль \n2. Чатик \n3. Список всех '
                                                   'участников\n' + "-" * 40,
                                        'UTF-8'))
                                print(f'ACCESS GRANTED FOR USER id{ClientThread.uuid}')

                                # break
                            else:
                                print(f'ACCESS DENIED FOR USER {email}, {client_ip}')
                                self.csocket.send(bytes(f'ACCESS DENIED', 'UTF-8'))
                        #break
                    finally:
                        # time.sleep(0.5)
                        connection.close()

                except Exception as ex:
                    print(f'CONNECTION FAILED \n {ex}')

            elif msg[0] == '@reauth':
                self.csocket.send(bytes('Происходит повторная авторизация', 'UTF-8'))
                email = msg[1]
                password = msg[2]
                print(f'Повторная авторизация пользователя {email} {clientAddress[0]}')
                print(f'Email {email}\npass {password}')

                try:
                    connection = pymysql.connect(host=config.host,
                                                 port=config.dbport,
                                                 user=config.user,
                                                 password=config.password,
                                                 database=config.db,
                                                 cursorclass=pymysql.cursors.DictCursor
                                                 )
                    try:
                        with connection.cursor() as cursor:
                            client_ip = str(clientAddress[0])
                            cursor.execute(f"SELECT * FROM `users` WHERE email='{email}'")
                            result = cursor.fetchall()
                            mail = result[0]["email"]
                            pas = result[0]["password"]
                            ip = result[0]["ip"]
                            if (mail == email) and (pas == password):
                                cursor.execute(f"UPDATE `users` SET ip='{client_ip}' WHERE ip='{ip}'")
                                connection.commit()
                                self.csocket.send(bytes(f'ACCESS GRANTED\n', 'UTF-8'))
                                self.csocket.send(
                                    bytes(
                                        "-" * 40 + '\nКуда отправимся)? \n1. Профиль \n2. Чатик \n3. Список всех '
                                                   'участников\n' + "-" * 40,
                                        'UTF-8'))
                            else:
                                print(f'ACCESS DENIED FOR USER {email}, {client_ip}')
                                self.csocket.send(bytes(f'ACCESS DENIED', 'UTF-8'))
                        #break
                    finally:
                        # time.sleep(0.5)
                        connection.close()

                except Exception as ex:
                    print(f'CONNECTION FAILED \n{ex}')

            elif msg[0] == '@registration':

                self.csocket.send(bytes('Происходит регистрация', 'UTF-8'))
                name = str(msg[1])
                surname = str(msg[2])
                password = str(msg[3])
                email = str(msg[4])
                logo = str(msg[5])
                about = str(msg[6])

                red_data = datetime.datetime.now().date()
                # self.csocket.send(bytes(f'логин {self.login}, pass {self.password}', 'UTF-8'))
                # print(f'мыло {self.email}\npass {self.password}')

                rand_text = [
                    random.choice(string.ascii_lowercase + string.digits if i != 5 else string.ascii_uppercase) for
                    i in range(32)]
                random_name = ''.join(rand_text)
                img_data = requests.get(logo).content
                with open(f'/Users/fedor/PycharmProjects/pythonProject14/user_images/{random_name}.png',
                          'wb') as handler:
                    handler.write(img_data)
                    print(f'Image saved successfully as: {random_name}.jpg')
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
                        client_ip = str(clientAddress[0])
                        with connection.cursor() as cursor:
                            insert_query = f"INSERT INTO `users`(id, first_name, last_name, email, logo_id, " \
                                           "raiting, about_user, chats_folder, authorized, password, reg_date, " \
                                           f"ip) VALUES ('','{name}','{surname}','{email}'," \
                                           f"'{random_name}.jpg','0','{about}','CHAT_FOLDER_TO_DO','?', " \
                                           f"'{password}','{red_data}','{client_ip}');"
                            cursor.execute(insert_query)
                            connection.commit()
                            # usr_id = cursor.execute("SELECT `id` FROM `users`")
                            # self.csocket.send(bytes(f'{usr_id}', 'UTF-8'))
                            # print(usr_id)
                    finally:
                        connection.close()

                except Exception as ex:
                    print(f'CONNECTION FAILED \n {ex}')
            elif (str(msg[1]) == 'Профиль') and (''.join(msg).find('@id') != -1):
                u_id = str(msg[2])[3:]
                try:
                    connection = pymysql.connect(host=config.host,
                                                 port=config.dbport,
                                                 user=config.user,
                                                 password=config.password,
                                                 database=config.db,
                                                 cursorclass=pymysql.cursors.DictCursor
                                                 )
                    try:
                        with connection.cursor() as cursor:
                            cursor.execute(f"SELECT * FROM `users` WHERE id='{u_id}'")
                            result = cursor.fetchall()
                            logo = result[0]["logo_id"] #
                            id = result[0]["id"] #
                            name = result[0]["first_name"] #
                            surname = result[0]["last_name"] #
                            rating = result[0]["raiting"]
                            about = result[0]["about_user"]
                            reg_date = result[0]["reg_date"]


                    finally:

                        connection.close()
                        time.sleep(0.5)
                        self.csocket.send(
                            bytes('\n\n' + '-' * 40 + f'\nПрофиль {rating}★ | {name} {surname} @id{id}' +
                                  f'\nЛоготип(фронт привет) {logo}\n' +
                                  '-' * 13 + f'О пользователе' + '-' * 13 +
                                  f'\n{about}\n' +
                                  f'-' * 40 +
                                  f'\nЗарегистрирован: {reg_date}\n\n', 'UTF-8'))
                except Exception as ex:
                    print(f'CONNECTION FAILED \n {ex}')

            elif (str(msg[1]) == 'Профиль') and (''.join(msg).find('@id') == -1):
                try:
                    connection = pymysql.connect(host=config.host,
                                                 port=config.dbport,
                                                 user=config.user,
                                                 password=config.password,
                                                 database=config.db,
                                                 cursorclass=pymysql.cursors.DictCursor
                                                 )
                    try:
                        with connection.cursor() as cursor:
                            cursor.execute(f"SELECT * FROM `users` WHERE ip='{clientAddress[0]}'")
                            result = cursor.fetchall()
                            logo = result[0]["logo_id"] #
                            id = result[0]["id"] #
                            name = result[0]["first_name"] #
                            surname = result[0]["last_name"] #
                            rating = result[0]["raiting"]
                            about = result[0]["about_user"]
                            reg_date = result[0]["reg_date"]

                    finally:

                        connection.close()
                        time.sleep(0.5)
                        self.csocket.send(
                            bytes('\n\n' + '-' * 40 + f'\nПрофиль {rating}★ | {name} {surname} @id{id}' +
                                  f'\nЛоготип(фронт привет) {logo}\n' +
                                  '-' * 13 + f'О пользователе' + '-' * 13 +
                                  f'\n{about}\n' +
                                  f'-' * 40 +
                                  f'\nЗарегистрирован: {reg_date}\n\n', 'UTF-8'))
                        self.csocket.send(
                            bytes(f'\n\nЧтобы узнать информацию о пользователе используйте: 1. Профиль @id1234',
                                  'UTF-8'))
                except Exception as ex:
                    print(f'CONNECTION FAILED \n {ex}')


            elif (str(msg[0]) == '2. Чатик') :
                folder_id = f'id{ClientThread.uuid}'
                folder = f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}'
                if not os.path.exists(folder):

                    self.csocket.send(bytes('Вы еще не начинали нискем диалог, создаю новую папку', 'UTF-8'))
                    print(f'Creating new folder with dialogs to user {folder_id}')
                    try:
                        os.mkdir(f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}')
                        print(f'Successfully created folder: {folder_id}')
                        time.sleep(1)
                        self.csocket.send(bytes('Успех', 'UTF-8'))
                        self.csocket.send(bytes('Кому напишем? использование: ', 'UTF-8'))
                    except Exception as ex:
                        print(f'FAIL:\n {ex}')
                    print('flag')
                    #ClientThread.chatting(self)
                else:
                    self.csocket.send(bytes('Кому напишем? использование @id... Например: @id1234 чтоб перейти к диалогу с пользователем: ', 'UTF-8'))


            else:
                self.csocket.send(bytes('Не понимаю команду', 'UTF-8'))





while True:
    server.listen(1)
    clientsock, clientAddress = server.accept()
    newthread = ClientThread(clientAddress, clientsock)
    print(f'Newthread {newthread} started')
    newthread.start()

