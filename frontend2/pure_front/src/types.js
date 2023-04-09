class User {
    constructor(id, username, email, password, firstname=null, lastname=null, rating=1000, about=null, age=null, gender=null, last_online=null, reg_date=null) {
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

export {User};