use gzlib::id::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gzid", about = "Gardenzilla ID checking and generator tool")]
enum Command {
    /// Pound acorns into flour for cookie dough.
    #[structopt(about = "(U64) Create new Luhn check-summed ID (HEX) from u64.")]
    Create { id: u64 },
    /// Add magical sparkles -- the secret ingredient!
    #[structopt(
        about = "(String) Check a given ID (HEX) if its correct HEX and Luhn check-summed."
    )]
    Check { id: String },
}

fn main() {
    let opt = Command::from_args();

    match opt {
        Command::Create { id } => println!("{}", generate_id(id, IdKind::LuhnTwo).to_hex()),
        Command::Check { id } => println!("{}", id.luhn_check().is_ok()),
    }
}
