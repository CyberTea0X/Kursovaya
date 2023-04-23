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
    const [current, setCurrent] = useState();
    const [other, setOther] = useState();
    const [messages, setMessages] = useState([]);
    const [message, setMessage] = useState("");
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
        let password = Cookies.get("password");
        send_message(email, password, other.id, message).then(
            getChatMessages(email, password, other.id).then(messages => setMessages(messages))
        )
    }

    const recognizeUser = (user) => {
        if (parseInt(user.id) === parseInt(Cookies.get("id"))) {
            setCurrent(user);
        }
        else {
            setOther(user)
        }
    }

    useEffect(() => {
        let email = Cookies.get("email");
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
        if (other === undefined) {
            return;
        }
        let email = Cookies.get("email").toLowerCase()
        let password = Cookies.get("password");
        getChatMessages(email, password, other.id).then(messages => setMessages(messages))
    }, [other]);

    useEffect(() => {
        if (user1 !== undefined) {
            recognizeUser(user1)
        }
    }, [user1]);

    useEffect(() => {
        if (user2 !== undefined) {
            recognizeUser(user2)
        }
    }, [user2]);

    useEffect(() => {
        if (user1 === undefined || user2 === undefined) {
            return;
        }
        if (user1.id == user2.id) {
            setOther(user1);
        }
    }, [user1, user2]);

    return (
        <div className="chat-container">
            <ChatHeader user2={other} onBack={goToMessenger} onAvatarClick={() => goToProfile(other.id)}> </ChatHeader>
            <MessageList ref={msgList} messages={messages} />
            <MessageInput onSubmit={handleSendMessage} message={message} setMessage={setMessage} />
        </div>
    )
}

export {Chat}