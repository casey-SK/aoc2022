use color_eyre::eyre::Context;

use clap::Parser;

use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input filename
   #[arg(short, long)]
   path: String,
}


fn main() -> color_eyre::Result<()> {
    // Install panic hooks
    color_eyre::install()?;

    // Get CLI arguments
    let args = Args::parse();

    // Create a path to the desired file
    let path = Path::new(&args.path);
    let display = path.display();

    let file = File::open(path).wrap_err(format!("reading {:?}", display))?;

    let reader = BufReader::new(file);

    let mut err = Ok(());
    let mut x = reader
        .lines() // iterate over each line in the file
        .scan(&mut err, until_err) // panic if an error is found later on
        .group_by(|vi| !vi.is_empty()) // group elements together, using the empty lines as delimiter
        .into_iter()
        .filter_map(|(filt, group)| // convert text to u32 and sum each group
            if filt {
                Some(group
                    .into_iter()
                    .map(|x| x.parse::<u32>().wrap_err(format!("{:?}", x)).unwrap())
                    .sum::<u32>())
            } else { None })
        .into_iter()
        .collect::<Vec<_>>();

    err?;

    x.sort_by(|a, b| a.cmp(b).reverse()); // sort the sums
    x.truncate(3); // we only care about the top three

    println!("Top Elf: {:?}", x[0]);

    println!("Top Three Elves: {:?}", x.iter().sum::<u32>());

    Ok(())
}

/// Used in an iterator to prevent silent errors
fn until_err<T, E>(err: &mut &mut Result<(), E>, item: Result<T, E>) -> Option<T> {
    match item {
        Ok(item) => Some(item),
        Err(e) => {
            **err = Err(e);
            None
        }
    }
}
