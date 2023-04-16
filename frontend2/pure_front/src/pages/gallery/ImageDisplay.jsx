import './ImageDisplay.css';

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