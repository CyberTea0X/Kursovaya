import datetime
import pymysql
import random
import socket
import string
import threading
import os
import config
import requests



def listener():
    pass


def threaded(conn):
    uuid = ''
    status = "AUTHORIZED"
    while True:
        data = conn.recv(4096)
        response = data.decode()
        print(response)
        response = str(response).split('℻')
        '''if response[0] == '':
            print('diconnection')
            break'''

        if response[0] == '@auth':
            conn.send(bytes('Происходит авторизация', 'UTF-8'))
            email = response[1]
            password = response[2]
            uuid = response[3]
            print(f'Авторизация клиента id{uuid}')
            print(f'логин {email}\npass {password}')
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
                        cursor.execute(f"SELECT * FROM `users` WHERE id={uuid}")
                        result = cursor.fetchall()
                        mail = result[0]["email"]
                        pas = result[0]["password"]
                        if (mail == email) and (pas == password):
                            conn.send(bytes(f'ACCESS GRANTED\n', 'UTF-8'))
                            cursor.execute(f"UPDATE `users` SET authorized = 'AUTHORIZED' WHERE id={uuid}")
                            connection.commit()
                            cursor.execute(f"SELECT * FROM `users` WHERE id={uuid}")
                            result = cursor.fetchall()
                            status = result[0]["authorized"]
                            conn.send(
                                bytes(
                                    "-" * 40 + '\nКуда отправимся)? \n1. Профиль \n2. Чатик \n3. Список всех '
                                               'участников\n' + "-" * 40,
                                    'UTF-8'))
                            print(f'ACCESS GRANTED FOR USER id{uuid}')

                            # break
                        else:
                            print(f'ACCESS DENIED FOR USER id{uuid}')
                    break
                finally:
                    # time.sleep(0.5)
                    connection.close()
                    print(f'USER id{uuid} is {status}')

            except Exception as ex:
                print(f'CONNECTION FAILED \n {ex}')

        elif response[0] == '@registration':

            conn.send(bytes('Происходит регистрация', 'UTF-8'))
            name = str(response[1])
            surname = str(response[2])
            password = str(response[3])
            email = str(response[4])
            logo = str(response[5])
            about = str(response[6])

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

                try:
                    with connection.cursor() as cursor:
                        # id_now = cursor.execute("SELECT `id` FROM `users WHERE MAX(id) FROM users`")
                        insert_query = f"INSERT INTO `users`(id, first_name, last_name, email, logo_id, " \
                                       "raiting, about_user, chats_folder, authorized, password, reg_date, " \
                                       f"is_online) VALUES ('','{name}','{surname}','{email}'," \
                                       f"'{random_name}.jpg','0','{about}','CHAT_FOLDER_TO_DO','?', " \
                                       f"'{password}','{red_data}','1');"
                        cursor.execute(insert_query)
                        connection.commit()
                        usr_id = cursor.execute("SELECT `id` FROM `users`")
                        conn.send(bytes(f'{usr_id}', 'UTF-8'))
                        print(usr_id)
                finally:
                    threaded(conn)
                    connection.close()

            except Exception as ex:
                print(f'CONNECTION FAILED \n {ex}')

        elif str(response[0]) == '1. Профиль':
            conn.send(bytes('TO_DO', 'UTF-8'))

        elif str(response[0]) == '2. Чатик':

            folder_id = f'id{uuid}'
            folder = f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}'
            if not os.path.exists(folder):
                os.mkdir(f'C:/Users/fedor/PycharmProjects/pythonProject14/user_chats/{folder_id}')
            conn.send(bytes('TO_DO', 'UTF-8'))


def Main():
    host = "192.168.31.46"

    # reserve a port on your computer
    # in our case it is 12345 but it
    # can be anything
    port = config.con_port
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    print("socket binded to port", port)

    # put the socket into listening mode
    server.listen(5)
    print("socket is listening")

    # a forever loop until client wants to exit
    while True:
        # establish connection with client
        conn, addr = server.accept()


        # lock acquired by client
        print('Connected to :', addr[0], ':', addr[1])

        # Start a new thread and return its identifier
        t1 = threading.Thread(target=threaded(conn))
        t1.start()
        t1.join()
        # start_new_thread(threaded, (conn,))
    # server.close()


if __name__ == '__main__':
    Main()
