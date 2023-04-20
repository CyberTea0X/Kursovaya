import React from 'react';
import { ChatMessage } from './ChatMessage';
import './styles/ChatMessages.css';

const ChatMessages = ({ messages }) => {
  return (
    <div className="chat-messages">
      {messages.map((message) => (
        <ChatMessage key={message.id} message={message} />
      ))}
    </div>
  );
};

export { ChatMessages };