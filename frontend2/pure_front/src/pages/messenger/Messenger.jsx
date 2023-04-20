import React, { useState } from 'react';
import { Link, Route, Routes } from 'react-router-dom';
import { ChatHeader } from './Header';
import { ChatList } from './ChatList';
import { ChatWindow } from './ChatWindow';
import './styles/Messenger.css';

const Messenger = () => {
  const [currentChat, setCurrentChat] = useState(null);

  const handleChatClick = (chat) => {
    setCurrentChat(chat);
  };

  return (
    <div className="messenger">
      <ChatHeader />
      <div className="messenger-content">
        <ChatList onChatClick={handleChatClick} />
        <div className="chat-window">
          <Routes>
            <Route path="/" element={<h2>Welcome to Messenger!</h2>} />
            <Route path="/chat/:chatId" element={<ChatWindow chat={currentChat} />} />
          </Routes>
        </div>
      </div>
      <div className="messenger-footer">
        <input type="text" placeholder="Type a message..." />
        <button>Send</button>
      </div>
    </div>
  );
};

export { Messenger };