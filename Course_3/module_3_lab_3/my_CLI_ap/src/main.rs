use clap::Parser;
use std::fs::File;
use std::io;
#[derive(Parser, Debug)]
#[clap(author, version, about)]

struct Args {
    #[clap(short,long)]
    input: String,

}

fn main() -> io::Result<()>{
    println!("Hello, world!");
    let args = Args::parse();


    let file =  File::open(&args.input)?;

    println!("File opened successfully: {:?}", file);
    Ok(())
}

/*
fn main() -> io::Result<()> {
    println!("Hello, world!");

    let args = Args::parse();

    // Try to open the file
    match File::open(&args.input) {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => eprintln!("Failed to open file '{}': {}", args.input, e),
    }

    Ok(())
}
*/