import React, { useState, useEffect } from 'react';
import "./styles/Searchbar.css";

const Chats = ({chats}) => {

    return (
      <div>
        {chats.map((chat) => (<div> chat.user2.username </div>))}
      </div>
      );
};

export { Chats };