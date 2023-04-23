import React, {useEffect, useState, useCallback} from "react";
import "./styles/chat.css";
import { ChatHeader } from "./ChatHeader";
import { getUserProfile } from "../../server/requests_handler";
import { useParams } from 'react-router-dom';
import { useNavigate } from "react-router-dom";
import { MessageInput } from "./MessageInput";
import { MessageList } from "./MessagesList";

let Chat = () => {
    const { userId1, userId2 } = useParams();
    const [user1, setUser1] = useState();
    const [user2, setUser2] = useState();

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

    useEffect(() => {
        getUserProfile(userId1).then(user => setUser1(user))
        getUserProfile(userId2).then(user => setUser2(user))
    }, [userId1, userId2]);

    return (
        <div className="chat-container">
            <ChatHeader user2={user2} onBack={goToMessenger} onAvatarClick={() => goToProfile(user2.id)}> </ChatHeader>
            <MessageList/>
            <MessageInput />
        </div>
    )
}

export {Chat}