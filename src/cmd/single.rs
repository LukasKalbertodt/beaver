use anyhow::Result;

use crate::{cli, tm::Tm};


pub(crate) fn run(id: u64, args: &cli::Args) -> Result<()> {
    match args.n {
        1 => do_run::<1>(id, args),
        2 => do_run::<2>(id, args),
        3 => do_run::<3>(id, args),
        4 => do_run::<4>(id, args),
        5 => do_run::<5>(id, args),
        6 => do_run::<6>(id, args),
        _ => panic!("invalid value for n: argument parsing should catch this"),
    }
}

fn do_run<const N: usize>(id: u64, _args: &cli::Args) -> Result<()> {
    let tm = <Tm<N>>::from_id(id)
        .ok_or(anyhow::anyhow!("Turing machine ID is not valid for N = {}", N))?;

    bunt::println!("Turing machine for ID {[blue]}:", id);
    bunt::println!("{:#?}", tm);

    // TODO: run TM

    Ok(())
}
