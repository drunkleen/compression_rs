use flate2::bufread::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    if args().len() != 4 {
        eprintln!("Usage: <method: compress|decompress> <source> <output>");
        return Err("Invalid arguments".into());
    }

    let method = args().nth(1).unwrap();
    let source = args().nth(2).unwrap();
    let output = args().nth(3).unwrap();

    match method.as_str() {
        "-c" => compress(&source, &output)?,
        "-d" => decompress(&source, &output)?,
        _ => {
            eprintln!("Invalid method");
            return Err("Invalid method".into());
        }
    }

    Ok(())
}

fn decompress(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_file)?;
    let output_file = File::create(output_file)?;

    let input = BufReader::new(input_file);
    let mut output = BufWriter::new(output_file);

    let mut decoder = GzDecoder::new(input);
    let start = Instant::now();
    copy(&mut decoder, &mut output)?;
    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}

fn compress(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let input_file = File::open(input_file)?;
    let output_file = File::create(output_file)?;

    let input = BufReader::new(input_file);
    let mut output = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(input, Compression::default());
    let start = Instant::now();
    copy(&mut encoder, &mut output)?;

    println!(
        "Source len: {:?}",
        encoder.get_ref().get_ref().metadata()?.len()
    );
    println!("Output len: {:?}", output.get_ref().metadata()?.len());
    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}
