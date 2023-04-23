import React, { useState, useEffect } from "react";
import "./settings.css"
import UnknownPerson from "../../img/Unknown_person.jpg";
import { editUser } from "../../server/requests";
import { User as UserProfile} from "../../types";
import { getUserProfile, getTagsArray, editTagsFromStr, getAvatarImage  } from "../../server/requests_handler";
import Cookies from "js-cookie";
import { useNavigate } from 'react-router-dom';
import { delete_profile } from "../../server/requests";


const Settings = () => {
    const [user, setUser] = useState(UserProfile.emptyUser()); // хранение данных об аккаунте
    const [tags, setTags] = useState(""); // хранение данных о тегах
    const [avatar, setAvatar] = useState(UnknownPerson);


    let navigate = useNavigate(); 
    const routeChange = (route) =>{ 
        let path = `/${route}`; 
        navigate(path);
    }
      
    const getAccount = async () => {
        try {
            let email = Cookies.get("email").toLowerCase();
            let user_ = await getUserProfile(email);
            user_.email = email;
            setUser(user_);
        }
        catch (error) {
            if (error instanceof TypeError) {
                routeChange("Login");
                return;
            }
            alert(error.message);
        }
    }

    const loadAvatar = async () => {
        let logo_img = await getAvatarImage(user.id);
        if (logo_img !== undefined) {
          setAvatar(logo_img.url);
        }
      }

    const retrieveUserTags = async () => {
        try {
            let id = Cookies.get("id");
            let tags_arr = await getTagsArray(id);
            setTags(tags_arr.join(', '));
        }
        catch (error) {
            alert(error.message);
        }
    }

    const handleDeleteProfile = async () => {
        let really_delete = prompt(
            "Вы собираетесь удалить свой профиль! Введите свой логин для подтверждения!"
            ) == user.username;
        if (!really_delete) {
            alert("Неверно введён логин, надеемся, вы передумали");
            return;
        }
        const email = Cookies.get("email").toLowerCase();
        const pw = Cookies.get("password");
        Cookies.remove("id", { path: '/' });
        Cookies.remove("email", { path: '/' });
        Cookies.remove("password", { path: '/' });
        let data = await delete_profile(email, pw);
        if (data["status"] != "OK") {
            alert(data["reason"])
            return;
        }
        routeChange('');
    }

    const editUserData = async () => {
        try {
            const user_clone = user.clone()
            let email = Cookies.get("email").toLowerCase();
            let password = Cookies.get("password")
            user_clone.password = (user.password ==="secret") ? password: user.password;
            user_clone.email = (user.email ==="secret") ? email: user.email;
            await editUser(email, password, user_clone)
            .then(data => {
                if (data["status"] !== "OK") {
                    throw Error(data["reason"]);
                }
                else {
                    if (user_clone.password !== "secret") {
                        Cookies.set("password", user_clone.password);
                    }
                    if (user_clone.email !== "secret") {
                        Cookies.set("email", user_clone.email);
                    }
                }
                })
            if (tags.length != 0) {
                await editTagsFromStr(email, password, tags);
            }
        }
        catch (error) {
            alert(error.message);
            return;
        }
        alert("Изменения успешно внесены");
    }
    const addTag = (tag) => {
        if (!tags.includes(tag)) {
          setTags(tags ? tags + ", " + tag : tag);
        }
      }

    const handleSubmit = async (e) => {
        e.preventDefault();
        editUserData()
    }
    useEffect(() => {
        getAccount();
    }, []);

    useEffect(() => {
        if (user.email === undefined) {
            return;
        }
        loadAvatar();
        retrieveUserTags();
    }, [user])
    return (
        <div className="account">
            
                <div className="container">
                    
                    <div className="account-content">
                        
                        <h2 className="account-title">Фото профиля</h2>
                        <div className="account-user">
                            <img className="account-img" src={avatar} alt="" />
                            <a href="/Profile/me" className="account-link">Выбрать фотографию</a>
                        </div>
                        <h3 className="account-title" style={{fontWeight: '700', padding:'0 0 20px 0'}}>Личная информация</h3>
                        <p className="account-title">Имя пользователя</p>
                        <input className="account-input" placeholder="Введите имя" type="text" value={user.username ? user.username : ""} onChange={(e) => setUser(user.clone({username: e.target.value}))} />
                        <p className="account-title">Почта</p>
                        <input className="account-input" placeholder="Введите email" type="email" value={user.email ? user.email : ""} onChange={(e) => setUser(user.clone({email: e.target.value}))} />
                        <p className="account-title">Имя</p>
                        <input className="account-input" placeholder="Введите имя" type="text" value={user.firstname ? user.firstname : ""} onChange={(e) => setUser(user.clone({firstname: e.target.value}))} />
                        <p className="account-title">Фамилия</p>
                        <input className="account-input" placeholder="Введите фамилию" type="text" value={user.lastname ? user.lastname : ""} onChange={(e) => setUser(user.clone({lastname: e.target.value}))} />
                        <p className="account-title">Возраст</p>
                        <input className="account-input" placeholder="Введите возраст" type="number" value={user.age ? user.age : ""} onChange={(e) => setUser(user.clone({age: e.target.value}))} />
                        <p className="account-title">Пол</p>
                        <select className="account-input" value={user.gender ? user.gender : ""} onChange={(e) => setUser(user.clone({gender: e.target.value}))}>
                        <option value="">Выберите пол</option>
                        <option value="male">Мужской</option>
                        <option value="female">Женский</option>
                        <option value="other">Другой</option>
                        </select>
                        <p className="account-title">Если Вы художник, пожалуйста, выберите теги из предложенных, под которыми вы рисуете:<br/>
                        <p className="Tags">
                        <div className="tags-container">
                        <div className="tag" onClick={() => addTag("#Traditional")}>#Traditional</div>
                        <div className="tag" onClick={() => addTag("#Digital")}>#Digital</div>
                        <div className="tag" onClick={() => addTag("#Portraits")}>#Portraits</div>
                        <div className="tag" onClick={() => addTag("#Animalistic")}>#Animalistic</div>
                        <div className="tag" onClick={() => addTag("#Anime")}>#Anime</div>
                        <div className="tag" onClick={() => addTag("#Nature")}>#Nature</div>
                        <div className="tag" onClick={() => addTag("#Landscape")}>#Landscape</div>
                        </div>
                        <textarea style={{resize:'none'}} className="account-input2" placeholder="Ваши теги" type="text" value={tags ? tags : ""} onChange={(e) => setTags(e.target.value)} />
                        </p>
                        </p>
                        <p className="account-title"> О себе</p>
                        <textarea style={{resize:'none'}} className="account-input2" placeholder="Напишите что-нибудь о себе" type="text" value={user.about ? user.about : ""} onChange={(e) => setUser(user.clone({about: e.target.value}))} />
                        <h3 className="account-title" style={{fontWeight: '700', padding:'0 0 20px 0'}}>Изменение пароля</h3>
                        <p className="account-title">Текущий пароль</p>
                        <input className="account-input" placeholder="Текущий пароль" type="password"  />
                        <p className="account-title">Новый пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password" onChange={(e) => setUser(user.clone({password: e.target.value}))}/>
                        <p className="account-title">Подтвердите пароль</p>
                        <input className="account-input" placeholder="Пароль" type="password"  />
                        <div style={{display: "flex", flexDirection: "row"}}>
                        <form onSubmit={handleSubmit} style={{backgroundColor: "rgba(255, 255, 255, 0)"}}>
                            <button className="account-btn2" type="submit">Сохранить</button>
                        </form>
                        <form onSubmit={handleDeleteProfile} style={{backgroundColor: "rgba(255, 255, 255, 0)"}}>
                            <button className="account-delete" type="submit">Удалить профиль</button>
                        </form>
                        </div>
                    </div>
                </div>
            </div>
    )
}




export {Settings};