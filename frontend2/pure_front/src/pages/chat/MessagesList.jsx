import React from "react";
import "./styles/messages_list.css";
import { Message } from "./Message";

const MessageList = React.forwardRef(({messages, userId1}, ref) => {
    return (
        <div className="messages-container">
            {messages.map((msg, id) => (
                <Message key={id} message={msg} userId1={userId1} />
            ))}
            <div ref={ref} />
        </div>
    )
});

export {MessageList}