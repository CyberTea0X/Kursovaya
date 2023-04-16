import { FaPencilAlt } from 'react-icons/fa';
import './ImageDisplay.css';
import React, { useEffect, useState } from 'react';

const ImageDisplay = ({ img, onClose, onPrevious, onNext }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [description, setDescription] = useState('Автор ничего не рассказал нам о картинке');
  const [title, setTitle] = useState('Без названия');

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
  };

  const handleEdit = () => {
    if (isEditing) {
        setIsEditing(false);
    }
    else {
        setIsEditing(true);
    }
  };

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

  return (
    <div className="image-display">
      <button className="button-close" onClick={onClose}>
        X
      </button>
      <div className="image-container">
        <img src={img} className="image" />
        <div style={{ color: 'white' }} onClick={handleEdit}>
            <FaPencilAlt />
        </div>
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

export { ImageDisplay };