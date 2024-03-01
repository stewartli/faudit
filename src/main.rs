use faudit;

fn main() -> anyhow::Result<()> {
    faudit::CliArgs::run()?;
    Ok(())
}
