extern crate csv;

use std::env;
use std::error::Error;
use std::io;
use std::process;

fn run(
    args : Vec<String>,
    source : &mut dyn io::Read,
    target : &mut dyn io::Write
) -> Result<(), Box<dyn Error>> {
    let query = match args.first() {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };
    let mut rdr = csv::Reader::from_reader(source);
    let mut wtr = csv::Writer::from_writer(target);
    wtr.write_record(rdr.headers()?)?;
    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == *query) {
            wtr.write_record(&record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    args.drain(0..1);  // Remove program path
    let mut source = io::stdin();
    let mut target = io::stdout();
    if let Err(err) = run(args, &mut source, &mut target) {
        println!("{}", err);
        process::exit(1);
    }
}
