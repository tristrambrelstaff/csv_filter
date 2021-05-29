use std::env;
use std::error::Error;
use std::io;
use std::process;

fn run(
    args : Vec<String>,
    src : &mut dyn io::Read,
    tgt : &mut dyn io::Write
) -> Result<(), Box<dyn Error>> {
    let query = match args.first() {
        None => return Err(From::from("Program must be called with 1 argument")),
        Some(query) => query,
    };
    let mut rdr = csv::Reader::from_reader(src);
    let mut wtr = csv::Writer::from_writer(tgt);
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
    let mut src = io::stdin();
    let mut tgt = io::stdout();
    if let Err(err) = run(args, &mut src, &mut tgt) {
        println!("ERROR: {}", err);
        process::exit(1);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pred_eq() {
        let args = vec!["a".to_string()];
        let mut src = "a,b,c\n1,2,3\n4,a,6\n7,8,9".as_bytes();
        let mut tgt : Vec<u8> = Vec::new();
        if let Err(err) = run(args, &mut src, &mut tgt) {
            println!("ERROR: {}", err);
        } else {
            assert_eq!(tgt, "a,b,c\n4,a,6\n".as_bytes().to_vec());
	}
    }

}
