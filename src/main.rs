use asciigen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let filename = String::from("img.png");

    asciigen::asciigen::convert(&filename)?;
    Ok(())
}
