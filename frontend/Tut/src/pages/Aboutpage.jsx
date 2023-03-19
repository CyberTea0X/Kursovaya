import React, {useState} from 'react';
import Masonry,{ResponsiveMasonry} from 'react-responsive-masonry';


const images = [
"https://w-dog.ru/wallpapers/5/19/376948382466555/koshka-kot-polosatyj-glaza-zelenye-sidit-lavka-skamya.jpg",
"https://picsum.photos/3000/1500",
"https://picsum.photos/1000/1500",
"https://picsum.photos/1500/1500",
"https://picsum.photos/2500/1500",
]

const Aboutpage = () => {
    const [data, setData] = useState({img: ', i: 0'})


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
            {data.img &&
                <div style={{
                    width: '100%',
                    height: '100vh',
                    background: 'black',
                    position: 'fixed',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    overflow: 'hidden',
                }}>
                    <button onClick={() => imgAction()} style={{position: 'absolute', top: '10px', right: '10px'}} >X</button>
                    <button onClick={() => imgAction('previous-img')} >Previous</button>
                    <img src={data.img} style={{width: 'auto', maxWidth: '90%', maxHeight: '90%'}}/>
                    <button onClick={() => imgAction('next-img')}>Next</button>
                </div>
            }
            <div style={{padding: '10px'}}>
                <ResponsiveMasonry
                    columnsCountBreakPoints={{350: 1, 750: 2, 900: 3}}
            >
                    <Masonry gutter='10px'>
                        {images.map((image, i) => (
                            <img
                                key={i}
                                src={image}
                                style={{width: "100%", display: "block", cursor: 'pointer'}}
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

export {Aboutpage};
