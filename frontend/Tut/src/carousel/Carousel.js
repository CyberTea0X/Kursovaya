import { useEffect, useState, Children, cloneElement } from 'react'
import './carousel.css'

export const Carousel = ({children}) => {
    const [pages, setPages] = useState([])

    useEffect(() => {
        setPages(
            Children.map(children, (child) => {
             return cloneElement(child, {
                style: {
                    height: '100%',
                    widht: '100%',                   
                },
            })
        })
    )    
    }, []) 



    return (
        <div className="main-container">
            <div className="window">
                <div className="all-items-container"> {pages} </div>
            </div>
        </div>
    )
}