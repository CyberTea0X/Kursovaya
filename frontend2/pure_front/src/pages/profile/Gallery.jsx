import React, { useState, useEffect, useCallback } from 'react';
import Masonry, { ResponsiveMasonry } from 'react-responsive-masonry';
import { FaPlusSquare } from 'react-icons/fa';
import { ImageView } from './ImageDisplay';
import { upload_image } from '../../server/requests';
import { getImages } from '../../server/requests_handler';
import Cookies from 'js-cookie';
import './profile.css';


const Gallery = ({user, isOwner}) => {
  const [data, setData] = useState({ img: '', i: 0 });
  const [uploadFormActive, setUploadFormActive] = useState(false);
  const [images, setImages] = useState([]); // хранение данных об изображениях
  const totalImages = images.length;
  // function to display an image
  const viewImage = (img, i) => {
    setData({ img, i });
  };

  const getGallery = useCallback( async () => {
    setImages(await getImages(user.id));
  }, [user.id])

  useEffect(() => {
    if (user.id === undefined) {
      return;
    }
    getGallery();
  }, [user, getGallery]);

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
    if (e.target.files.length === 0) {
      return;
    }
    const file = e.target.files[0];
    let email = Cookies.get("email").toLowerCase();
    let pw = Cookies.get("password");
    upload_image(email, pw, file).then(() => {getGallery()})
  };

  // render the gallery UI
  return (
    <>
      {/* display the image button */}
      {data.img ? (
        <ImageView
          img={data.img}
          onClose={() => imgAction()}
          onPrevious={() => imgAction('previous-img')}
          onNext={() => imgAction('next-img')}
          isOwner={isOwner}
        />
      ) : null}
      <div style={{ padding: '10px'}}>
        <ResponsiveMasonry
          columnsCountBreakPoints={{ 350: 3, 750: 4, 900: 5 }}
        >
          <Masonry gutter="10px">
            {images.map((image, i) => (
              <img
                key={i}
                src={image.url}
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
      {isOwner && (
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
      )}
    </>
  );
};

export { Gallery };