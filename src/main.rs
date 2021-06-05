use structopt::StructOpt;
mod puck;
pub use crate::puck::Puck;

#[derive(StructOpt)]
#[structopt(name = "Puck", about = "CLI to Punch the Clock")]
enum PuckCli {
    // time in or time out
    #[structopt(
        name = "punch",
        about = "Puch the clock and set the time that you are comming (in) ou going (out)"
    )]
    Punch {
        #[structopt(name = "io", about = "In or Out", required = true)]
        io: String,
        
        #[structopt(
            name = "hour",
            about = "Hour to register",
            required = false,
            default_value = ""
        )]
        hour: String,
        
        #[structopt(
            name = "date",
            long = "--date",
            short = "-d",
            about = "date to register in",
            required = false,
            default_value = ""
        )]
        date: String,
    },

    #[structopt(name = "balance", about = "Balance of hours")]
    Balance {
        #[structopt(name = "year")]
        year: String,
        #[structopt(name = "month")]
        month: String,
        #[structopt(name = "day")]
        day: String,
    },
    #[structopt(name = "user", about = "User :0")]
    User {
        #[structopt(
            name = "new",
            about = "Register a new user",
            long = "new",
            required = false,
            default_value = "",
        )]
        new: String,

        #[structopt(
            name = "delete",
            about = "Delete the user data.",
            short = "-d",
            long = "--delete"
        )]
        delete: bool,
    },
}

fn main() {
    let mut puck = Puck;
    match PuckCli::from_args() {
        PuckCli::Punch { io, hour, date } => {
            
            puck.register(io, hour, date);
        },
        PuckCli::Balance { year, month, day } => {
            println!("balance for year: {:?}, month: {:?}, and a day: {:?}", year, month, day);
        },
        PuckCli::User { new, delete } => {
            if delete {
                puck.clear_user();
                return
            }
            println!("new user: {:?}", new);
            if !new.is_empty() {
                puck.set_user(new);
                return
            }
            println!("new is empty");

            println!("Curent user: {:?}", puck.get_user().unwrap());
        },
        // _ => ()  // should handle that?
    }
}
