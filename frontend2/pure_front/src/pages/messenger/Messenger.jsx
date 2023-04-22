import React, { useState, useEffect } from 'react';
import { Searchbar } from './Seachbar';
import { Chat } from '../../types';

const chats = [
    new Chat(1, 1, 2, '2022-01-01'),
    new Chat(2, 1, 3, '2022-01-02'),
    new Chat(3, 2, 3, '2022-01-03'),
];

const Messenger = () => {
    return (
    <Searchbar chats={chats}></Searchbar>
    )

}

export { Messenger };