use std::collections::HashMap;
// use std::marker::Copy;
use chrono::{Datelike, Timelike, Utc, DateTime, NaiveTime};
use serde::{Deserialize};
// use serde_json::Result;


/*
ideia: match get para cara nivel do json, e cada nivel popula uma variavel diferente.
no final retorna Puck { user, ...}
*/
/*
mudar estrutura de dados:
{
    "username": {
        "28-04-2021": {
            "10:30": "in"
        }
    }
}
*/

//pub struct Puck {
//    user: User
//}
pub struct Puck;

impl Puck {
    // pub fn new() -> Result<Puck, std::io::Error> {
    //     println!("Create a new instance of puck");
    //     match Data::new() {
    //         Ok(data) => Ok(Puck { user:data.get_default_user() }),
    //         Err(e) => panic!("Error: {}", e)
    //     }
    // }

    pub fn register(&mut self, io: String, hour: String, user: String) {
        let mut data = FullData::new().expect("Erro");
        println!("io: {:?}, hour: {:?}, sera? {:?}", io, hour, data.users.get("beck").unwrap().get("01-06-2021"));
    }

}

// usuario -> ano -> mes -> dia -> hora -> in/out
// #[derive(Copy, Clone)]
struct UserData {
    // default: String,
    user: String,
    registers: HashMap<String, IOHour>,
    // HashMap<String, String>
    // {
    //     "default": "default value",
    //     "users": {
    //         "user_name": {
    //             "01-06-2021": {
    //                 "9:30": "in",
    //                 "12:00": "out",
    //             }
    //         }
    //     }
    // }
    /*{
        "name": {
            "value": {
                "some": "thing"
            }
        }
    }*/
    // user: HashMap<String, String>,
    // total_user_data: HashMap<String, user>,
    // ano: HashMap<String, String>,
    // mes: HashMap<String, String>,
    // dia: HashMap<String, String>,
    // hora: HashMap<String, String>,
    // user: HashMap<String, HashMap<String, HashMap<String, HashMap<String, HashMap<String, String>>>>>
}

#[derive(Deserialize)]
struct IOHour {
    io_hour: HashMap<String, String>
}

#[derive(Deserialize)]
struct UserTimeData {
    time_data: HashMap<String, IOHour>
}

#[derive(Deserialize)]
struct FullData {
    default: String,
    users: HashMap<String, HashMap<String, HashMap<String, String>>>
}

// struct User {
//     user: HashMap<String, Ano>
// }

// struct Ano {
//     ano: HashMap<String, Mes>
// }

// struct Mes {
//     mes: HashMap<String, Dia>
// }

// struct Dia {
//     dia: HashMap<String, Hora>
// }

// struct Hora {
//     hora: HashMap<String, String>
// }

impl FullData {
    fn new() -> Result<FullData, std::io::Error> {
        // open db.json
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(".db.json")?;
        // serialize json as HashMap
        // let data = serde_json::from_reader(file)?;
        // Ok(data)

        let full_data: FullData = serde_json::from_reader(file).unwrap();
        // let default_user = full_data.default;
        // let user_registers: HashMap<String, IOHour> = full_data.users.get(&default_user).unwrap().time_data;
        // let user = UserData { user: default_user.to_string(), registers: user_registers};
        Ok(full_data)

        // match serde_json::from_reader::<_, serde_json::Value>(file) {
        //     Ok(value) => {
        //         // let data = Data { value };
        //         let default: String = value.get("default").map(|v| v.to_string()).unwrap_or_default();
        //         // match map.get::<String>("default".to_string()) {
        //         //     Some(value) => default = value.to_string(),
        //         //     None => default = String::from(""),
        //         // }
        //         // let users_avaliable: HashMap<String, HashMap<String, String>;
        //         // let user = default;
        //         // match map.get("users") {
        //         //     Some(value) => users_avaliable = HashMap::new(),
        //         //     None => users_avaliable = HashMap::new()
        //         // }
        //         let registers_from_file: HashMap<String, HashMap<String, String>> = value.get(user.to_string());  // ??
        //         let t = value.
        //         let data = Data { user: user, registers:  registers_from_file};
        //         Ok(data)
        //     },
        //     // Err(e) if e.is_eof() => Ok(Data {
        //     //     default: String::new(),
        //     //     users: User { user: HashMap::new() },
        //         // <String, Ano { ano: HashMap<String, Mes { mes: HashMap<String, Dia { dia: HashMap<String, Hora { hora: HashMap<String, String>}>}> }>}>::new()
        //         //String::new(), Ano { String::new(), Mes { String::new(), Dia { String::new(), Hora { String::new(), String::new() } } } }
        //         // ano: HashMap::new(),
        //         // mes: HashMap::new(),
        //         // dia: HashMap::new(),
        //         // hora: HashMap::new()
        //     //}),
        //     Err(e) => panic!("An error occurred: {}", e),

        // }
    }

    // fn get_default_user(&mut self) -> User { // Result<User, std::io::Error>
    //     match &self.users.user {
    //         users_avaliable => {
    //             User {user: users_avaliable}
    //         },
    //         //Err(e) => panic!("An error occured: {}", e),
    //     }
    // }

    fn add(&mut self, key: String, value: String) {
        println!("Writing on file.");
    }
}
