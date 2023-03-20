import socket
from threading import Thread
import threading
import os.path
import hashlib

SERVER = '195.93.160.52'
PORT = 25567


def work():
    print(111)
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((SERVER, PORT))
    #client.sendall(bytes('@BREAKCON', 'UTF-8'))

    '''TO_DO: СДЕЛАТЬ ПО ЧЕЛОВЕЧЕСКИ'''
    while True:
        client.sendall(bytes(input('inp = '), 'UTF-8'))
        inp_data = client.recv(4096)
        msg = inp_data.decode()
        print(msg)


def authorisation():
    if not os.path.exists('messenger_cash.dsrv'):
        file = open(f"messenger_cash.dsrv", "w+", encoding='UTF-8')

        print('У вас еще нет аккаунта, начинаю процесс регистрации')
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((SERVER, PORT))
        email = input('Введите email')  # 1
        password = input('Введите пароль')  # 2
        password_confirm = input('Подтвердите пароль')
        if password_confirm != password:
            pass
        name = input('Введите имя')  # 3
        surname = input('Введите фамилию')  # 4
        about = input('Расскажите о себе')  # 5
        logo = input('прикрепите лого P.S. ЭТО НАЧАЛЬНАЯ ВЕРСИЯ, ПРОСТО ПРИКРЕПЛЯЕМ ССЫЛКУ НА КАРТИНКУ ИЗ ГУГЛА')  # 6
        password = hashlib.md5(password.encode('utf-8')).hexdigest()
        print(password)

        client.sendall(bytes(f'@registration℻{name}℻{surname}℻{password}℻{email}℻{logo}℻{about}', 'UTF-8'))

        # https://amiel.club/uploads/posts/2022-03/1647567047_1-amiel-club-p-grustnii-angel-kartinki-1.jpg
        while True:
            inp_data = client.recv(4096)
            usr_id = inp_data.decode()
            print(usr_id)
            if (usr_id != '') and (usr_id.isdecimal()):
                text = f'{email}\n{password}\n{usr_id}'
                file.write(text)
                file.close()
                break
            else:
                print(usr_id)
        authorisation()


    else:
        with open("messenger_cash.dsrv", "r") as messanger_cash:
            a = []
            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            client.connect((SERVER, PORT))
            for line in messanger_cash.readlines():
                a.append(str(line).strip())
            print(a)
            uuid, email, password = a[2], a[0], a[1]
            print(a[2], a[0], a[1])

            client.sendall(bytes(f'@auth℻{email}℻{password}℻{uuid}', 'UTF-8'))

            print('У вас уже есть аккаунт, происходит автоматическая авторизация')
    while True:
        in_data = client.recv(4096)
        print(in_data.decode())
        s = ''  # КОСТЫЛЬ ИБО НЕ РАБОТАЕТ ПО ДРУГОМУ
        s = s.join(in_data.decode())  # КОСТЫЛЬ
        s = s.split('\n')  # КОСТЫЛЬ
        if 'ACCESS GRANTED' in s:  # КОСТЫЛЬ
            work()
            #break
        else:
            pass



t1 = Thread(target=authorisation)
t1.start()
t1.join()

while True:
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((SERVER, PORT))
    client.sendall(bytes(input('inp = '), 'UTF-8'))
    inp_data = client.recv(4096)
    msg = inp_data.decode()
    print(msg)
# t2.start()


# t2.join()