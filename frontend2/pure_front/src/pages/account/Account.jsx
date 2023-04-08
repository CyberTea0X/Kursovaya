import React, { useState, useEffect } from "react";
import "./userprofile.css"
import User from "./Unknown_person.jpg"
import { userProfile, editUser } from "../../server/server";
import { User as UserProfile} from "../../types";
import Cookies from "js-cookie";

const Account = () => {
    const [user, setUser] = useState(UserProfile.emptyUser()); // хранение данных об аккаунте
    const getAccount = async () => {
        try {
            let email = Cookies.get("email").toLowerCase();
            await userProfile(email)
            .then(data => {
            if (data["status"] != "OK") {
                throw Error(data["reason"]);
            }
            let user_ = UserProfile.fromJson(data["user"])
            user_.email = email;
            setUser(user_);
            })
        }
        catch (error) {
            alert(error.message);
        }
    }

    const editUserProfile = async () => {
        alert("Y")
        try {
            let email = Cookies.get("email").toLowerCase();
            let password = Cookies.get("password")
            await editUser(email, password, user)
            .then(data => {
                if (data["status"] != "OK") {
                    throw Error(data["reason"]);
                }
                else {
                    throw Error(data["reason"]);
                }
                })
        }
        catch (error) {
            alert(error.message);
        }
    }

    const handleSubmit = async (e) => {
        e.preventDefault();
        editUserProfile()
    }
    useEffect(() => {
        getAccount();
    }, []); // вызываем getAccount() только один раз при загрузке компонента
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
                        <input className="account-input" placeholder="Введите имя" type="text" value={user.username ? user.username : ""} onChange={(e) => setUser({...user, username: e.target.value})} />
                        <p className="account-title">Почта</p>
                        <input className="account-input" placeholder="Введите email" type="email" value={user.email ? user.email : ""} onChange={(e) => setUser({...user, email: e.target.value})} />
                        <p className="account-title">Имя</p>
                        <input className="account-input" placeholder="Введите имя" type="text" value={user.firstname ? user.firstname : ""} onChange={(e) => setUser({...user, firstname: e.target.value})} />
                        <p className="account-title">Фамилия</p>
                        <input className="account-input" placeholder="Введите фамилию" type="text" value={user.lastname ? user.lastname : ""} onChange={(e) => setUser({...user, lastname: e.target.value})} />
                        <p className="account-title">Возраст</p>
                        <input className="account-input" placeholder="Введите возраст" type="number" value={user.age ? user.age : ""} onChange={(e) => setUser({...user, age: e.target.value})} />
                        <p className="account-title">Пол</p>
                        <select className="account-input" value={user.gender ? user.gender : ""} onChange={(e) => setUser({...user, gender: e.target.value})}>
                        <option value="">Выберите пол</option>
                        <option value="male">Мужской</option>
                        <option value="female">Женский</option>
                        <option value="other">Другой</option>
                        </select>
                        <p className="account-title">Если Вы художник, пожалуйста, выберите теги из предложенных, под которыми вы рисуете:<br/>
                        <p className="Tags">
                        #Traditional<br/>
                        #Digital<br/>
                        #Portraits<br/>
                        #Animalistic<br/>
                        #Anime<br/>
                        #Nature<br/>
                        #Landscape<br/>
                        </p>
                        </p>
                        <textarea style={{resize:'none'}} className="account-input2" placeholder="Напишите что-нибудь о себе" type="text" value={user.about ? user.about : ""} />
                        <p className="account-title">Введите номер телефона</p>
                        <input className="account-input" placeholder="Введите номер телефона" type="tel" />
                        <h3 className="account-title" style={{fontWeight: '700', padding:'0 0 20px 0'}}>Изменение пароля</h3>
                        <p className="account-title">Текущий пароль</p>
                        <input className="account-input" placeholder="Текущий пароль" type="password"  />
                        <p className="account-title">Новый пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password"  />
                        <p className="account-title">Подтвердите пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password"  />
                        <form onSubmit={handleSubmit}>
                        <button className="account-btn2" type="submit">Сохранить</button>
                        </form>
                    </div>
                
            </div>
        </div>
    )
}




export {Account};