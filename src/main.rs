#![feature(const_generics, const_generic_impls_guard)]
#![allow(incomplete_features)] // I know const generics is still buggy...

use std::{
    array::LengthAtMost32,
};

use structopt::StructOpt;

use crate::{
    cli::Args,
    tape::Tape,
    tm::{Action, Move, State, Tm, gen_all_tms},
};


mod cli;
mod tape;
mod tm;





/// The index of a cell. All TM start with `0` as the active cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CellId(i64);

/// The binary value of a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellValue(bool);




fn main() {
    let args = Args::from_args();

    match args.n {
        0 => {
            eprintln!("`-n` must be above 0! Zero state TMs do not make sense.");
            return;
        }
        1 => run::<1>(),
        2 => run::<2>(),
        3 => run::<3>(),
        4 => run::<4>(),
        _ => {
            eprintln!(
                "Currently, only `-n` up to 4 are allowed. This whole problem \
                    is about quickly growing functions, you know..."
            );
            return;
        }
    }
}

fn run<const N: usize>()
where
    [State; N]: LengthAtMost32,
{
    println!("▸ Generating all possible TMs with {} states...", N);
    let tms = gen_all_tms::<{N}>();
    println!("  ... generated {} TMs", tms.len());
}
