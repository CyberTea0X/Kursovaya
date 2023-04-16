import { Link } from 'react-router-dom'
import {Text} from 'react'
    import React from 'react'
    import Img1 from './img/img1.png'
    import Img2 from './img/img2.png'
    import Img3 from './img/img3.png'
    import Img4 from './img/img4.png'
    import Img5 from './img/img5.png'
    
    import './trends.css';
import { Gallery } from '../gallery/Gallery'
    

const Trends = () => {
    let data  = [
        {
            id: 0,
            imgSrc: Img1,
            name: "Vasa",
        },
        {
            id: 1,
            imgSrc: Img2,
            name: "Peta",
        },
        {
            id: 2,
            imgSrc: Img3,
            name: "Andrew",
        },
          {
            id: 3,
            imgSrc: Img4,
            name: "Tolia",
        },
        {
            id: 4,
            imgSrc: Img5, 
            name: "Vova",
        },
        
    ]
   
    return (
        <div className='trends'>  
            <div  style={{
                fontSize: '30px',
                textAlign: 'right',
               
                color: 'rgb(18, 6, 28)', 
            }}>
            </div>      
            <div style={{
                textAlign: 'center',
                backgroundColor: 'rgb(56, 56, 56)',
                color: 'rgb(241, 241, 241)',
                fontSize: '30px',
                padding: '40px 30px',
                
            }}>Найдите что-то для себя</div>
            <div className= "gallery">
                {data.map((item, index)=>{
                    return(
                        <div className="pics" key={index} >
                            
                            <img src ={item.imgSrc} style={{width: '100%', borderRadius: '30px', border: '2px solid black'}}/>
                            <Link to="/gallery" className='trends-link'>{item.name}</Link>
                             
                            
                        </div>
                    )
                })}
            </div>
        </div>
        
        
    )
}

export {Trends}
