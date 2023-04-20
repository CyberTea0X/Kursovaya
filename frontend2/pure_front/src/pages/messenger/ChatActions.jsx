import React from 'react';
import './styles/ChatActions.css';

const ChatActions = () => {
  const handleAddUser = () => {
    // обработчик добавления пользователя в чат
  };

  const handleLeaveChat = () => {
    // обработчик выхода из чата
  };

  return (
    <div className="chat-actions">
      <button className="add-user" onClick={handleAddUser}>Add User</button>
      <button className="leave-chat" onClick={handleLeaveChat}>Leave Chat</button>
    </div>
  );
};

export { ChatActions };