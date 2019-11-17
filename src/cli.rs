use structopt::StructOpt;


/// Simple program to play the Busy Beaver game. That is: to simulate Turing
/// machines (TMs) with N states on a binary tape (each cell is either 0 or 1,
/// with all being initialized to 0).
#[derive(StructOpt, Debug, Clone)]
pub struct Args {
    /// `N` is the number of states of the TMs.
    #[structopt(short)]
    pub n: u8,

    /// Number of steps after which TMs are stopped.
    #[structopt(long, default_value = "200")]
    pub max_steps: u64,

    /// If specified, the progress bar is not shown.
    #[structopt(long)]
    pub no_pb: bool,

    /// If specified, TMs that are aborted after `max-steps` will be printed.
    /// You probably want to define this with `--no-pb`.
    #[structopt(long)]
    pub print_aborted: bool,

    /// Number of threads to use. Defaults to the number of virtual CPUs.
    #[structopt(long, short = "j")]
    pub num_threads: Option<u32>,
}
