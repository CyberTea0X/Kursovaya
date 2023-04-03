import React, { useState } from 'react';

function SearchForm({ onSubmit }) {
  const [query, setQuery] = useState('');

  const handleSubmit = (event) => {
    event.preventDefault();
    onSubmit(query);
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="text" value={query} onChange={(event) => setQuery(event.target.value)} />
      <button type="submit">Search</button>
    </form>
  );
}

function SearchResult({ result }) {
  return (
    <div>
      <h2>{result.title}</h2>
      <p>{result.description}</p>
    </div>
  );
}