import React, { useState, useEffect } from 'react';
import Masonry, { ResponsiveMasonry } from 'react-responsive-masonry';
import './gallery.css';
import User from '../account/Unknown_person.jpg';
import { ImageDisplay } from './ImageDisplay';
import { FaPlusSquare } from 'react-icons/fa';


const Gallery = () => {
  const [data, setData] = useState({ img: '', i: 0 });
  const [uploadFormActive, setUploadFormActive] = useState(false);
  const [images, setImages] = useState([]);
  const totalImages = images.length;

  // function to display an image
  const viewImage = (img, i) => {
    setData({ img, i });
  };

  // function to handle image navigation
  const imgAction = (action) => {
    let i = data.i;
    if (action === 'next-img') {
      i = (i + 1) % totalImages;
      setData({ img: images[i], i });
    }
    if (action === 'previous-img') {
      i = (i - 1 + totalImages) % totalImages;
      setData({ img: images[i], i });
    }
    if (!action) {
      setData({ img: '', i: 0 });
    }
  };

  // function to handle image upload
  const handleImageUpload = (e) => {
    if (e.target.files.length == 0) {
      return;
    }
    const file = e.target.files[0];
    const reader = new FileReader();

    reader.onload = () => {
      const newImages = [...images, reader.result]; 
      // создаем новый массив, содержащий все старые изображения
      // и новое добавленное изображение
      setImages(newImages); // устанавливаем новый массив в качестве состояния images
    };

    reader.readAsDataURL(file);
  };

  // render the gallery UI
  return (
    <>
      {/* display the image button */}
      {data.img ? (
        <ImageDisplay
          img={data.img}
          onClose={() => imgAction()}
          onPrevious={() => imgAction('previous-img')}
          onNext={() => imgAction('next-img')}
        />
      ) : null}

      {/* display the user profile image */}
      <img className="profile-img" src={User} alt="" />

      {/* display the image grid */}
      <div style={{ padding: '10px' }}>
        <ResponsiveMasonry
          columnsCountBreakPoints={{ 350: 3, 750: 4, 900: 5 }}
        >
          <Masonry gutter="10px">
            {images.map((image, i) => (
              <img
                key={i}
                src={image}
                style={{
                  width: '100%',
                  display: 'block',
                  cursor: 'pointer',
                  borderRadius: '30px',
                }}
                alt=""
                onClick={() => viewImage(image, i)}
              />
            ))}
          </Masonry>
        </ResponsiveMasonry>
      </div>

      {/* display the image upload form */}
      <div
        className={`upload-form ${uploadFormActive ? 'active' : ''}`}
        onClick={() => setUploadFormActive(!uploadFormActive)}
      >
        <label htmlFor="image-upload" className="upload-label">
          <FaPlusSquare size={50} color="#ccc" />
        </label>
        <input
          type="file"
          id="image-upload"
          accept="image/*"
          onChange={handleImageUpload}
        />
      </div>
    </>
  );
};
  
export { Gallery };