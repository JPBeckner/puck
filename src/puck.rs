use std::collections::HashMap;
use chrono::{Datelike, Timelike, Utc, DateTime, NaiveTime};

/*
ideia: match get para cara nivel do json, e cada nivel popula uma variavel diferente.
no final retorna Puck { user, ...}
*/

pub struct Puck {
    user: User
}

impl Puck {
    pub fn new() -> Result<Puck, std::io::Error> {
        println!("Create a new instance of puck");
        match Data::new() {
            Ok(data) => Ok(Puck { user:data.get_default_user() }),
            Err(e) => panic!("Error: {}", e)
        }
    }

    pub fn register(&mut self, io: String, hour: String, user: String) {
        println!("io: {:?}, hour: {:?}", io, hour);
    }

}

// usuario -> ano -> mes -> dia -> hora -> in/out
struct Data {
    default: String,
    users: User,
    // user: HashMap<String, String>,
    // total_user_data: HashMap<String, user>,
    // ano: HashMap<String, String>,
    // mes: HashMap<String, String>,
    // dia: HashMap<String, String>,
    // hora: HashMap<String, String>,
    // user: HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, String>>>>>
}

struct User {
    user: HashMap<String, Ano>
}

struct Ano {
    ano: HashMap<String, Mes>
}

struct Mes {
    mes: HashMap<String, Dia>
}

struct Dia {
    dia: HashMap<String, Hora>
}

struct Hora {
    hora: HashMap<String, String>
}

impl Data {
    fn new() -> Result<Data, std::io::Error> {
        // open db.json
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize json as HashMap
        match serde_json::from_reader::<_, HashMap<String, String>>(file) {
            Ok(map) => {
                let default: String;
                match map.get("default") {
                    Some(value) => default = String::from(value),
                    None => default = String::from(""),
                }
                let users_avaliable: HashMap<String, Ano>;
                match map.get("users") {
                    Some(value) => users_avaliable = HashMap::new(),
                    None => users_avaliable = HashMap::new()
                }
                let data = Data { default: default, users: User { user: users_avaliable } };
                Ok(data)
            },
            Err(e) if e.is_eof() => Ok(Data {
                default: String::new(),
                users: User { user: HashMap::new() },
                // <String, Ano { ano: HashMap<String, Mes { mes: HashMap<String, Dia { dia: HashMap<String, Hora { hora: HashMap<String, String>}>}> }>}>::new()
                //String::new(), Ano { String::new(), Mes { String::new(), Dia { String::new(), Hora { String::new(), String::new() } } } }
                // ano: HashMap::new(),
                // mes: HashMap::new(),
                // dia: HashMap::new(),
                // hora: HashMap::new()
            }),
            Err(e) => panic!("An error occurred: {}", e),

        }
    }

    fn get_default_user(&mut self) -> User { // Result<User, std::io::Error>
        match &self.users.user {
            users_avaliable => {
                User {user: users_avaliable}
            },
            //Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn add(&mut self, key: String, value: String) {
        println!("Writing on file.");
    }
}
