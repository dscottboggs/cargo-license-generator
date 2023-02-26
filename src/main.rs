use chrono::Datelike;
use chrono::Local;
use structopt::StructOpt;

use cargo_generate_license::{create_license, write_license};
use std::env;
use std::process::{self, Command};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT", required = true)]
    inputs: Vec<String>,
    #[structopt(long = "author")]
    author: Option<String>,
    #[structopt(long = "project")]
    project: Option<String>,
    #[structopt(long = "year")]
    year: Option<u32>,
    #[structopt(long = "output")]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let year = if let Some(year) = opt.year {
        year
    } else {
        let dt = Local::now();
        dt.year() as u32
    };
    let author = if let Some(author) = opt.author {
        author
    } else {
        let name = Command::new("git")
            .args(["config", "--get", "user.name"])
            .output()?;
        let email = Command::new("git")
            .args(["config", "--get", "user.email"])
            .output()?;
        format!(
            "{} <{}>",
            String::from_utf8(name.stdout)?.trim(),
            String::from_utf8(email.stdout)?.trim()
        )
    };
    let project = opt.project.unwrap_or_else(|| {
        env::current_dir()
            .expect("use --project: Not found current dir")
            .file_name()
            .expect("use --project: Not found directory name")
            .to_os_string()
            .into_string()
            .expect("use --project: Fail to unwrap os_string")
    });
    let output = opt.output.unwrap_or_else(|| "LICENSE".to_string());
    let multi_license = opt.inputs.len() > 1;

    opt.inputs.iter().for_each(|s| {
        let license = create_license(s.as_str()).unwrap_or_else(|| {
            eprintln!("Not found match license: {}", s);
            process::exit(1);
        });
        let license_text = license.notice(year, &author, &project);
        let output = if multi_license {
            format!("{}-{}", output, s.to_uppercase())
        } else {
            output.clone()
        };
        write_license(&license_text, &output).unwrap_or_else(|error| {
            eprintln!("Can not write license text to \"{}\": {}", output, error);
            process::exit(1);
        });
    });
    Ok(())
}
