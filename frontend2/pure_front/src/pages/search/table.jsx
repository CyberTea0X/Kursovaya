import React from "react";


const Table = ({ data }) => {
    return (
      <table>
        <tbody>
          <tr>
            
            {/* <th>Profile</th>
            <th>Name</th>
            <th>Surname</th> */}
          </tr>
          {data.map((item) => (
            <tr key={item.id}>
              <td>{<img src="https://avatars.dzeninfra.ru/get-zen_doc/1884623/pub_60be1f9abcbf42494ea7da85_60be206a746af706906e32df/scale_1200" style={{
                width: '50px',
                height: '50px',
                borderRadius: '25px',
              }} />}{item.first_name}</td>
              <td>{item.email}</td>
              <td>{item.last_name}</td>
            </tr>
          ))}
        </tbody>
      </table>
    );
  };

    export {Table}