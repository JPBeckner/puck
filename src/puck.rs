use std::collections::HashMap;
use chrono::{Datelike, Timelike, Local};
use serde::{Deserialize, Serialize};
use serde_json;


pub struct Puck;

impl Puck {

    pub fn register(&mut self, io: String, mut hour: String, mut date: String) {
        let mut data = UserData::new().expect("Erro");

        let now = Local::now();

        if hour.eq("") {
            hour.insert_str(
                0,
                format!(
                    "{:02}:{:02}", now.hour(), now.minute()
                ).as_str()
            );
        }

        if date.eq(&"".to_string()) {
            date.insert_str(
                0,
                format!(
                    "{:02}-{:02}-{:?}", now.day(), now.month(), now.year()
                ).as_str()
            );
        }
        data.registers.entry(date.clone()).or_insert(HashMap::new()).entry(hour).or_insert(io);

        println!("Full data to register: {:?}", data.registers);
        data.save().expect("Can't save the file.");
    }

    pub fn get_user(&mut self) -> Result<String, std::io::Error> {
        match UserData::new() {
            Ok(data) => {
                let user = data.user;
                Ok(user)
            }
            _ => {
                panic!("Can't get the current user.");
            }
        }
    }

    pub fn set_user(&mut self, new: String) {
        println!("Puck is registring a new user or cleaning the actual user data.");
        let mut data = UserData::new().expect("Erro");
        
        data.user = new;
        data.save().expect("Can't save the file.");

    }

    pub fn clear_user(&mut self) {
        let mut data = UserData::new().expect("Error");
        data.user.clear();
        data.registers.clear();
        println!("cleared registers: {:?}", data.registers);
        data.save().expect("Can't clear the registers");
    }

}

#[derive(Deserialize, Serialize)]
struct UserData {
    user: String,
    registers: HashMap<String, HashMap<String, String>>
}

impl UserData {
    fn new() -> Result<UserData, std::io::Error> {
        // open redorded data file, or create one if there is no file yet
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(".record_storage.json")?;

        let full_data: UserData = serde_json::from_reader(file).unwrap();
        Ok(full_data)

    }

    fn save(&mut self) -> Result<(), std::io::Error> {
        println!("Writing on file.");
        let file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(".record_storage.json")?;
        
        // write to file with actual data.
        match serde_json::to_writer_pretty(file, &self) {
            Ok(()) => { Ok(()) },
            _ => { panic!("Can save file."); }
        }        
    }
}
