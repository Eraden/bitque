use std::collections::{HashMap, HashSet};
use std::fs::*;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::time::Duration;
use std::time::SystemTime;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

const INPUT: &str = "./jirs-client/js/styles.css";

type Css = Arc<RwLock<CssFile>>;

// mod prop;

#[derive(Debug)]
enum Partial {
    String(String),
    File(Css),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum FileState {
    Clean,
    Dirty,
    Dead,
}

#[derive(Debug)]
struct CssFile {
    pub path: String,
    pub lines: Vec<Partial>,
    pub last_changed: SystemTime,
    pub state: FileState,
}

impl CssFile {
    pub fn new(path: String) -> Self {
        Self {
            path,
            lines: vec![],
            last_changed: SystemTime::UNIX_EPOCH,
            state: FileState::Clean,
        }
    }

    pub fn drop_dead(&mut self) {
        let mut old = vec![];
        std::mem::swap(&mut self.lines, &mut old);

        for child in old {
            match child {
                Partial::String(_) => {
                    self.lines.push(child);
                }
                Partial::File(file) => {
                    let state = file.read().map(|f| f.state).unwrap();

                    if state != FileState::Dead {
                        if let Ok(mut css) = file.write() {
                            css.drop_dead();
                        }
                        self.lines.push(Partial::File(file));
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for CssFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.state == FileState::Dead {
            return Ok(());
        }
        f.write_str(format!("\n/* -- {} --- */\n\n", self.path).as_str())?;
        for line in self.lines.iter() {
            match line {
                Partial::String(line) => {
                    f.write_str(line.as_str())?;
                    f.write_str("\n")?;
                }
                Partial::File(file) => {
                    if let Ok(css) = file.read() {
                        f.write_str(format!("{}", css).as_str())?;
                        f.write_str("\n")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Application {
    input: String,
    output: Option<String>,
    watch: bool,
    prelude_selector: bool,
    files_map: HashMap<String, HashSet<String>>,
    fm: HashMap<String, Css>,
    root_file: Option<Css>,
    sender: Option<Sender<DebouncedEvent>>,
}

impl Application {
    fn read_timestamp(input: &Path) -> Result<SystemTime, String> {
        std::fs::File::open(input)
            .and_then(|file| file.metadata())
            .and_then(|meta| meta.modified())
            .map_err(|e| format!("{}", e))
    }

    fn check_timestamps(
        &mut self,
        input: &Path,
        output_timestamp: SystemTime,
    ) -> Result<bool, String> {
        let input_dir = input
            .parent()
            .ok_or_else(|| format!("Not a valid path {:?}", input))?;

        let path = input_dir.to_str().unwrap();
        let paths =
            glob::glob(format!("{}/**/*.css", path).as_str()).map_err(|e| format!("{}", e))?;
        for path in paths.filter_map(Result::ok) {
            if Self::read_timestamp(path.as_path())? > output_timestamp {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn parse(&mut self) -> Result<(), String> {
        let root_path = self.input.to_string();
        let root = std::path::Path::new(&root_path);
        let root_file = self.parse_file(root)?;
        self.root_file = Some(root_file);
        Ok(())
    }

    fn parse_file(&mut self, input: &Path) -> Result<Css, String> {
        let file_path = input.display().to_string();
        let input_dir = input
            .parent()
            .ok_or_else(|| format!("Not a valid path {:?}", input))?;

        let file = if self.fm.contains_key(&file_path) {
            self.fm.get(&file_path).unwrap().clone()
        } else {
            let css = Arc::new(RwLock::new(CssFile::new(file_path.clone())));
            self.fm.insert(file_path.clone(), css.clone());
            if let Some(ref tx) = self.sender {
                let path = Path::new(&file_path);
                tx.send(DebouncedEvent::Create(path.to_path_buf()))
                    .map_err(|e| format!("{}", e))?;
            }
            css
        };

        if let Ok(mut css) = file.write() {
            css.last_changed = Self::read_timestamp(input)?;
        }

        for line in read_to_string(file_path.as_str())
            .map_err(|e| format!("{}", e))?
            .lines()
        {
            let l = line.trim();
            match l {
                "" => continue,
                _ if l.starts_with("@import ") => {
                    let imported = line
                        .replace("@import ", "")
                        .trim()
                        .replace("\"", "")
                        .replace(";", "")
                        .to_string();
                    let child = input_dir
                        .join(imported.as_str())
                        .canonicalize()
                        .map_err(|e| format!("{}", e))?;
                    let child_file = self.parse_file(&child)?;

                    if let Ok(mut css) = file.write() {
                        css.lines.push(Partial::File(child_file));
                    }
                }
                _ => {
                    if let Ok(mut css) = file.write() {
                        css.lines.push(Partial::String(l.to_string()));
                    }
                }
            }
        }
        Ok(file)
    }

    pub fn mark_dirty(&mut self, path: &Path) {
        if let Ok(mut css) = self.css_at_path(path) {
            css.state = FileState::Dirty;
        }
    }

    pub fn mark_dead(&mut self, path: &Path) {
        if let Ok(mut css) = self.css_at_path(path) {
            css.state = FileState::Dead;
        }
    }

    fn css_at_path(&mut self, path: &Path) -> Result<RwLockWriteGuard<CssFile>, bool> {
        self.fm
            .get(path.display().to_string().as_str())
            .ok_or(false)
            .and_then(|css| css.write().or(Err(false)))
    }

    fn refresh(&mut self) {
        if let Ok(mut root) = self
            .root_file
            .as_mut()
            .ok_or_else(|| false)
            .and_then(|f| f.write().map_err(|_| false))
        {
            root.drop_dead();
        }
        let mut old = HashMap::new();
        std::mem::swap(&mut old, &mut self.fm);
        for (key, file) in old.into_iter() {
            if file
                .read()
                .map(|f| f.state != FileState::Dead)
                .unwrap_or_default()
            {
                self.fm.insert(key, file);
            }
        }
    }

    fn print(&self) {
        let css = match self.root_file.as_ref().unwrap().read() {
            Ok(css) => css,
            _ => return,
        };
        match self.output.as_ref() {
            Some(f) => {
                std::fs::create_dir_all(Path::new(f).parent().unwrap()).unwrap();
                std::fs::write(f, format!("{}", css)).unwrap();
                println!("CSS merge done");
            }
            _ => println!("{}", css),
        }
    }

    pub fn pipe(&mut self, tx: Sender<DebouncedEvent>) {
        self.sender = Some(tx);
    }
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

    let mut app = Application {
        input: matches.value_of("input").unwrap().to_string(),
        output: matches.value_of("output").map(|s| s.to_string()),
        watch: matches.is_present("watch"),
        prelude_selector: matches.is_present("prelude"),
        files_map: Default::default(),
        fm: Default::default(),
        root_file: None,
        sender: None,
    };
    let root_path = app.input.to_string();
    let root = std::path::Path::new(&root_path);

    let output_timestamp = matches
        .value_of("output")
        .ok_or_else(|| std::io::Error::from_raw_os_error(0))
        .and_then(File::open)
        .and_then(|file| file.metadata())
        .and_then(|meta| meta.modified())
        .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);

    if app.check_timestamps(root, output_timestamp)? {
        return Ok(());
    }

    let (tx, rx) = channel();
    app.pipe(tx.clone());
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    app.parse()?;
    app.print();

    if !matches.is_present("watch") {
        return Ok(());
    }

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::NoticeWrite(path)) => {
                app.mark_dirty(path.as_path());
                if let Err(s) = app.parse_file(&path) {
                    eprintln!("{}", s);
                }
                app.print();
            }
            Ok(DebouncedEvent::NoticeRemove(path)) => {
                app.mark_dead(path.as_path());
                watcher.unwatch(path).unwrap();
                app.refresh();
                app.print();
            }
            Ok(DebouncedEvent::Create(path)) => {
                if let Err(e) = watcher.watch(path, RecursiveMode::NonRecursive) {
                    eprintln!("{}", e);
                }
            }
            Ok(_event) => (),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}
