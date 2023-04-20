import React from 'react';
import './styles/ChatInfo.css';

const ChatInfo = () => {
  return (
    <div className="chat-info">
      <img src="https://via.placeholder.com/50" alt="Chat Avatar" />
      <span>Chat Name</span>
    </div>
  );
};

export { ChatInfo };