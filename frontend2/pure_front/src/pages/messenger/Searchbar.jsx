import React from 'react';
import "./styles/searchbar.css";
import { FaSearch } from 'react-icons/fa';

const Searchbar = ({onSearch}) => {

    return (
        <div className='search-container'>
          <FaSearch className='search-icon' />
          <input type="text" onChange={onSearch} className='search-input' />
        </div>
      );
};

export { Searchbar };