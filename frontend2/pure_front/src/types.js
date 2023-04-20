class User {
    constructor(id, username, email, password, firstname, lastname, rating, about, age, gender, last_online, reg_date) {
      this.id = id;
      this.username = username;
      this.email = email;
      this.password = password;
      this.firstname = firstname;
      this.lastname = lastname;
      this.rating = rating;
      this.about = about;
      this.age = age;
      this.gender = gender;
      this.last_online = last_online;
      this.reg_date = reg_date;
    }
  
    static fromJson(jsonData) {
      const { id=null, username=null, email=null, password=null, firstname=null, lastname=null, rating=null, about=null, age=null, gender=null, last_online=null, reg_date=null } = jsonData;
      return new User(id, username, email, password, firstname, lastname, rating, about, age, gender, last_online, reg_date);
    }

    toQuery(excludeRating=true) {
        let query = "";
        for (const key in this) {
            if (this[key] !== null && typeof this[key] !== "function") { // проверяем, что ключ не равен excludeRating
                if (key === "rating" && excludeRating) { // если ключ равен rating и excludeRating равен true, то пропускаем
                    continue;
                }
                query += `${key}=${this[key]}&`;
            }
        }
        return query.slice(0, -1);
    }

    static emptyUser() {
      return new User();
    }

    clone(attributesToChange={}) {
        const { id=this.id, username=this.username, email=this.email, password=this.password, firstname=this.firstname, lastname=this.lastname, rating=this.rating, about=this.about, age=this.age, gender=this.gender, last_online=this.last_online, reg_date=this.reg_date } = attributesToChange;
        return new User(id, username, email, password, firstname, lastname, rating, about, age, gender, last_online, reg_date);
    }
}

class Image {
  constructor(id, owner_id, published_at, about, image_name, extension, tags, views, likes) {
    this.id = id;
    this.owner_id = owner_id;
    this.published_at = published_at;
    this.about = about;
    this.image_name = image_name;
    this.extension = extension;
    this.tags = tags;
    this.views = views;
    this.likes = likes;
    this.url = null
  }


  setUrl(ip, port) {
    this.url = `http://${ip}:${port}/api/images/${this.owner_id}/gallery/${this.id}.${this.extension}`
  }

  withUrl(ip, port) {
    this.setUrl(ip, port)
    return this;
  }

  static fromJson(json) {
    const image = new Image(
      json.id,
      json.owner_id,
      new Date(json.published_at),
      json.about,
      json.image_name,
      json.extension,
      json.tags,
      json.views,
      json.likes
    );
    return image;
  }
}



export {User, Image};