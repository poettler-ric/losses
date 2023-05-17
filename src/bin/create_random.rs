use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let big_file = Path::new("big.csv");
    for _ in 1..=100_000 {
        losses::add(rand::random::<losses::Cause>(), big_file)?;
    }
    Ok(())
}
