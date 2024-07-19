use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    // Should not add semicolon for implicit return
    Ok(())
}

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        // Error printing in rust (instead of println directive)
        eprintln!("{}", e);
    }
}
