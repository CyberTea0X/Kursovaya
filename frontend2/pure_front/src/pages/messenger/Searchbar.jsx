import React from 'react';
import { FaSearch } from 'react-icons/fa';
import "./styles/search.css";

const Searchbar = ({onSearch}) => {

    return (
        <div className='search-container'>
          <FaSearch className='search-icon' />
          <input type="text" onChange={onSearch} className='search-input' />
        </div>
      );
};

export { Searchbar };