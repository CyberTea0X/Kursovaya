import "./styles/chatbutton.css";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Avatar } from "../../sharedComponents/Avatar";

const ChatButton = ({chat, current_user}) => {
    const [other, setOther] = useState();
    let navigate = useNavigate(); 

    const goToChat = (user1id, user2id) =>{ 
        let path = `/Chat/${user1id}/${user2id}`; 
        navigate(path);
    }

    const handleChatClick = (chat) => {
        goToChat(chat.userid1, chat.userid2)
    }

    const setOtherUser = (chat, current) => {
        if (parseInt(chat.userid1) == parseInt(current)) {
          // Мы хотим отобразить второго участника чата
          // Пользователю совсем необязательно видеть чат с собой
          setOther(chat.user2);
        } else {
          setOther(chat.user1);
        }
    }

  useEffect(() => {
    if (chat === undefined || current_user === undefined) {
      return;
    }
    setOtherUser(chat, current_user);
  }, [chat, current_user]);

  return (
    <> 
    <Avatar user_id={other?.id} onClick={() => handleChatClick(chat)} />
    <div className='chat-item'> {other?.username} </div>
    </>
  );
};

export { ChatButton };