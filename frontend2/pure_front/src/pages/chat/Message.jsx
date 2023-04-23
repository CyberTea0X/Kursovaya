import React from "react";
import "./styles/message.css";
import { Avatar } from "../../sharedComponents/Avatar";

let Message = ({message, userId1}) => {
    return (
        <div className={(userId1 == message.owner_id) ? 'message_sent': 'message_received'}>
            {message.content}
        </div>
    )
}

export {Message}