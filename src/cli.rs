use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short)]
    pub n: u8,

    #[structopt(long, default_value = "100")]
    pub max_steps: u64,

    #[structopt(long, default_value = "100")]
    pub max_cells: u64,
}
