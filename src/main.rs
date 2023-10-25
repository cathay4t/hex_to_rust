// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remove_prefix = regex::Regex::new("[0-9]+ +")?;

    for (i, line) in std::io::stdin().lines().enumerate() {
        let line = line?;
        let line = remove_prefix.replace(&line, "");
        if line.len() == 0 {
            continue;
        }
        if i != 0 {
            print!(",\n");
        }
        print!("{}", if i == 0 { "[" } else { " " });
        for (j, hex_str) in line.split(" ").enumerate() {
            if j != 0 {
                print!(", ");
            }
            print!("0x{hex_str}");
        }
    }
    println!("]");
    Ok(())
}