use clap::{Args, Parser, Subcommand};

// main cli interface
#[derive(Parser, Debug)]
#[command(name = "faudit", version = "v0.1.0", author = "stewartli", about = "Financial audit workflow tool", long_about = None)]
#[command(after_help = "© 2024 RAudit Solution LLP")]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Audit>,
}

// define subcommands
#[derive(Subcommand, Debug)]
pub enum Audit {
    /// Initialize the faproj root
    Init,
    /// Create the faproj client job
    New(Job),
    /// View the faproj job structure
    #[command(subcommand)]
    Show(ShowCommand),
    /// Render a client job report
    Report(Job),
}

// args for new subcommand
#[derive(Args, Debug)]
pub struct Job {
    /// Specify cient name
    #[arg(short, long = "client", help = "Provide a client name")]
    pub client: String,
    /// Specify job year
    #[arg(short, long = "year", help = "Provide the job year")]
    pub year: String,
    /// Specify your activity purpose
    #[arg(short, long = "activity", help = "Provide the current activity")]
    pub activity: String,
    /// Open RStudio IDE
    #[arg(
        short,
        long = "rstudio",
        default_value_t = false,
        help = "Open RStudio IDE"
    )]
    pub ide: bool,
}

// define subcommands for show subcommand
#[derive(Subcommand, Debug)]
pub enum ShowCommand {
    /// Show the tree view of faproj/job
    Tree,
    /// Show the list view of faproj/job
    List,
}
