use asciigen;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = asciigen::asciigen::Args::parse();

    asciigen::asciigen::convert(args)?;
    Ok(())
}
