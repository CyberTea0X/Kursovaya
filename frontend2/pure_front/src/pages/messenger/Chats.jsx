import React, { useState, useEffect } from 'react';
import "./styles/chats.css";

const Chats = ({chats}) => {

    return (
      <div className='chat-item-container'>
        {chats.map((chat) => (<div className='chat-item' key={chat.id}> {chat.user2.username} </div>))}
      </div>
      );
};

export { Chats };