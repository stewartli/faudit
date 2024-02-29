use crate::{
    cli::check_proj_home,
    util::{read_json, write_json},
};
use std::{
    env,
    process::{Command, Stdio},
};

// check if R/Python are installed
pub fn check_r_py() {
    Command::new("R")
        .arg("--version")
        .output()
        .expect("✘ R is not installed");
    Command::new("python3")
        .arg("--version")
        .output()
        .expect("✘ Python is not installed");
    println!("✓ R and Ptyhon already exist");
}

// open RStudio IDE
pub fn rstudio() {
    Command::new("powershell.exe")
        .args(["Start", "RStudio"])
        .output()
        .expect("✘ RStudio command failed to start");
}

// quarto render repot.qmd
pub fn qrender(client: &str, year: &str, activity: String, ide: bool) -> std::io::Result<()> {
    // get job/client/year/report/report.qmd
    let path_proj = check_proj_home().unwrap();
    let path_proj_conf = path_proj.join("config.json");
    let path_job = path_proj.join("job");
    let path_year = path_job.join(client).join(year);
    let path_report = path_year.join("report/report.qmd");

    // quarto render
    let mut qmd = Command::new("quarto");
    if qmd.spawn().is_ok() {
        qmd.args(["render", path_report.to_str().unwrap()])
            .output()
            .expect("✘ failed to execute process");
        println!("✓ done quarto render {}", path_report.display());
    } else {
        eprintln!("✘ Quarto command failed to start");
    }

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

// bash tree
pub fn show_job_tree() {
    let path_proj = check_proj_home().unwrap();
    let path_job = path_proj.join("job");
    env::set_current_dir(&path_job).unwrap();

    let out = Command::new("tree")
        .args(["-L", "2"])
        .stdout(Stdio::piped())
        .output()
        .expect("✘ tree command failed to start");

    let stdout = String::from_utf8(out.stdout).unwrap();
    println!("{}", stdout);
}

// bash find
pub fn show_job_list() {
    let path_proj = check_proj_home().unwrap();
    let path_job = path_proj.join("job");
    env::set_current_dir(&path_job).unwrap();

    let out = Command::new("find")
        .args(["-maxdepth", "2", "-type", "d", "-print"])
        .stdout(Stdio::piped())
        .output()
        .expect("✘ ls command failed to start");

    let stdout = String::from_utf8(out.stdout).unwrap();
    println!("{}", stdout);
}
