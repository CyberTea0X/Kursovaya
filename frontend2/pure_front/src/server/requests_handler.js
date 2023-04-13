import { userProfile, get_tags, edit_tags } from "./requests.js"
import { User } from "../types.js"

function is_valid_tags(tags) {
    const regex = /^[a-zA-Z0-9]+(,[a-zA-Z0-9]+)*$/;
    // Регулярное выражение проверяет, что строка может содержать запятую и букву или цифру.
    // Это может повторяться неограниченное количество раз.
    const tagsArray = tags.split(",");
    const uniqueTags = new Set(tagsArray);
    return regex.test(tags) && tagsArray.length === uniqueTags.size;
}

async function getUserProfile(email) {
    const data = await userProfile(email);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let user = User.fromJson(data["user"]);
    user.email = email;
    return user;
}

async function getTagsArray(user_id, add_hstag=true) {
    const data = await get_tags(user_id);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let tags = data["tags"]
    if (add_hstag) {
        tags = tags.map(tag => `#${tag}`);
    }
    return tags;
}

async function editTagsFromStr(email, password, newtags) {
    newtags = newtags.replace(/ /g, '').replace(/#/g, '')
    if (!is_valid_tags(newtags)) {
        throw Error("Invalid tags");
    }
    const data = await edit_tags(email, password, newtags);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
}

export {getUserProfile, getTagsArray, editTagsFromStr}