import config from '../config.json';


async function registerUser(config, login, email, password, firstname, lastname, age, gender, about) {
    const url = `http://${config.serverIp}:${config.serverPort}/api/user/register/${login}/${email}/${password}`;
    const response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            login,
            email,
            password,
            firstname,
            lastname,
            age,
            gender,
            about
        })
    });
    return response.json();
}

export { registerUser };