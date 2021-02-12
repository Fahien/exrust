/// Using the reqwest library
fn http() -> Result<(), Box<dyn std::error::Error>> {
    let content = reqwest::get("https://www.antoniocaggiano.eu")?.text()?;
    for line in content.split('\n') {
        println!("{}", line);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    http()?;
    Ok(())
}
