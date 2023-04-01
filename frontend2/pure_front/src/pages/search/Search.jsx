import { useState } from 'react'
import { Link } from 'react-router-dom'
import './search.css'
import { Users } from "./users"
import { Table } from "./table"

const SearchPage = () => {
    const [query, setQuery] = useState("");

    const keys = ["first_name", "last_name", "email"]

    const search = (data) => {
        return data.filter((item) => 
        keys.some(key=>item[key].toLowerCase().includes(query.toLowerCase()))

        );
    };


    return (
        
        <div className="searchpage">
            <input type="text" placeholder='Поиск...' className="search" onChange={e=> setQuery(e.target.value)} />
           <Table data={search(Users)}/>
        </div>
    )
}

export {SearchPage};
