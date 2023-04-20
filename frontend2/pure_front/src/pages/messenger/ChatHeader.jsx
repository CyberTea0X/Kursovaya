import React from 'react';
import './styles/ChatHeader.css';

const ChatHeader = ({ chat }) => {
  const handleAddUser = () => {
    // обработчик добавления пользователя в чат
  };

  return (
    <div className="chat-header">
      <div className="chat-info">
        <h2>{chat.name}</h2>
        <span>{chat.members.length} members</span>
      </div>
      <button className="add-user" onClick={handleAddUser}>Add User</button>
    </div>
  );
};

export { ChatHeader };