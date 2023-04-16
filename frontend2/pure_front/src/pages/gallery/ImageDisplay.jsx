import './ImageDisplay.css';
import React, {useEffect} from 'react';

const ImageDisplay = ({ img, onClose, onPrevious, onNext }) => {
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

  const handleKeyDown = (event) => {
    if (event.key === 'Escape') {
    onClose();
    } else if (event.key === 'ArrowLeft' || event.keyCode === 65) {
    handlePrevious();
    } else if (event.key === 'ArrowRight' || event.keyCode === 68) {
    handleNext();
    }
    };
    
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
      <button className="button-previous" onClick={handlePrevious}>
        Previous
      </button>
      <div className="image-container">
        <img src={img} className="image" />
      </div>
      <button className="button-next" onClick={handleNext}>
        Next
      </button>
    </div>
  );
};

export {ImageDisplay}