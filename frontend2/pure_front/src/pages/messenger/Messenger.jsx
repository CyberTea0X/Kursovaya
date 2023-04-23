import React, { useState, useEffect, useCallback } from 'react';
import { Searchbar } from './Searchbar';
import { getUserChats } from '../../server/requests_handler';
import Cookies from 'js-cookie';
import { Chats } from './Chats';
import { Header } from './Header';
import Fuse from 'fuse.js';
import { useNavigate } from 'react-router-dom';

const Messenger = () => {
    const [chats, setChats] = useState([]);

    let navigate = useNavigate(); 
    const routeChange = useCallback((route) =>{ 
        let path = `/${route}`; 
        navigate(path);
    }, [navigate])

    useEffect(() => {
        let email = Cookies.get("email").toLowerCase()
        let password = Cookies.get("password");
        if (email === undefined) {
            routeChange("./Login");
            return;
        }
        getUserChats(email, password).then((chats) => setChats(chats));
    }, [])

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
            <Chats chats={chats} current_user={Cookies.get("id")}/>
        </div>
    )

}

export { Messenger };