import React, {useState} from "react";
import "./styles/message_input.css";
import { FaPaperPlane } from 'react-icons/fa';


let MessageInput = ({onSubmit}) => {
    const [message, setMessage] = useState();

    return (
        <div className="chat-message-input-container">
            <form className="input-container" onSubmit={onSubmit}>
            <input 
                type="text" 
                placeholder="Type your message here..." 
                className="chat-message-input"
                value={message} 
                onChange={(event) => setMessage(event.target.value)} 
            />
            <button className="message-button-send" onClick={onSubmit}>
            <FaPaperPlane />
            </button>
            </form>
        </div>
    )
}

export {MessageInput}