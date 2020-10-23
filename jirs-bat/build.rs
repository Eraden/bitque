use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all("./tmp")?;

    std::process::Command::new("git")
        .arg("clone")
        .arg("https://github.com/sharkdp/bat.git")
        .arg("./tmp/bat")
        .spawn()?;
    std::process::Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .current_dir("./tmp/bat")
        .spawn()?;
    Ok(())
}
