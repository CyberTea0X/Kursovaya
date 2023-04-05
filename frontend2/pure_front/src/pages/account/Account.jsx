import React from "react";
import "./userprofile.css"
import User from "./Unknown_person.jpg"

const Account = () => {
    return (
        <div className="account">
            
                <div className="container">
                    
                    <div className="account-content">
                        
                        <h2 className="account-title">Фото профиля</h2>
                        <div className="account-user">
                            <img className="account-img" src={User} alt="" />
                            <button className="account-btn">Выбрать фотографию</button>
                        </div>
                        <h3 className="account-title" style={{fontWeight: '700', padding:'0 0 20px 0'}}>Личная информация</h3>
                        <p className="account-title">Имя пользователя</p>
                        <input className="account-input" placeholder="Введите имя" type="text" />
                        <p className="account-title">Обо мне</p>
                        <textarea style={{resize:'none'}} className="account-input2" placeholder="Напишите что-нибудь о себе" type="text" />
                        <p className="account-title">Введите номер телефона</p>
                        <input className="account-input" placeholder="Введите номер телефона" type="tel" />
                        <button className="account-btn2">Сохранить</button>
                        <h3 className="account-title" style={{fontWeight: '700', padding:'0 0 20px 0'}}>Изменение пароля</h3>
                        <p className="account-title">Текущий пароль</p>
                        <input className="account-input" placeholder="Текущий пароль" type="password"  />
                        <p className="account-title">Новый пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password"  />
                        <p className="account-title">Подтвердите пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password"  />
                        <button className="account-btn2">Сохранить</button>
                    </div>
                
            </div>
        </div>
    )
}




export {Account};