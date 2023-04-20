import React, { useState, useEffect } from 'react';
import Masonry, { ResponsiveMasonry } from 'react-responsive-masonry';
import './profile.css';
import UnknownPerson from "../img/Unknown_person.jpg";
import { ImageView } from './ImageDisplay';
import { FaPlusSquare } from 'react-icons/fa';
import { User } from '../../types';
import { useNavigate } from 'react-router-dom';
import Cookies from 'js-cookie';
import { getUserProfile, getImages, getTagsArray, getAvatarImage } from '../../server/requests_handler';
import { upload_image, visit } from '../../server/requests';
import { useParams } from 'react-router-dom';


const Profile = () => {
  const [data, setData] = useState({ img: '', i: 0 });
  const [uploadFormActive, setUploadFormActive] = useState(false);
  const [images, setImages] = useState([]);
  const [user, setUser] = useState(User.emptyUser());
  const [avatar, setAvatar] = useState(UnknownPerson);
  const [isOwner, setIsOwner] = useState(false);
  const [tags, setTags] = useState([]); // хранение данных о тегах
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
        userIdOrEmail = (userId == "me") ? Cookies.get("email").toLowerCase(): userId;
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

  const loadAvatar = async () => {
    let logo_img = await getAvatarImage(user.id);
    if (logo_img !== undefined) {
      setAvatar(logo_img.url);
    }
  }

  // function to display an image
  const viewImage = (img, i) => {
    setData({ img, i });
  };

  const retrieveUserTags = async () => {
    try {
        let tags_arr = await getTagsArray(user.id);
        if (tags_arr.length > 0) {
          setTags(tags_arr);
        }
    }
    catch (error) {
        alert(error.message);
    }
  }
  

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
    if (user.id == undefined) {
      return;
    }
    retrieveUserTags()
    if (images.length == 0) {
      getGallery();
    }
    if (user.id == Cookies.get("id")) {
      setIsOwner(true)
    }
    else {
      let email = Cookies.get("email").toLowerCase();
      let pw = Cookies.get("password");
      visit(email, pw, user.id);
    }
    loadAvatar()
  }, [user]);


  useEffect(() => {
    if (user.id == undefined) {
      return;
    }
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
      <div className="user-info">
        <div className="main-info">
          <div className="user-name"> {user.username} </div>
          <img className="profile-img" src={avatar || UnknownPerson} alt="" />
          <div className="rating-container">
            <b> Рейтинг: </b>
            <div> {user.rating} </div>
          </div>
          <div className='message-user'>
            Сообщение
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
          <div className='user-tags'>
            {tags.map((tag, i) => (
              <span key={i}> {tag} </span>
            ))}
          </div>
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
  
export { Profile };