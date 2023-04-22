import React, { useState, useEffect } from 'react';
import { Searchbar } from './Seachbar';
import { Chat } from '../../types';
import { getUserChats } from '../../server/requests_handler';
import Cookies from 'js-cookie';
import { Chats } from './Chats';

const chats = [
    new Chat(1, 1, 2, '2022-01-01'),
    new Chat(2, 1, 3, '2022-01-02'),
    new Chat(3, 2, 3, '2022-01-03'),
];

const Messenger = () => {
    const [chats, setChats] = useState([]);
    const [email, password] = [Cookies.get("email").toLowerCase(), Cookies.get("password")]

    useEffect(() => {
        getUserChats(email, password).then((chats) => setChats(chats));
    }, [])

    const handleSearch = (event) => {
        console.log(event.target.value);
    }
    
    return (
        <div>
            <Searchbar onSearch={handleSearch} />
            <Chats chats={chats}/>
        </div>
    )

}

export { Messenger };