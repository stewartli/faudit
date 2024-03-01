use std::fs;

#[allow(unused)]
pub fn cp_temp(src: &str, des: &str) -> anyhow::Result<()> {
    fs::copy(src, des)?;
    println!("✓ file copied");
    Ok(())
}
