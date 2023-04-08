import { useState } from "react";
import "./log_in.css";
import FormInput from "../sign-up/FormInput";
import { useNavigate } from 'react-router-dom';
import Cookies from 'js-cookie';
import { login } from "../../server/server";
import bcrypt from 'bcryptjs';

const Log_in = () => {
  const [values, setValues] = useState({
    email: "",
    password: "",
});




  let navigate = useNavigate(); 
  const routeChange = () =>{ 
    let path = `/`; 
    navigate(path);
  }

  const inputs = [
    {
      id: 1,
      name: "email",
      type: "email",
      placeholder: "Email",
      errorMessage: "На Ваш email не зарегистрирован аккаунт или некорректно введён адрес электронной почты",
      label: "Email",
      required: true,
    },
    
    {
      id: 2,
      name: "password",
      type: "password",
      placeholder: "Password",
      errorMessage:
        "Неправильный пароль",
      label: "Password",
    //   pattern: ,
      required: true,
    },

  ];

  const save_to_cookies = () => {
    Cookies.set('email', values["email"]);
    Cookies.set('password', values["password"]);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      await login(values["email"].toLowerCase(), values["password"])
      .then(data => {
        // аутентификации
        if (data["status"] == "OK") {
          alert("Авторизация успешна");
        }
        else {
          throw Error(data["reason"])
        }
      })
    }
    catch (error) {
      alert(error.message)
      return;
    }
    save_to_cookies();
    routeChange();
  };

  const onChange = (e) => {
    setValues({ ...values, [e.target.name]: e.target.value });
  };

 

  return (
    <div className="reg">
      <form onSubmit={handleSubmit}>
        <h1>Войти</h1>
        {inputs.map((input) => (
          <FormInput
            key={input.id}
            {...input}
            value={values[input.name]}
            onChange={onChange}
          />
        ))}
        <button className="register-button">Войти</button>
      </form>
    </div>
  );
};

export {Log_in};
