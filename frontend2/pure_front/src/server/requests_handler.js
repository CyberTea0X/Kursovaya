import { userProfile, get_tags, get_many_tags, edit_tags, gallery, ip, port } from "./requests.js"
import { User, Image } from "../types.js"


async function getImages(user_id) {

    const data = await gallery(user_id);
    if (data.status !== "OK") {
      throw Error(data.reason);
    }
    const images = data["images"].map((image) => {
      image = Image.fromJson(image);
      image.setUrl(ip, port, user_id);
      return image;
    });
    return images;
}


function is_valid_tags(tags) {
    const regex = /^[a-zA-Z0-9]+(,[a-zA-Z0-9]+)*$/;
    // Регулярное выражение проверяет, что строка может содержать запятую и букву или цифру.
    // Это может повторяться неограниченное количество раз.
    const tagsArray = tags.split(",");
    const uniqueTags = new Set(tagsArray);
    return regex.test(tags) && tagsArray.length === uniqueTags.size;
}


async function getUserProfile(idOrEmail) {
    const data = await userProfile(idOrEmail);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let user = User.fromJson(data["user"]);
    if (isNaN(parseInt(idOrEmail))) {
        user.email = idOrEmail;
    }
    return user;
}

async function getManyTagsArray(range, add_hstag=true, sep=", ") {
    const data = await get_many_tags(range);
    if (data["status"] !== "OK") {
        throw Error(data["reason"]);
    }
    let items = data["items"]
    if (add_hstag) {
        items = items.map(tags => [tags[0], tags[1].map(tag => `#${tag}`).join(sep)]);
    } 
    return items;
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

export {getUserProfile, getTagsArray, getManyTagsArray, editTagsFromStr, getImages}