import React from "react";
import "./styles/message.css";
import { Avatar } from "../../sharedComponents/Avatar";

let Message = ({message, current_user}) => {
    return (
        <div className={(current_user.id == message.owner_id) ? 'message_sent': 'message_received'}>
            {message.content}
        </div>
    )
}

export {Message}