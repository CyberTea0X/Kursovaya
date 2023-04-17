import { useState } from "react";
import "./registration.css";
import "./registration_main.css";
import FormInput from "./FormInput";
import { useNavigate } from 'react-router-dom';
import { registerUser } from "../../server/requests";
import { userToCookies, removeOldCookies } from "../../cookies";
import { getUserProfile } from "../../server/requests_handler";

const Signup = () => {
  const [values, setValues] = useState({
    username: "",
    email: "",
    password: "",
    confirmPassword: "",
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
        "Password should be at least 8 characters and include at least 1 letter, 1 number and 1 special character!",
      label: "Password",
      pattern: `^(?=.*[!@#$%^&*])[a-zA-Z0-9!@#$%^&*]{8,}$`,
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
  ];

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await registerUser(values.username, values.email.toLowerCase(), values.password)
      .then(data => {
          if (data["status"] != "OK") {
            throw Error(data["reason"]);
        }
      });
    }
    catch (error) {
      alert(error.message)
      return;
    }
    let user = await getUserProfile(values.email.toLowerCase());
    removeOldCookies();
    userToCookies(values["email"].toLowerCase(), values.password, user.id);
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
        <center>
        <p className="register-link">
        Уже есть аккаунт?{" "}
        <a href="/Login">Вход</a>
      </p>
      </center>
      </form>
    </div>
  );
};

export {Signup};
