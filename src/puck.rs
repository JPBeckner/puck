use std::collections::HashMap;
use chrono::{Datelike, Timelike, Utc, DateTime, NaiveTime};

pub struct Puck {
    data: Data
}

impl Puck {
    pub fn new() -> Result<Puck, std::io::Error> {
        println!("Create a new instance of puck");
        match Data::new() {
            Ok(data) => Ok(Puck { data }),
            Err(e) => panic!("Error: {}", e)
        }
    }

    pub fn register(&mut self, io: String, hour: String, user: String) {
        println!("io: {:?}, hour: {:?}", io, hour);
    }

}

pub struct Data {
    // user: String,
    map: HashMap<String, String>,
}

impl Data {
    pub fn new() -> Result<Data, std::io::Error> {
                // open db.json
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // serialize json as HashMap
        match serde_json::from_reader(file) {
            Ok(map) => Ok(Data { map }),
            Err(e) if e.is_eof() => Ok(Data {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),

        }
    }
}
