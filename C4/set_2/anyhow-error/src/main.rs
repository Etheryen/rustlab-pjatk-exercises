use std::fs;

use anyhow::Context;

fn read_file(path: &str) -> anyhow::Result<String> {
    let content = fs::read_to_string(path).context("reading file failed")?;

    Ok(content)
}

fn main() -> anyhow::Result<()> {
    let file = read_file("Cargo.oml").context("couldn't read the important file")?;
    println!("{}", file);

    Ok(())
}
