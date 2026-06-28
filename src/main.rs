/*
-- get flutter project directory
-- ask to clean before build or build
-- edit local.properties file
-- ask for apk or aab
-- ask if signed or not signed
-- then release type
*/
mod model;
use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::Command,
};

use clap::Parser;

use crate::model::Params;

fn main() {
    // parse_local_properties(&PathBuf::from(
    //     "/Users/pius/Documents/projects/rust/abbifex/local.properties",
    // ));
    // run_pub_get();
    // ps aux | grep -v grep | grep -c  suricata

    let args = Params::parse();
    match parse_local_properties(&args.path, args.version_code, args.build_name) {
        Ok(_) => println!("Success!"),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Error: The target properties file does not exist.");
            } else {
                eprintln!("An unexpected error occurred: {}", err);
            }
        }
    }
}

fn parse_local_properties(
    path: &PathBuf,
    version_code: i64,
    version_name: String,
) -> io::Result<()> {
    let mut found_version_code = false;
    let mut found_build_mode = false;
    let mut found_version_name = false;
    let local_path = path.join("android").join("local.properties");
    println!("{}", &local_path.display());
    let local_properties_file = File::open(&local_path).expect("could not read file");
    let reader = BufReader::new(local_properties_file);
    let mut tokens: Vec<String> = Vec::new();
    for lines in reader.lines() {
        if let Ok(mut texts) = lines {
            if texts.starts_with("#") || texts.starts_with("//") || texts.is_empty() {
                continue;
            }
            if texts.contains("flutter.versionCode") {
                texts = String::from(format!("flutter.versionCode={version_code}"));
                found_version_code = true;
            }
            if texts.contains("flutter.buildMode") {
                texts = String::from("flutter.buildMode=release");
                found_build_mode = true;
            }
            if texts.contains("flutter.versionName") {
                texts = String::from(format!("flutter.versionName={version_name}"));
                found_version_name = true;
            }

            tokens.push(texts);
        }
    }
    if !found_version_code {
        tokens.push(String::from("flutter.versionCode=350"));
        println!("Im doing this");
    }
    if !found_build_mode {
        tokens.push(String::from("flutter.buildMode=release"));
    }
    if !found_version_name {
        tokens.push(String::from("flutter.versionName=4.1.2"));
    }
    println!("{:?}", tokens);
    let _ = fs::write(&local_path, tokens.join("\n"))?;
    Ok(())
}

fn run_pub_get(command: String, arg: String) {
    let mut flutter_command = Command::new(command);
    flutter_command.arg(arg);

    let status = flutter_command.status().expect("Failed to run mkdir");

    if status.success() {
        println!("Directory created successfully.");
    } else {
        println!("Failed with exit code: {:?}", status.code());
    }
}

fn parse_app_build_gralde(path: &PathBuf) {
    let app_build_gradle = File::open(path).expect("could not read file");
    let reader = BufReader::new(app_build_gradle);
    let mut tokens: Vec<String> = Vec::new();
    for lines in reader.lines() {
        if let Ok(mut texts) = lines {
            if texts.starts_with("#") || texts.starts_with("//") || texts.is_empty() {
                continue;
            }
            if texts.contains("flutter.versionCode") {
                texts = String::from("flutter.versionCode=350")
            }
            if texts.contains("flutter.buildMode") {
                texts = String::from("flutter.buildMode=debug")
            }
            if texts.contains("flutter.versionName") {
                texts = String::from("flutter.versionName=4.1.2")
            }

            tokens.push(texts);
        }
    }
    let _ = fs::write(path, tokens.join("\n"));
}
