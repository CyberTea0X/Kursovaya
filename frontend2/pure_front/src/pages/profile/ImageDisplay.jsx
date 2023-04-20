import { FaPencilAlt, FaTrashAlt, FaEllipsisH } from 'react-icons/fa';
import './ImageDisplay.css';
import React, { useEffect, useState } from 'react';
import { edit_image_data, delete_image, set_as_avatar } from '../../server/requests';
import Cookies from 'js-cookie';
import { Menu, useContextMenu, Item } from 'react-contexify';
import "react-contexify/dist/ReactContexify.css";


const ImageView = ({ img, onClose, onPrevious, onNext, isOwner}) => {
  const [isEditing, setIsEditing] = useState(false);
  const [description, setDescription] = useState(img.about);
  const [title, setTitle] = useState(img.image_name);
  const MENU_ID = "menu-id";

  const { show } = useContextMenu({
    id: MENU_ID
  });

  function handleContextMenu(e){
    show({
      event: e,
    });
  }

  const handlePrevious = () => {
    if (onPrevious) {
      onPrevious();
    }
  };

  const handleNext = () => {
    if (onNext) {
      onNext();
    }
  };

  const handleSave = () => {
    setIsEditing(false);
    let email = Cookies.get("email").toLowerCase();
    let pw = Cookies.get("password");
    edit_image_data(email, pw, img.id, description, title);
  };

  const handleEdit = () => {
    if (isEditing) {
        setIsEditing(false);
    }
    else {
        setIsEditing(true);
    }
  };

  const handleDelete = () => {
    const email = Cookies.get("email").toLowerCase();
    const pw = Cookies.get("password");
    delete_image(email, pw, img.id);
    onPrevious();
  }

  const handleSetAvatar = async () => {
    const email = Cookies.get("email").toLowerCase();
    const pw = Cookies.get("password");
    const id = Cookies.get("id");
    const data = await set_as_avatar(email, pw, img.id);
    console.log(data);
  }    

  let handleKeyDown = {};

  useEffect(() => {
    handleKeyDown = (event) => {
      if (isEditing === false) {
        if (event.key === 'Escape') {
          onClose();
        } else if (event.key === 'ArrowLeft' || event.keyCode === 65) {
          handlePrevious();
        } else if (event.key === 'ArrowRight' || event.keyCode === 68) {
          handleNext();
        }
      }
    };
  
    document.addEventListener('keydown', handleKeyDown);
  
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [isEditing, onClose, handlePrevious, handleNext]);

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose, onPrevious, onNext]);

  useEffect(() => {
    setDescription(img.about);
    setTitle(img.image_name);
  }, [img]);

  return (
    <div className="image-display">
      <button className="button-close" onClick={onClose}>
        X
      </button>
      <div className="image-container">
        <img src={img.url} className="image" />
        {isOwner &&
        (
        <div className='image-toolbar'>
          <div style={{ color: 'white' }} onClick={handleEdit}>
              <FaPencilAlt />
          </div>
          <div style={{ color: 'white' }} onClick={handleDelete}>
              <FaTrashAlt />
          </div>
          <div style={{ color: "white" }} onClick={handleContextMenu}>
            <FaEllipsisH />
          </div>
              <Menu id={MENU_ID}>
                <Item onClick={handleSetAvatar}>Установить как аватар</Item>
              </Menu>
        </div>
        )
        }
        {isEditing && (
        <div className="edit-container">
            <input
            className="title-input"
            placeholder="Enter title"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            />
            <textarea
            className="description-input"
            placeholder="Enter description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            />
            <button className="save-button" onClick={handleSave}>
            Save
            </button>
        </div>
        )}
        {!isEditing && (
        <div>
            <h2 className="title">{title}</h2>
            <p className="description">{description}</p>
        </div>
        )}
      </div>
      <button className="button-previous" onClick={handlePrevious}>
        Previous
      </button>
      <button className="button-next" onClick={handleNext}>
        Next
      </button>
    </div>
  );
};

export { ImageView };