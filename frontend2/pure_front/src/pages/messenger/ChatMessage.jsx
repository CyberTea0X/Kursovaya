import React from 'react';
import './styles/ChatMessage.css';

const ChatMessage = ({ message }) => {
  return (
    <div className="chat-message">
      <span className="sender">{message.sender}: </span>
      <span className="text">{message.text}</span>
      <span className="timestamp">{message.timestamp}</span>
    </div>
  );
};

export { ChatMessage };