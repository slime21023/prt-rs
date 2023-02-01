use clap::{arg, value_parser, ArgAction, Command};
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
use std::fs::File;


fn print_file(path: PathBuf, with_num: bool, with_end: bool) -> io::Result<()> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for (index, line) in reader.lines().enumerate() {
        if with_num && with_end {
            let id = index + 1;
            println!("{id}: {}$", line?);
        } else if with_num {
            let id = index + 1;
            println!("{id}: {}", line?);
        } else if with_end {
            println!("{}$", line?);
        } else {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn main() {
    let matches = Command::new("prt")
        .about("Simple program to print the files")
        .arg(
            arg!([FILE] "the files")
                .required(true)
                .value_parser(value_parser!(PathBuf))
                .action(ArgAction::Append)
        )
        .arg(
            arg!(-e --end r#"print with the end("$")"#)
                .required(false)
        )
        .arg(
            arg!(-n --num r#"print with the line number"#)
                .required(false)
        )
        .get_matches();

    let files = matches
        .get_many::<PathBuf>("FILE")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    
    let with_end = matches.get_flag("end");
    let with_num = matches.get_flag("num");
    

    for f in files {
        match print_file(f.to_path_buf(), with_num, with_end) {
            Ok(_) => {
                println!();
            }
            err => { eprintln!("{:?}", err); }
        }
    }

}