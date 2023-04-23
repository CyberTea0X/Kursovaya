import React, { useState, useEffect, useCallback } from 'react';
import './profile.css';
import { FaCog } from 'react-icons/fa';
import UnknownPerson from "../../img/Unknown_person.jpg";
import { User } from '../../types';
import { useNavigate } from 'react-router-dom';
import Cookies from 'js-cookie';
import { getUserProfile, getTagsArray, getAvatarImage} from '../../server/requests_handler';
import { create_chat } from '../../server/requests';
import { visit } from '../../server/requests';
import { useParams } from 'react-router-dom';
import { Gallery } from './Gallery';


const Profile = () => {
  const [email, pw] = [Cookies.get("email")?.toLowerCase(), Cookies.get("password")];
  const [user, setUser] = useState(User.emptyUser());
  const [avatar, setAvatar] = useState(UnknownPerson);
  const [isOwner, setIsOwner] = useState(false);
  const [tags, setTags] = useState([]); // хранение данных о тегах
  const { userId } = useParams();

  let navigate = useNavigate(); 
  const routeChange = useCallback((route) =>{ 
      let path = `/${route}`; 
      navigate(path);
  }, [navigate])

  const getAccount = useCallback( async () => {
    let userIdOrEmail;
      try {
        userIdOrEmail = (userId === "me") ? Cookies.get("email").toLowerCase(): userId;
          let user_ = await getUserProfile(userIdOrEmail);
          setUser(user_);
      }
      catch (error) {
          if (error instanceof TypeError) {
              routeChange("Login");
              return;
          }
          alert(error.message);
      }
  }, [routeChange, userId]);

  const handleMessageUser = () => {
    let my_id = Cookies.get("id");
    create_chat(email, pw, user.id).then(routeChange(`./Chat/${my_id}/${user.id}`))
  }

  const retrieveUserTags = useCallback( async () => {
    try {
        let tags_arr = await getTagsArray(user.id);
        if (tags_arr.length > 0) {
          setTags(tags_arr);
        }
    }
    catch (error) {
        alert(error.message);
    }
  }, [user.id])

  const loadAvatar = useCallback(async () => {
    let logo_img = await getAvatarImage(user.id);
    if (logo_img !== undefined) {
      setAvatar(logo_img.url);
    }
  }, [user.id]);

  useEffect(() => {
    getAccount();
  }, [getAccount]);

  useEffect(() => {
    if (user.id === undefined) {
      return;
    }
    retrieveUserTags()
    if (user.id === parseInt(Cookies.get("id"))) {
      setIsOwner(true)
    }
    else {
      visit(email, pw, user.id);
    }
    loadAvatar()
  }, [user, email, loadAvatar, pw, retrieveUserTags]);

  // render the profile UI
  return (
    <>
      <div className="user-info">
        <div className="main-info">
          <div className="user-name"> {user.username} </div>
          <img className="profile-img" src={avatar || UnknownPerson} alt="" />
          <div className="rating-container">
            <b> Рейтинг: </b>
            <div> {user.rating} </div>
          </div>
          <button className='message-user' onClick={handleMessageUser}>
            Сообщение
          </button>
        </div>
        <div className="additional-info">
          <div className='user-fullname'>
            <p> {user.firstname}</p>
            <p> {user.lastname}</p>
          </div>
            <p className='user-about'>
              {user.about}
            </p>
          <div className='user-tags'>
            {tags.map((tag, i) => (
              <span key={i}> {tag} </span>
            ))}
          </div>
        </div>
        <div className='profile-cog'>
          <FaCog style = {{cursor: 'pointer'}} onClick={()=>routeChange("Settings")}></FaCog>
        </div>
      </div>

      {/* display the image grid */}
      <Gallery user={user} isOwner={isOwner}/>
    </>
  );
};

export { Profile };