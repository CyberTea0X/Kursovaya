import './carousel.css'

export const Carousel = ({children}) => {
    return (
        <div className="main-container">
            <div className="window">
                <div className="all-items-container"> {children} </div>
            </div>
        </div>
    )
}