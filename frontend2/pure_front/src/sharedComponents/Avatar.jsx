import React, { useState, useEffect } from 'react';
import "./styles/avatar.css";
import { getAvatarImage } from '../server/requests_handler';
import UnknownPerson from "../img/Unknown_person.jpg";

const Avatar = ({user_id, onClick}) => {
    const [avatar, setAvatar] = useState();

    useEffect(() => {
        console.log(user_id)
        if (user_id === undefined) {
            return;
        }
        getAvatarImage(user_id).then(avatar => setAvatar(avatar));
    }, [user_id]);

    return (
      <div onClick={onClick} className='avatar-container'>
        <img className='avatar-image' src={(avatar && avatar.url) || UnknownPerson}></img>
      </div>
      );
};

export { Avatar };