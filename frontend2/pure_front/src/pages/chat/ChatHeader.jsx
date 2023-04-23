import React from "react";
import "./styles/chat_header.css";
import { Avatar } from "../../sharedComponents/Avatar";

let ChatHeader = ({onBack, user2, onAvatarClick}) => {
    return (
        <div className="chat-header-container">
            <button className="chat-button-back" onClick={onBack}> Назад </button>
            <h3 className="chat-others-username"> {(user2) ? user2.username: "Loading..."} </h3>
            <h3 className={"chat-others-fullname"}>  {((user2) ? `(${user2.firstname} ${user2.lastname})`: " ")} </h3>
            <Avatar user_id={(user2) ? user2.id: undefined} onClick={onAvatarClick}> </Avatar>
        </div>
    )
}

export {ChatHeader}