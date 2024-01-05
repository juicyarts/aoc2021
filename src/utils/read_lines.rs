use std::fs::{File, self};
use std::io::{self, BufRead };
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file<P>(filename: P) -> io::Result<String>
where P: AsRef<Path>,
{
    let file = fs::read_to_string(filename)?;
    Ok(file)
}
