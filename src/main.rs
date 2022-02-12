#![feature(adt_const_params)]
#![allow(incomplete_features)]

use structopt::StructOpt;

mod analyze;
mod cmd;
mod gen;
mod outcome;
mod tape;
mod tm;


fn main() {
    let args = Args::from_args();

    let res = match args {
        Args::Single(args) => cmd::single::run(args),
        Args::Full(args) => cmd::full::run(args),
    };

    if let Err(e) = res {
        bunt::eprintln!("{$red}An error occured!{/$}");
        eprintln!("{:?}", e);
    }
}

/// Simple program to play the Busy Beaver game. That is: to simulate Turing
/// machines (TMs) with N states on a binary tape (each cell is either 0 or 1,
/// with all being initialized to 0). TMs up to N=6 are supported.
#[derive(StructOpt, Debug, Clone)]
pub enum Args {
    /// Shows information about and runs a single Turing machine, specified by
    /// its ID.
    Single(cmd::single::Args),

    /// Analyzes the full class of TMs with N states.
    Full(cmd::full::Args),
}

#[derive(StructOpt, Debug, Clone)]
pub struct SharedArgs {
    /// Number of states of the Turing machine.
    #[structopt(short, possible_values(&["1", "2", "3", "4", "5", "6"]))]
    pub n: u8,

    /// Number of steps after which TMs are stopped.
    #[structopt(long, default_value = "200", global = true)]
    pub max_steps: u32,
}

