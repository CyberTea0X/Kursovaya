import React, {useState, seMemo, useCallback, useContext} from "react";
//21import Img from "../img/img6.png ";
import {useDropzone} from "react-dropzone";

import Style  from './userprofile.css'
import images from "../img/img1.png"
import Form from  "../userPageForm/Form";

const Account = () => {
    const [fileUrl, setFileUrl] = useState(null);
    return (
    <div className={Style.account}>
     <div className={Style.account_info}>
        <h1>Profile settings</h1>
        <p>
            Функционал
        </p>
     </div>
     <div className={Style.account_box}>
        <div className={Style.account_box_img}>
            <input />
            <img src ={"../img/img6.png "} style={{width: '100%', borderRadius: '30px'}}/>
            <p className={Style.account_box_img_para}>Change Image</p>
        </div>
        <div className={Style.account_box_from}>
            <Form />
        </div>
     </div>
     </div>
    ) 
};

export {Account};