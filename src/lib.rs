use clap::Parser;

mod cli;
mod precheck;
mod util;

pub use cli::CliArgs;

impl cli::CliArgs {
    // for main.rs
    pub fn run() -> anyhow::Result<()> {
        let cmd = cli::CliArgs::parse();
        match cmd.command {
            Some(cli::Audit::Init) => cli::init()?,
            Some(cli::Audit::New(job)) => {
                cli::new(&job.client.to_lowercase(), &job.year, job.activity, job.ide)?
            }
            Some(cli::Audit::Show(action)) => action.carry(),
            Some(cli::Audit::Report(job)) => {
                precheck::qrender(&job.client.to_lowercase(), &job.year, job.activity, job.ide)?
            }
            None => (),
        }
        Ok(())
    }
}

// put it here to avoid mod import
impl cli::ShowCommand {
    pub fn carry(&self) {
        match &self {
            cli::ShowCommand::Tree => precheck::show_job_tree(),
            cli::ShowCommand::List => precheck::show_job_list(),
        }
    }
}
