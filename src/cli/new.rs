use std::env;
use std::path::Path;
use std::{fs, path::PathBuf};

use crate::precheck::rstudio;
use crate::util::{read_json, write_json};

pub fn new(client: &str, year: &str, activity: String, ide: bool) -> anyhow::Result<()> {
    // create faproj/job/client/year
    let path_proj = check_proj_home().unwrap();
    let path_proj_conf = path_proj.join("config.json");
    let path_job = path_proj.join("job");
    let path_client = path_job.join(client);
    let path_year = path_job.join(client).join(year);

    // check if job/client/year exists
    if path_client.exists() {
        if path_year.exists() {
            eprintln!("✘ cd {} as it exists", &path_year.display());
        } else {
            fs::create_dir_all(&path_year)?;
            create_audit_folder(&path_year)?;
        }
    } else {
        fs::create_dir_all(&path_year)?;
        create_audit_folder(&path_year)?;
    }

    // cd to cwd job/client/year
    env::set_current_dir(&path_year)?;
    let cwd = env::current_dir().unwrap();
    println!("✓ your cwd is {}", cwd.display());

    // copy clean.R and report.qmd to cwd
    copy_include()?;

    // record activity by appending to config.json
    let mut record = read_json(&path_proj_conf)?;
    write_json(
        format!("{client}_{year}").as_str(),
        &path_proj_conf,
        activity,
        &mut record,
    )?;

    // start RStudio IDE
    if ide {
        rstudio();
    }

    Ok(())
}

// locate faproj root
pub fn check_proj_home() -> Option<PathBuf> {
    let path_proj = env::var_os("USER_FA_DIR").unwrap().into_string().unwrap();
    let path_proj = Path::new(&path_proj);
    if path_proj.exists() {
        println!("✓ USER_FA_DIR is set");
        Some(path_proj.to_path_buf())
    } else {
        eprintln!("✘ USER_FA_DIR is not set");
        None
    }
}

// create job/client/year/awp folder
fn create_audit_folder(path_job: &PathBuf) -> anyhow::Result<()> {
    let path_pbc = path_job.join("pbc");
    let path_awp = path_job.join("awp");
    let path_report = path_job.join("report");
    let path_misc = path_job.join("misc");
    let path_doc = path_job.join("doc");
    fs::create_dir_all(&path_pbc).unwrap();
    fs::create_dir_all(&path_awp).unwrap();
    fs::create_dir_all(&path_report).unwrap();
    fs::create_dir_all(&path_misc).unwrap();
    fs::create_dir_all(&path_doc).unwrap();
    Ok(())
}

// copy assets to job/client/year folder
fn copy_include() -> anyhow::Result<()> {
    fs::File::create("awp/clean.R")?;
    let ctx_clean = include_str!("../temp/clean.R");
    fs::write("awp/clean.R", ctx_clean)?;

    fs::File::create("report/report.qmd")?;
    let ctx_report = include_str!("../temp/report.qmd");
    fs::write("report/report.qmd", ctx_report)?;
    Ok(())
}
