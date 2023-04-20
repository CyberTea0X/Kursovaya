import React, { useState } from 'react';
import { ChatListItem } from './ChatListItem'
import './styles/ChatList.css';

const ChatList = () => {
  const [chats, setChats] = useState([
    { id: 1, name: 'Chat 1', image: 'https://via.placeholder.com/50' },
    { id: 2, name: 'Chat 2', image: 'https://via.placeholder.com/50' },
    { id: 3, name: 'Chat 3', image: 'https://via.placeholder.com/50' },
  ]);
  const [search, setSearch] = useState('');

  const handleSearchChange = (event) => {
    setSearch(event.target.value);
  };

  const filteredChats = chats.filter((chat) =>
    chat.name.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="chat-list">
      <input type="text" value={search} onChange={handleSearchChange} />
      {filteredChats.map((chat) => (
        <ChatListItem key={chat.id} chat={chat} />
      ))}
    </div>
  );
};

export { ChatList };