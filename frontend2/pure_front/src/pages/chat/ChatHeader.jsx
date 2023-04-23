import React, { useEffect, useState } from "react";
import "./styles/chat_header.css";
import { Avatar } from "../../sharedComponents/Avatar";

let ChatHeader = ({onBack, user2, onAvatarClick}) => {
    const [firstname, setFirstname] = useState("");
    const [lastname, setLastname] = useState("");
    const [fullname, setFullname] = useState("");

    useEffect(() => {
        if (user2 === undefined) {
            return;
        }
        setFirstname((user2.firstname) ? user2.firstname: "");
        setLastname((user2.lastname) ? user2.lastname: "");

    }, [user2]);

    useEffect(() => {
        if (!firstname && !lastname) {
            return;
        }
        setFullname(`(${firstname} ${lastname})`)
    }, [firstname, lastname]);
    return (
        <div className="chat-header-container">
            <button className="chat-button-back" onClick={onBack}> Назад </button>
            <h3 className="chat-others-username"> {(user2) ? user2.username: "Loading..."} </h3>
            <h3 className={"chat-others-fullname"}>  {fullname} </h3>
            <Avatar user_id={(user2) ? user2.id: undefined} onClick={onAvatarClick}> </Avatar>
        </div>
    )
}

export {ChatHeader}