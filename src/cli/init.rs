use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use crate::precheck::check_r_py;
use crate::util::write_yml;

pub fn init() -> Result<(), std::io::Error> {
    // don't init again
    if env::var("USER_FA_DIR").is_ok() {
        eprintln!("✘ faproj root already exists");
        println!("✓ faproj root is {}", env::var("USER_FA_DIR").unwrap());
    } else {
        // create faproj/box/stbox/box.R
        // create faproj/box/config.yml
        // create faproj/job
        // create faproj/config.json
        let user_cwd = env::current_dir().unwrap();
        let path_proj = &user_cwd.join("faproj/job");
        let path_box = &user_cwd.join("faproj/box/stbox");
        let path_proj_conf = &user_cwd.join("faproj/config.json");
        let path_box_r = &user_cwd.join("faproj/box/stbox/box.R");
        let path_box_yml = &user_cwd.join("faproj/box/config.yml");
        let path_stbox = &user_cwd.join("faproj/box");

        // copy box.R and write config.yml
        fs::create_dir_all(path_proj)?;
        fs::create_dir_all(path_box)?;
        File::create(path_proj_conf)?;
        File::create(path_box_r)?;
        copy_box(path_box_r)?;
        write_yml(path_stbox, path_box_yml)?;

        // check predicate (R/Python)
        check_r_py();

        // export faproj root env vairable
        env::set_var("USER_FA_DIR", user_cwd.join("faproj"));
        println!("✓ export USER_FA_DIR={}", env::var("USER_FA_DIR").unwrap());
    }

    Ok(())
}

// copy the content of box.R
fn copy_box(des: &PathBuf) -> Result<(), std::io::Error> {
    let ctx = include_str!("../temp/box.R");
    fs::write(des, ctx)?;
    Ok(())
}
