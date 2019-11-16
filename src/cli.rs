use structopt::StructOpt;


/// Simple program to play the Busy Beaver game. That is: to simulate Turing
/// machines (TMs) with N states on a binary tape (each cell is either 0 or 1,
/// with all being initialized to 0).
#[derive(StructOpt, Debug)]
pub struct Args {
    /// `N` is the number of states of the TMs.
    #[structopt(short)]
    pub n: u8,

    /// Number of steps after which TMs are stopped.
    #[structopt(long, default_value = "200")]
    pub max_steps: u64,
}
