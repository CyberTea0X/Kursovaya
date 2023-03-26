import socket
import time
from threading import Thread
import os.path
import hashlib
import validators
from email_validate import validate

# SERVER = '195.93.160.52'
SERVER = '192.168.31.46'
PORT = 25567

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect((SERVER, PORT))


def authorisation():
    if not os.path.exists('auth'):
        print('У вас еще нет аккаунта, начинаю процесс регистрации')
        '''client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((SERVER, PORT))'''
        email = input('Введите email: ')  # 1
        email_valid = validate(
            email_address=email,
            check_format=True,
            check_blacklist=True,
            check_dns=True,
            dns_timeout=10,
            check_smtp=False,
            smtp_debug=False)
        while not email_valid:
            email = input('Не валидный email. Повторите ввод: ')  # 1
        password = input('Введите пароль длинной не менее 4 символов: ')  # 2
        while len(password) < 4:
            password = input('Слишком короткий пароль, повторите ввод: ')  # 2
        password_confirm = input('Подтвердите пароль: ')
        while not password_confirm == password:
            print('Пароли не совпадают')
            password = input('Введите пароль: ')  # 2
            password_confirm = input('Подтвердите пароль: ')
        name = input('Введите имя: ')  # 3
        surname = input('Введите фамилию: ')  # 4
        about = input('Расскажите о себе: ')  # 5
        logo = input('прикрепите лого P.S. ЭТО НАЧАЛЬНАЯ ВЕРСИЯ, ПРОСТО ПРИКРЕПЛЯЕМ ССЫЛКУ НА КАРТИНКУ ИЗ ГУГЛА: ')  # 6
        if validators.url(logo) is not True:
            while validators.url(logo) is not True:
                logo = input('Не правильная ссылка, введите новую: ')
        password = hashlib.md5(password.encode('utf-8')).hexdigest()
        os.mkdir("auth")
        os.chdir("auth")
        file = open(f"messenger_cash.dsrv", "w+", encoding='UTF-8')
        text = f'{email}\n{password}'
        file.write(text)
        file.close()
        client.sendall(bytes(f'@registration℻{name}℻{surname}℻{password}℻{email}℻{logo}℻{about}', 'UTF-8'))
        time.sleep(1)
        authorisation()
        # https://amiel.club/uploads/posts/2022-03/1647567047_1-amiel-club-p-grustnii-angel-kartinki-1.jpg

    else:
        os.chdir("auth")
        with open("messenger_cash.dsrv", "r") as messanger_cash:
            a = []

            for line in messanger_cash.readlines():
                a.append(str(line).strip())
            email, password = a[0], a[1]
            print('У вас уже есть аккаунт, происходит автоматическая авторизация')
            client.sendall(bytes(f'@auth℻{email}℻{password}', 'UTF-8'))

    while True:
        in_data = client.recv(4096)
        print(in_data.decode())
        '''TO_DO: СДЕЛАТЬ ПО ЧЕЛОВЕЧЕСКИ'''
        s = ''  # КОСТЫЛЬ ИБО НЕ РАБОТАЕТ ПО ДРУГОМУ
        s = s.join(in_data.decode())  # КОСТЫЛЬ
        s = s.split('\n')  # КОСТЫЛЬ
        if 'ACCESS GRANTED' in s:  # КОСТЫЛЬ
            while True:
                inp_data = client.recv(4096)
                msg = inp_data.decode()
                print(msg)
                client.sendall(bytes(input('inp = '), 'UTF-8'))

            # break
        elif 'ACCESS DENIED' in s:
            while 'ACCESS GRANTED' not in s:
                print('Упс... Что-то пошло не так:( \nВведите данные заново:')
                email = input('Введите email: ')  # 1
                password = input('Введите пароль: ')  # 2
                password = hashlib.md5(password.encode('utf-8')).hexdigest()
                file = open(f"messenger_cash.dsrv", "w+", encoding='UTF-8')
                text = f'{email}\n{password}'
                file.write(text)
                file.close()
                client.sendall(bytes(f'@reauth℻{email}℻{password}', 'UTF-8'))


def listener():
    while True:
        inp_data = client.recv(4096)
        msg = inp_data.decode()
        print(msg)


t1 = Thread(target=authorisation)
#t2 = Thread(target=listener)
t1.start()
#t2.start()

t1.join()
#t2.join()

