import React from 'react';
import './styles/ChatListItem.css';

const ChatListItem = ({ chat }) => {
  const handleClick = () => {
    // обработчик клика на элементе списка чатов
  };

  return (
    <div className="chat-list-item" onClick={handleClick}>
      <img src={chat.image} alt={chat.name} />
      <span>{chat.name}</span>
    </div>
  );
};

export { ChatListItem };