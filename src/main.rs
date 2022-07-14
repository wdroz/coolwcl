use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = vec![];
    io::stdin().read_to_end(&mut buffer)?;
    let mut cpt = 0;
    for _ in std::str::from_utf8(&buffer).unwrap().split("\n") {
        cpt += 1;
        let cpt_str = cpt.to_string();
        print!("{}{}", cpt, "\r".repeat(cpt_str.len()));
    }
    println!();
    Ok(())
}
