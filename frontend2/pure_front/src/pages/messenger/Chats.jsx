import React, { useEffect } from 'react';
import "./styles/chats.css";
import { Avatar } from '../../sharedComponents/Avatar';
import { useNavigate } from 'react-router-dom';
import { ChatButton } from './ChatButton';


const Chats = ({chats, current_user}) => {

    return (
      <div className='chat-items-container'>
        {chats.map((chat) => (
          <div className='chat-item-container' key={chat.id}>
            <ChatButton chat={chat} current_user={current_user}></ChatButton>
          </div>
        ))}
      </div>
      );
};

export { Chats };