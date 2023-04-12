import Cookies from 'js-cookie';


function userToCookies(email, password, id = null) {
    Cookies.set("email", email);
    Cookies.set("password", password);
    if (id !== null) {
    Cookies.set("id", id);
    }
}
    
function removeOldCookies() {
    Cookies.remove("id");
}

export { userToCookies, removeOldCookies };