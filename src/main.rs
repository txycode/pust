use basic::{init, run};
use clap::Parser;
use std::{fs::File, io, io::BufRead, path::Path};
mod basic;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // file path
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Args::parse();
    let p = args.path.as_path().display().to_string();
    let mut interpreter = init();
    if let Ok(lines) = read_lines(p) {
        for line in lines {
            if let Ok(text) = line {
                run(
                    args.path.as_path().display().to_string(),
                    text,
                    &mut interpreter,
                );
            }
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
