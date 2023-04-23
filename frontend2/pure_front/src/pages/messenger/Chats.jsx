import React from 'react';
import "./styles/chats.css";
import { Avatar } from '../../sharedComponents/Avatar';
import { useNavigate } from 'react-router-dom';


const Chats = ({chats}) => {

  let navigate = useNavigate(); 
  const goToChat = (user1id, user2id) =>{ 
    let path = `/Chat/${user1id}/${user2id}`; 
    navigate(path);
  }

  const handleChatClick = (chat) => {
    goToChat(chat.userid1, chat.userid2)
  }

    return (
      <div className='chat-items-container'>
        {chats.map((chat) => (
          <div className='chat-item-container' key={chat.id}>
          <Avatar user_id={chat.userid2} onClick={() => handleChatClick(chat)} />
          <div className='chat-item'> {chat.user2.username} </div>
          </div>
        ))}
      </div>
      );
};

export { Chats };