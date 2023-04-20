import React, { useState } from 'react';
import './styles/ChatInput.css';

const ChatInput = ({ onSendMessage }) => {
  const [text, setText] = useState('');

  const handleChange = (event) => {
    setText(event.target.value);
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    if (text.trim() !== '') {
      onSendMessage(text);
      setText('');
    }
  };

  return (
    <form className="chat-input" onSubmit={handleSubmit}>
      <input type="text" value={text} onChange={handleChange} />
      <button type="submit">Send</button>
    </form>
  );
};

export { ChatInput };