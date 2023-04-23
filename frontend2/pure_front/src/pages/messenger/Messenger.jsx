import React, { useState, useEffect } from 'react';
import { Searchbar } from './Searchbar';
import { getUserChats } from '../../server/requests_handler';
import Cookies from 'js-cookie';
import { Chats } from './Chats';
import { Header } from './Header';
import Fuse from 'fuse.js';

const Messenger = () => {
    const [chats, setChats] = useState([]);
    const [email, password] = [Cookies.get("email").toLowerCase(), Cookies.get("password")]

    useEffect(() => {
        getUserChats(email, password).then((chats) => setChats(chats));
    }, [email, password])

    const handleSearch = (event) => {
        if (chats === undefined) {
            return;
        }
        let query = event.target.value;
        const fuse = new Fuse(chats, {
            keys: ['user2.username'],
            includeScore: true,
        })
        const result = fuse.search(query);
        const sorted_chats = result.map(r => r.item);
        const other_chats = chats.filter(chat => !sorted_chats.includes(chat));
        setChats(sorted_chats.concat(other_chats));
    }
    
    return (
        <div>
            <Header> Мессенджер </Header>
            <Searchbar onSearch={handleSearch} />
            <Chats chats={chats}/>
        </div>
    )

}

export { Messenger };