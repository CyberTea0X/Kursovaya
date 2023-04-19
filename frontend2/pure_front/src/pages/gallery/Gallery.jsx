import React, { useState, useEffect } from 'react';
import Masonry, { ResponsiveMasonry } from 'react-responsive-masonry';
import './gallery.css';
import UnknownPerson from '../account/Unknown_person.jpg';
import { ImageView } from './ImageDisplay';
import { FaPlusSquare } from 'react-icons/fa';
import { User } from '../../types';
import { useNavigate } from 'react-router-dom';
import Cookies from 'js-cookie';
import { getUserProfile, getImages } from '../../server/requests_handler';
import { upload_image } from '../../server/requests';
import { useParams } from 'react-router-dom';


const Gallery = () => {
  const [data, setData] = useState({ img: '', i: 0 });
  const [uploadFormActive, setUploadFormActive] = useState(false);
  const [images, setImages] = useState([]);
  const [user, setUser] = useState(User.emptyUser());
  const [logo, setLogo] = useState(UnknownPerson);
  const [isOwner, setIsOwner] = useState(false);
  const { userId } = useParams();
  const totalImages = images.length;

  let navigate = useNavigate(); 
  const routeChange = (route) =>{ 
      let path = `/${route}`; 
      navigate(path);
  }
    
  const getAccount = async () => {
    let userIdOrEmail;
      try {
        userIdOrEmail = (userId == -1) ? Cookies.get("email").toLowerCase(): userId;
          let user_ = await getUserProfile(userIdOrEmail);
          setUser(user_);
      }
      catch (error) {
          if (error instanceof TypeError) {
              routeChange("Login");
              return;
          }
          alert(error.message);
      }
  }

  const getGallery = async () => {
    setImages(await getImages(user.id));
  }

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
    let email = Cookies.get("email").toLowerCase();
    let pw = Cookies.get("password");
    upload_image(email, pw, file).then(() => {getGallery()})
  };

  useEffect(() => {
    getAccount();
  }, []);
  useEffect(() => {
    if (images.length == 0) {
      getGallery();
    }
    if (user.id == Cookies.get("id")) {
      setIsOwner(true)
    }
  }, [user]);

  useEffect(() => {
    getGallery();
  }, [data.img]);

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

      {/* display the user profile image */}
      <div className="user-info">
        <div className="main-info">
          <div className="user-name"> {user.username} </div>
          <img className="profile-img" src={logo} alt="" />
          <div className="rating-container">
            <b> Рейтинг: </b>
            <div> {user.rating} </div>
          </div>
        </div>
        <div className="additional-info">
          <div className='user-fullname'>
            <p> {user.firstname}</p>
            <p> {user.lastname}</p>
          </div>
            <p className='user-about'>
              {user.about}
            </p>
        </div>
      </div>

      {/* display the image grid */}
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