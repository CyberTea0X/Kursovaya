import React from 'react';
import { ChatInfo } from './ChatInfo';
import { ChatActions } from './ChatActions';
import './styles/ChatHeader.css';

const ChatHeader = () => {
  return (
    <div className="chat-header">
      <ChatInfo />
      <ChatActions />
    </div>
  );
};

export { ChatHeader };