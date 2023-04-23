import { useState } from "react";
import "./log_in.css";
import FormInput from "../sign-up/FormInput";
import { useNavigate } from 'react-router-dom';
import { login } from "../../server/requests";
import { getUserProfile } from "../../server/requests_handler";
import { userToCookies, removeOldCookies} from "../../cookies";


async function authentificate(email, password) {
  const data = await login(email, password);
  if (data["status"] === "OK") {
    alert("Авторизация успешна");
  } else {
    throw Error(data["reason"]);
  }
}
const Login = () => {
  const [values, setValues] = useState({
    email: "",
    password: "",
    id: "",
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

  const handleSubmit = async (e) => {
    e.preventDefault();
    let user;
    try {
      user = await getUserProfile(values["email"].toLowerCase());
      await authentificate(values["email"].toLowerCase(), values["password"]);
    }
    catch (error) {
      alert(error.message)
      return;
    }
    removeOldCookies();
    userToCookies(values["email"].toLowerCase(), values["password"], user.id);
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
        <p className="register-link">
        Ещё не зарегистрированы?{" "}
        <a href="/Signup">Регистрация</a>
      </p>
      </form>
    </div>
  );
};

export { Login };
