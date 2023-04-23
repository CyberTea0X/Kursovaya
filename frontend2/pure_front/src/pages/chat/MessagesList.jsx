import React from "react";
import "./styles/messages_list.css";
import { Message } from "./Message";

const MessageList = React.forwardRef(({messages, current_user}, ref) => {
    return (
        <div className="messages-container">
            {messages.map((msg, id) => (
                <Message key={id} message={msg} current_user={current_user} />
            ))}
            <div ref={ref} />
        </div>
    )
});

export {MessageList}