import React from "react";
import "./styles/message.css";
import { BiCheck, BiCheckDouble } from "react-icons/bi";

let Message = ({message, current_user}) => {
    return (
        <div className={(message.isOwner(current_user)) ? 'message_sent': 'message_received'}>
            {message.content} {message.isOwner(current_user) && (message.is_read ? (<BiCheckDouble />): (<BiCheck />))}
            <time className="message-send-time"> {message.getFormattedTime()} </time>
        </div>
    )
}

export {Message}