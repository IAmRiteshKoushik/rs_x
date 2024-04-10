extern crate flate2;

use flate2::write::GzEncoder;   // Converts Unicode stream into bytes
use flate2::Compression;        // Handles the actual compression
use std::env::args;             // Accept filename from console
use std::fs::File;              // Access the file system
use std::io::copy;              // Copy files
use std::io::BufReader;         // Read buffers while copying
use std::time::Instant;         // Display the time to compress

fn main() {
    if args().len() != 3 {
        // Expects three arguments
        // If three arguments are not supplied, then return the function
        eprintln!("Usage: `source` `target` ");
        return;
    } 
    // Takes in the input, finds the file and reads the content into a buffer
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // Creates an output file using the 2nd argument supplied to the function
    let output = File::create(args().nth(2).unwrap()).unwrap();
    // Creates an encoder which 
    let mut encoder = GzEncoder::new(output, Compression::default());
    // Starts a stop-watch
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    // Length of source file
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    // Length of result file
    println!("Target len: {:?}", output.metadata().unwrap().len());
    // Checkpoint - checks elapsed time on the stop-watch
    println!("Elapsed time {:?}", start.elapsed());
}


