use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_yaml;
use std::collections::{BTreeMap, HashMap};
use std::{
    env, fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

// write config.yml
pub fn write_yml(path_stbox: &PathBuf, path_box_yml: &PathBuf) -> Result<(), std::io::Error> {
    // specify my yaml format
    let mut field = HashMap::new();
    field.insert("rbox".to_string(), path_stbox.to_str().unwrap().to_string());
    field.insert("pin".to_string(), String::new());
    field.insert("duckdb".to_string(), String::new());
    field.insert("psql".to_string(), String::new());
    field.insert("ftp".to_string(), String::new());

    let mut config_yaml = BTreeMap::new();
    config_yaml.insert("default".to_string(), &field);
    config_yaml.insert("dev".to_string(), &field);

    // deserialize ctx
    let ctx_yaml = serde_yaml::to_string(&config_yaml).unwrap();
    let de_ctx_yaml: BTreeMap<String, HashMap<String, String>> =
        serde_yaml::from_str(&ctx_yaml).unwrap();

    // write out
    let file_yml = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_box_yml)?;
    serde_yaml::to_writer(file_yml, &de_ctx_yaml).unwrap();

    Ok(())
}

// write config.json
#[derive(Serialize, Deserialize, Debug)]
pub struct Configjson {
    job: String,
    activity: String,
    filepath: PathBuf,
    date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configjsonlist {
    item: Vec<Configjson>,
}

pub fn write_json(
    client_year: &str,
    path_proj_config: &PathBuf,
    activity: String,
    record: &mut Configjsonlist,
) -> Result<(), std::io::Error> {
    // push new ctx to a clean json format without \\ string character
    let mut ctx_json = json!({"item": record.item});
    ctx_json["item"].as_array_mut().unwrap().push(json!({
        "job": client_year,
        "activity": activity,
        "filepath":  env::current_dir().unwrap(),
        "date": Utc::now().naive_utc(),
    }));

    // write out
    let file_josn = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(path_proj_config)?;
    let mut writer = BufWriter::new(file_josn);
    serde_json::to_writer_pretty(&mut writer, &ctx_json)?;
    writer.flush()?;
    Ok(())
}

pub fn read_json(path_proj_conf: &PathBuf) -> Result<Configjsonlist, std::io::Error> {
    if fs::metadata(path_proj_conf).unwrap().len() == 0 {
        // init a vector if config.json is empty
        Ok(Configjsonlist { item: Vec::new() })
    } else {
        // convert to a struct if config.json is not empty
        let data = fs::read_to_string(path_proj_conf).unwrap();
        let p: Configjsonlist = serde_json::from_str(&data).unwrap();
        Ok(p)
    }
}
