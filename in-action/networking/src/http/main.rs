use futures::executor::block_on;

/// Using the reqwest library
fn http() -> Result<(), Box<dyn std::error::Error>> {
    let content = block_on(reqwest::get("https://www.antoniocaggiano.eu"))?;
    let content = block_on(content.text())?;
    for line in content.split('\n') {
        println!("{}", line);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    http()?;
    Ok(())
}
