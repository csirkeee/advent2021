use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut v: Vec<i32> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(i) = line?.parse() {
            v.push(i);
        }
    }

    let mut last_depth = v[0] + v[1] + v[2];
    let mut incs = 0;

    for i in 0..v.len() - 2 {
        let depth = v[i] + v[i + 1] + v[i + 2];
        if depth > last_depth {
            incs += 1;
        }
        last_depth = depth;
    }

    println!("{}", incs);
    return Ok(());
}
