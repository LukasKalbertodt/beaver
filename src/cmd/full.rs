use anyhow::Result;

use crate::{cli, tm::Tm, gen::{All, TmGenerator}};


pub(crate) fn run(args: &cli::Args) -> Result<()> {
    match args.n {
        1 => do_run::<All<1>, 1>(args),
        2 => do_run::<All<2>, 2>(args),
        3 => do_run::<All<3>, 3>(args),
        4 => do_run::<All<4>, 4>(args),
        5 => do_run::<All<5>, 5>(args),
        6 => do_run::<All<6>, 6>(args),
        _ => panic!("invalid value for n: argument parsing should catch this"),
    }

    Ok(())
}

fn do_run<G: TmGenerator<N>, const N: usize>(_args: &cli::Args) {
    G::for_all(|tm| {
        bunt::println!("{$green}{:?}{/$}", tm);
    });
}
