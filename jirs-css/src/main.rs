use std::fs::*;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

const INPUT: &str = "./jirs-client/js/styles.css";

#[derive(Debug, Default)]
struct Configuration {
    input: String,
    output: Option<String>,
    watch: bool,
    prelude_selector: bool,
    files: Vec<String>,
}

impl Configuration {
    pub fn scan_fs(&mut self) -> Result<(), String> {
        let input_dir = Path::new(self.input.as_str())
            .parent()
            .ok_or_else(|| format!("Not a valid path {:?}", self.input))?;

        let path = input_dir.to_str().unwrap();
        let paths =
            glob::glob(format!("{}/**/*.css", path).as_str()).map_err(|e| format!("{}", e))?;
        for path in paths.filter_map(Result::ok) {
            self.files.push(path.display().to_string());
        }
        Ok(())
    }
}

fn merge_files(input: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let input_dir = input
        .clone()
        .parent()
        .ok_or_else(|| format!("Not a valid path {:?}", input))?;
    let contents: String =
        read_to_string(input).map_err(|_| format!("File cannot be read {:?}", input))?;

    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with("@import ") {
            out.push(line.to_string());
            continue;
        }
        let imported = {
            line.replace("@import ", "")
                .trim()
                .replace("\"", "")
                .replace(";", "")
                .to_string()
        };
        let child = input_dir.clone().join(imported.as_str());
        merge_files(child.as_path(), out)?;
    }
    Ok(())
}

fn read_timestamp(input: &Path) -> Result<SystemTime, String> {
    std::fs::File::open(input)
        .and_then(|file| file.metadata())
        .and_then(|meta| meta.modified())
        .map_err(|e| format!("{}", e))
}

fn check_timestamps(input: &Path, output_timestamp: SystemTime) -> Result<bool, String> {
    let input_dir = input
        .clone()
        .parent()
        .ok_or_else(|| format!("Not a valid path {:?}", input))?;

    let path = input_dir.to_str().unwrap();
    let paths = glob::glob(format!("{}/**/*.css", path).as_str()).map_err(|e| format!("{}", e))?;
    for path in paths.filter_map(Result::ok) {
        if read_timestamp(path.as_path())? > output_timestamp {
            return Ok(false);
        }
    }
    Ok(true)
}

fn main() -> Result<(), String> {
    let matches = clap::App::new("jirs-css")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .default_value(INPUT)
                .takes_value(true),
        )
        .arg(clap::Arg::with_name("output").short("O").takes_value(true))
        .arg(clap::Arg::with_name("watch").short("W"))
        .arg(
            clap::Arg::with_name("prelude")
                .short("p")
                .help("Prepend file name as class to each selector"),
        )
        .get_matches();

    let mut config = Configuration {
        input: matches.value_of("input").unwrap().to_string(),
        output: matches.value_of("output").map(|s| s.to_string()),
        watch: matches.is_present("watch"),
        prelude_selector: matches.is_present("prelude"),
        files: vec![],
    };
    config.scan_fs()?;
    println!("{:?}", config);

    let output_timestamp = matches
        .value_of("output")
        .ok_or(std::io::Error::from_raw_os_error(0))
        .and_then(|path| File::open(path))
        .and_then(|file| file.metadata())
        .and_then(|meta| meta.modified())
        .unwrap_or_else(|_| SystemTime::UNIX_EPOCH.clone());

    let mut file =
        std::fs::File::open(matches.value_of("input").unwrap()).map_err(|e| format!("{}", e))?;
    let input = matches.value_of("input").unwrap();

    if check_timestamps(Path::new(input), output_timestamp)? {
        return Ok(());
    }

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("{}", e))?;

    let mut out = vec![];
    merge_files(std::path::Path::new(input), &mut out)?;

    match matches.value_of("output") {
        Some(output) => {
            std::fs::create_dir_all(Path::new(output).parent().unwrap()).unwrap();
            std::fs::write(output, out.join("\n")).unwrap()
        }
        None => println!("{}", out.join("\n")),
    }

    Ok(())
}
