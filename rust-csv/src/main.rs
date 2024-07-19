use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        println!("{:?}", result);
    }

    // This means that the function executed successfully
    // You are returning an empty tuple (comes from the Result type)
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        // Error printing in rust (instead of println directive)
        eprintln!("{}", e);
    }
}
