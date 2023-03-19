import { Carousel } from '../carousel/Carousel'
import './home.css'
import '../carousel/carousel.css'

const Homepage = () => {
    return (
        <div>
            <h1>Get started with React-Router 6</h1>
            <Carousel>
                <div className="item item-1"> Item 1</div>
                <div className="item item-2"> Item 2</div>
                <div className="item item-2"> Item 3</div>
            </Carousel>
        </div>
    )
}

export {Homepage}
