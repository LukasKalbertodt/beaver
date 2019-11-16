use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short)]
    pub n: u8,
}
