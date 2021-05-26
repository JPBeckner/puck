use structopt::StructOpt;
mod puck;
pub use crate::puck::Puck;

#[derive(StructOpt)]
#[structopt(name = "Puck", about = "CLI to Punch the Clock")]
enum PuckCli {
    // time in or time out
    #[structopt(name = "punch", about = "Puch the clock and set the time that you are comming (in) ou going (out)")]
    Punch {
        #[structopt(name = "io", about = "In or Out", required = true)]
        io: String,
        #[structopt(name = "hour", about = "Hour to register", required = true)]
        hour: String,
        #[structopt(name = "user", about = "User to register in", required = false, default_value = "")]
        user: String,
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
        #[structopt(name = "default", about = "Set default user")]
        default: String,
    },
}

fn main() {
    match PuckCli::from_args() {
        PuckCli::Punch { io, hour, user } => {
            let mut puck = Puck::new().expect("init failed.");
            puck.register(io, hour, user);
        },
        PuckCli::Balance { year, month, day } => {
            println!("balance");
        },
        PuckCli::User { new, default } => {
            println!("user");
        },
        _ => ()
    }
}
