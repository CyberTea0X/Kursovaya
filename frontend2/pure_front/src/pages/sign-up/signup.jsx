import { useState } from "react";
import "./registration.css";
import "./registration_main.css";
import FormInput from "./FormInput";
import { useNavigate } from 'react-router-dom';
import { registerUser } from "../../server/server";

const Signup = () => {
  const [values, setValues] = useState({
    username: "",
    email: "",
    password: "",
    confirmPassword: "",
    firstname: "",
    lastname: "",
    age: "",
    gender: "",
    about: "",
  });




  let navigate = useNavigate(); 
  const routeChange = () =>{ 
    let path = `/`; 
    navigate(path);
  }

  const inputs = [
    {
      id: 1,
      name: "username",
      type: "text",
      placeholder: "Username",
      errorMessage:
        "Username should be 3-16 characters and shouldn't include any special character!",
      label: "Username",
      pattern: "^[A-Za-z0-9]{3,16}$",
      required: true,
    },
    {
      id: 2,
      name: "email",
      type: "email",
      placeholder: "Email",
      errorMessage: "It should be a valid email address!",
      label: "Email",
      required: true,
    },
    
    {
      id: 3,
      name: "password",
      type: "password",
      placeholder: "Password",
      errorMessage:
        "Password should be 8-20 characters and include at least 1 letter, 1 number and 1 special character!",
      label: "Password",
      pattern: `^(?=.*[0-9])(?=.*[a-zA-Z])(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{8,20}$`,
      required: true,
    },
    {
      id: 4,
      name: "confirmPassword",
      type: "password",
      placeholder: "Confirm Password",
      errorMessage: "Passwords don't match!",
      label: "Confirm Password",
      pattern: values.password,
      required: true,
    },
    {
      id: 5,
      name: "firstname",
      type: "text",
      placeholder: "First Name",
      errorMessage:
        "First name should be 2-20 characters and shouldn't include any special character or number!",
      label: "First Name",
      pattern: "^\\p{L}{2,20}$",
      required: true,
    },
    {
      id: 6,
      name: "lastname",
      type: "text",
      placeholder: "Last Name",
      errorMessage:
        "Last name should be 2-20 characters and shouldn't include any special character or number!",
      label: "Last Name",
      pattern: "^\\p{L}{2,20}$",
      required: true,
    },
    {
      id: 7,
      name: "age",
      type: "number",
      placeholder: "Age",
      errorMessage: "Age should be a number between 18 and 99!",
      label: "Age",
      min: 18,
      max: 99,
      required: true,
    },
    {
      id: 8,
      name: "gender",
      type: "text",
      placeholder: "Gender",
      errorMessage: "Please enter your gender!",
      label: "Gender",
      pattern: "^\\p{L}{3,6}$",
      required: true,
    },
    {
      id: 9,
      name: "about",
      type: "textarea",
      placeholder: "Tell us about yourself…",
      errorMessage:
        "About should be 5-500 characters and shouldn't include any special character!",
      label: "About",
      pattern: "^\\p{L}{5,500}$",
      required: true,
    },
  ];

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log(registerUser(values.username, values.email.toLowerCase(), values.password,
      values.firstname, values.lastname, values.age, values.gender, values.about))
    routeChange();
  };

  const onChange = (e) => {
    setValues({ ...values, [e.target.name]: e.target.value });
  };

 

  return (
    <div className="reg">
      <form onSubmit={handleSubmit}>
        <h1 className="h1">Регистрация</h1>
        {inputs.map((input) => (
          <FormInput
            key={input.id}
            {...input }
            value={values[input.name]}
            onChange={onChange}
          />
        ))}
        <button className="register-button">Регистрация</button>
      </form>
    </div>
  );
};

export {Signup};
