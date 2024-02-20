use std::{io::{self, Write}, time::Instant};
use clap::Parser;

#[derive (Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    sample: u16,
}


fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let mut deltas: Vec<u128> = Vec::new();
    for (i,_) in (0..args.sample).enumerate() {
        
        let now = Instant::now();
        printout(("Current test is number:".to_string() + i.to_string().as_str() + "\n").as_str().as_bytes())?;
        deltas.push(now.elapsed().as_micros());
    }
    let average_delta: u128 = deltas.iter().fold(1, |acc, x| acc + x) / deltas.len() as u128;
    println!("Average time between prints was: {average_delta}");
    Ok(())
}

fn printout(text: &[u8]) -> io::Result<()> {
    io::stdout().write_all(text)?;
    Ok(())
}


