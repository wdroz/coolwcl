use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut cpt = 0;
    for line in stdin.lock().lines() {
        match line {
            Err(_) => break, // with ^Z
            Ok(_) => {
                cpt += 1;
                let cpt_str = cpt.to_string();
                print!("{}{}", cpt, "\r".repeat(cpt_str.len()));
            }
        }
    }
    println!();
    Ok(())
}
