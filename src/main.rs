mod bign;
pub use bign::*;

use std::io::Write;

fn parse_args(args: &Vec<String>) -> Result<u32 , std::num::ParseIntError>
{
    args[0].parse::<u32>()
}

fn main()
{
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1
    {
        writeln!(std::io::stderr(), "Invalid parameters!").unwrap();
        std::process::exit(1);
    }
    //calculate_factorial();
    let number = parse_args(&args);
    match number
    {
        Ok(x) => calculate_factorial(x),
        Err(err) =>
        {
            writeln!(std::io::stderr(), "{}", err).unwrap();
            std::process::exit(1);
        },
    }
    println!("{:?}", parse_args(&args));
}
