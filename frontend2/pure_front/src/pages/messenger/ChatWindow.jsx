import React, { useState } from 'react';
import { ChatHeader } from './ChatHeader';
import { ChatMessages } from './ChatMessages';
import { ChatInput } from './ChatInput';
import './styles/ChatWindow.css';

const ChatWindow = () => {
  const [messages, setMessages] = useState([
    { id: 1, text: 'Hello!', sender: 'user', timestamp: '2021-10-01T12:00:00Z' },
    { id: 2, text: 'Hi there!', sender: 'bot', timestamp: '2021-10-01T12:01:00Z' },
    { id: 3, text: 'How are you?', sender: 'user', timestamp: '2021-10-01T12:02:00Z' },
  ]);

  const handleSendMessage = (message) => {
    // обработчик отправки сообщения
  };

  return (
    <div className="chat-window">
      <ChatHeader />
      <ChatMessages messages={messages} />
      <ChatInput onSendMessage={handleSendMessage} />
    </div>
  );
};

export { ChatWindow };