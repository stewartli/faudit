use faudit;

fn main() -> Result<(), std::io::Error> {
    faudit::CliArgs::run()?;
    Ok(())
}
