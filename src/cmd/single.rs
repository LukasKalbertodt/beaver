use anyhow::Result;
use structopt::StructOpt;

use crate::{SharedArgs, tm::Tm};


#[derive(StructOpt, Debug, Clone)]
pub struct Args {
    #[structopt(flatten)]
    shared: SharedArgs,

    /// The ID of the turing machine.
    id: u64,
}

pub(crate) fn run(args: Args) -> Result<()> {
    match args.shared.n {
        1 => do_run::<1>(args),
        2 => do_run::<2>(args),
        3 => do_run::<3>(args),
        4 => do_run::<4>(args),
        5 => do_run::<5>(args),
        6 => do_run::<6>(args),
        _ => panic!("invalid value for n: argument parsing should catch this"),
    }
}

fn do_run<const N: usize>(args: Args) -> Result<()> {
    let tm = <Tm<N>>::from_id(args.id)
        .ok_or(anyhow::anyhow!("Turing machine ID is not valid for N = {}", N))?;

    bunt::println!("Turing machine for ID {[blue]}:", args.id);
    bunt::println!("{:#?}", tm);

    // TODO: run TM

    Ok(())
}
