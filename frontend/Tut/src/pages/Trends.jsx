import { Link } from 'react-router-dom'
import {Text} from 'react'
    import React from 'react'
    import Img1 from './img/img1.png'
    import Img2 from './img/img2.png'
    import Img3 from './img/img3.png'
    import Img4 from './img/img4.png'
    import Img5 from './img/img5.png'
    import Img6 from './img/img6.png'

    import './trends.css';

const Trends = () => {
    let data  = [
        {
            id: 1,
            imgSrc: Img1,
            name: "Vasa",
        },
        {
            id: 2,
            imgSrc: Img2,
            name: "Peta",
        },
        {
            id: 3,
            imgSrc: Img3,
            name: "Andrew",
        },
          {
            id: 4,
            imgSrc: Img4,
            name: "Tolia",
        },
        {
            id: 5,
            imgSrc: Img5, 
            name: "Vova",
        },
        {
            id: 6,
            imgSrc: Img6, 
            name: "Dasha",
        }
    ]
   
    return (
        <>  
            <h2  style={{textAlign: 'right'}}>Вернуться на <Link to="/">главную</Link></h2>      
            <h1 style={{textAlign: 'center'}}>Популярное</h1>
            <div className= "gallery">
                {data.map((item, index)=>{
                    return(
                        <div className="pics" key={index} >
                            <img src ={item.imgSrc} style={{width: '50%'}}/>
                            <p style ={{textAlign: 'left'}}>
                                {item.name}
                            </p>
                        </div>
                    )
                })}
            </div>
        </>
        
        
    )
}

export {Trends}
