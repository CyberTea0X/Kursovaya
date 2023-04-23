import React, {useEffect, useState, useCallback, useRef} from "react";
import "./styles/chat.css";
import { ChatHeader } from "./ChatHeader";
import { getUserProfile } from "../../server/requests_handler";
import { useParams } from 'react-router-dom';
import { useNavigate } from "react-router-dom";
import { MessageInput } from "./MessageInput";
import { MessageList } from "./MessagesList";
import { getChatMessages } from "../../server/requests_handler";
import { send_message } from "../../server/requests";
import Cookies from 'js-cookie';

let Chat = () => {
    const { userId1, userId2 } = useParams();
    const [user1, setUser1] = useState();
    const [user2, setUser2] = useState();
    const [messages, setMessages] = useState([]);
    const [message, setMessage] = useState();
    const msgList = useRef(null);

    let navigate = useNavigate(); 
    const routeChange = useCallback((route) =>{ 
        let path = `/${route}`; 
        navigate(path);
    }, [navigate])

    const goToMessenger = useCallback(() =>{ 
        routeChange("./Messenger")
    }, [routeChange])

    const goToProfile = useCallback((user_id) =>{ 
        routeChange(`./Profile/${user_id}`)
    }, [routeChange])

    const handleSendMessage = (e) => {
        e.preventDefault();
        msgList.current?.scrollIntoView({ behavior: "smooth" });
        setMessage("");
        let email = Cookies.get("email").toLowerCase();
        let password = Cookies.get("passwprd");
        send_message(email, password, userId2, message).then(
            getChatMessages(email, password, userId2).then(messages => setMessages(messages))
        )
    }

    useEffect(() => {
        let email = Cookies.get("email");
        let password = Cookies.get("password");
        if (email === undefined) {
            routeChange("./Login");
            return;
        }
    }, [])

    useEffect(() => {
        getUserProfile(userId1).then(user => setUser1(user))
        getUserProfile(userId2).then(user => setUser2(user))
    }, [userId1, userId2]);

    useEffect(() => {
        let email = Cookies.get("email").toLowerCase()
        let password = Cookies.get("password");
        getChatMessages(email, password, userId2).then(messages => setMessages(messages))
    }, [userId2]);

    return (
        <div className="chat-container">
            <ChatHeader user2={user2} onBack={goToMessenger} onAvatarClick={() => goToProfile(user2.id)}> </ChatHeader>
            <MessageList ref={msgList} messages={messages} />
            <MessageInput onSubmit={handleSendMessage} message={message} setMessage={setMessage} />
        </div>
    )
}

export {Chat}