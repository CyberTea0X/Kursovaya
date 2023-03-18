import socket
from threading import Thread
import threading
import os.path
import hashlib


SERVER = '195.93.160.52'
PORT = 25567




# client.sendall(bytes(input('Ведите сообщение') , 'UTF-8'))


def authorisation():
    if os.path.exists('messenger_cash.dsrv'):
        with open("messenger_cash.dsrv", "r") as messanger_cash:
            a = []
            for line in messanger_cash.readlines():
                a.append(str(line))
            login, password = a[0], a[1]
            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            client.connect((SERVER, PORT))
            client.sendall(bytes(f'@auth℻{login}℻{password}', 'UTF-8'))
            print('У вас уже есть аккаунт, происходит автоматическая авторизация')
            return True

    else:
        file = open(f"messenger_cash.dsrv", "w+")

        print('У вас еще нет аккаунта, начинаю процесс регистрации')
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((SERVER, PORT))
        email = input('Введите email')#1
        password = input('Введите пароль')#2
        password_confirm = input('Подтвердите пароль')
        if password_confirm != password:
            pass
        name = input('Введите имя')#3
        surname = input('Введите фамилию')#4
        about = input('Расскажите о себе')#5
        logo = input('прикрепите лого P.S. ЭТО НАЧАЛЬНАЯ ВЕРСИЯ, ПРОСТО ПРИКРЕПЛЯЕМ ССЫЛКУ НА КАРТИНКУ ИЗ ГУГЛА')#6
        password = hashlib.md5(password.encode('utf-8')).hexdigest()
        print(password)
        file.write(f'{email}\n{password}')
        client.sendall(bytes(f'@registration℻{name}℻{surname}℻{password}℻{email}℻{logo}℻{about}', 'UTF-8'))
        file.close()
        authorisation()
    while True:
        in_data = client.recv(4096)
        print('Ответ от сервера:', in_data.decode())


def work():
    if authorisation():
        pass


'''client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect((SERVER, PORT))
def task():

    while True:

        in_data = client.recv(4096)
        print('Ответ от сервера:', in_data.decode())'''


def experiment():
    authorisation()




t1 = Thread(target=experiment, daemon=True)
#t2 = Thread(target=task)

t1.start()
#t2.start()

t1.join()
#t2.join()
