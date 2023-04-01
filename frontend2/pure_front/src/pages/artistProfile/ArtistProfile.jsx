import React, {useState} from 'react';
import Masonry,{ResponsiveMasonry} from 'react-responsive-masonry';
import "./artistProfile.css";
import User from "../account/Unknown_person.jpg"
const images = [
"https://i.pinimg.com/474x/98/64/58/98645877a607cc968e459047a6eba601.jpg",
"https://i.pinimg.com/564x/24/10/32/241032236c043b846d106cc36d26f5aa.jpg",
"https://i.pinimg.com/474x/62/77/90/6277908fb99b73cff178cc590e29f79c.jpg",
"https://i.pinimg.com/474x/90/7c/40/907c40a5d8c318f8cb1235677b73372b.jpg",
"https://i.pinimg.com/474x/ab/f7/b4/abf7b45e7dd6e108872bb9ef505d4818.jpg",
"https://i.pinimg.com/474x/97/33/12/973312f9abd98ccfb541890d3750ad6e.jpg",
"https://i.pinimg.com/474x/4e/39/c7/4e39c7322805c2b1176e8917603af005.jpg",
"https://i.pinimg.com/474x/12/e3/73/12e373a1e3b51aadad0d1153bf40bbf9.jpg",
"https://i.pinimg.com/474x/73/52/ef/7352ef46912cc353637425fab96027c7.jpg",
"https://i.pinimg.com/474x/e3/40/f7/e340f7457647c291d249d2abfa610c4e.jpg",
"https://i.pinimg.com/474x/b4/b5/43/b4b5433a7722124be16ae5dd5cf410aa.jpg",
"https://i.pinimg.com/474x/18/35/1f/18351f19449959523259c52efa5e15eb.jpg",
"https://i.pinimg.com/474x/e9/f8/4b/e9f84b03ae41bed71a9047fd254e0e78.jpg",
"https://i.pinimg.com/474x/38/d6/64/38d6640bdd5997bb49d035f21ccb5f6e.jpg",
"https://i.pinimg.com/474x/24/fa/f7/24faf7e2d2d1195e879d3a86bf138955.jpg",
]

const ArtistProfile = () => {
    const [data, setData] = useState({img: '', i: 0})


    const viewImage = (img, i) => {

        setData({img, i})
    }
    const imgAction = (action) => {
        let i = data.i;
        if(action === 'next-img')
            {
                setData({img: images[i + 1], i: i + 1});
            }
        if(action === 'previous-img')
            {
                setData({img: images[i - 1], i: i - 1});
            }
        if(!action)
        {
            setData({img: '', i: 0});
        }
        
    }
    return (
        <>
            <div>
                <img className="profile-img" src={User} alt="" />                        
            </div>
            <div  >
         
                {data.img &&
                    <div >                        
                        <div style={{
                            width: '100%',
                            height: '90vh',                          
                            position: 'fixed',
                            background: "rgba(0, 0, 0, 0.85)",                                    
                            display: 'flex',
                            justifyContent: 'center',
                            alignItems: 'center',
                            overflow: 'hidden',                                    
                        }}>
                            
                                    
                            <button className='buttons' onClick={() => imgAction()} style={{position: 'absolute', top: '10px', right: '10px'}}>X</button>
                            <button className='buttons' onClick={() => imgAction('previous-img')}>Previous</button>                                   
                            <div style={{
                                verticalAlign: 'text-bottom',
                                alignContent: 'center',
                                maxWidth:'30%',                                        
                                justifyContent: 'center',
                                alignItems: 'center',
                                verticalAlign: 'middle',
                            }}>
                                <center>
                                    <img src={data.img} style={{width: 'auto',  maxHeight: '100%', borderRadius: '30px', maxHeight:'600px', padding: '10px'}}/>
                                    <div style={{
                                        color: 'white',
                                        backgroundColor: "rgb(46, 4, 43)",
                                        borderRadius: '10px',                                       
                                        padding: '10px',
                                        flexDirection: 'column',
                                        wordWrap: 'break-word',
                                    }}> 
                                        Всем привет дорогие друзья, это канал куплинов плей и сегодня мы играем в симулять очка
                                    </div>
                                </center>
                            </div>
                                <button className='buttons' onClick={() => imgAction('next-img')}>Next</button>
                                    
                        </div>
                            
                    </div>
                        }
                    

                    
            </div>
           
            <div style={{padding: '10px'}}>
                <ResponsiveMasonry
                    columnsCountBreakPoints={{350: 3, 750: 4, 900: 5}}
                >
                    <Masonry gutter='10px'>                    
                        {images.map((image, i) => (
                            <img
                                key={i}
                                src={image}
                                style={{width: "100%", display: "block", cursor: 'pointer', borderRadius: '30px'}}
                                alt=""
                                onClick={()=> viewImage(image, i)}
                                
                            />
                            
                    ))}
                    
                    </Masonry>
                </ResponsiveMasonry>
            </div>
          
    </>
    );
};

export {ArtistProfile};
