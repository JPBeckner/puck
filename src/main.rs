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
            about = "date to register in",
            required = false,
            default_value = ""
        )]
        date: String,
    },

    // time to register
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
        #[structopt(name = "new", about = "Register new user")]
        new: String,

        #[structopt(
            name = "refresh",
            about = "Refresh the user data.",
            long = "--clear",
            required = false,
            default_value = "false",
        )]
        clear: String,
        // #[structopt(name = "default", about = "Set default user")]
        // default: String,
    },
}

fn main() {
    let b = false;
    match PuckCli::from_args() {
        PuckCli::Punch { io, hour, date } => {
            let mut puck = Puck;
            puck.register(io, hour, date);
        },
        PuckCli::Balance { year, month, day } => {
            println!("balance");
        },
        PuckCli::User { new, clear } => {
            println!("user");
        },
        _ => ()
    }
}
