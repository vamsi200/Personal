use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if let Err(e) = open_file(&args[1]) {
            eprintln!("ERROR: {}", e);
        }
    } else {
        let output = Command::new("sh")
            .arg("-c")
            .arg("rg --files --hidden $HOME | fzf")
            .output()
            .expect("[*] Failed to start rg and fzf");

        let selected_file = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if selected_file.is_empty() {
            println!("[*] No file selected.");
            return;
        }

        if let Some(parent) = Path::new(&selected_file).parent() {
            let parent_path = format!("\"{}\"", parent.display());
            if let Err(e) = copy_path(&parent_path) {
                eprintln!("ERROR: {}", e);
            }
            println!("[*] File DIR - {}", parent_path);
        }

        if let Err(e) = open_or_prompt(&selected_file) {
            eprintln!("ERROR: {}", e);
        }
    }
}

fn open_file(file: &str) -> io::Result<()> {
    open_or_prompt(file)
}

fn copy_path(path: &str) -> io::Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("echo {} | xclip -selection clipboard", path))
        .status()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Couldn't send the path to xclip: {}", e),
            )
        })?;
    Ok(())
}

fn open_or_prompt(file: &str) -> io::Result<()> {
    let extension = file.rsplit('.').next().unwrap_or("");
    let application = determine_application(extension);

    if let Some(app) = application {
        Command::new(app).arg(file).status().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("ERROR: Failed to open file with {}: {}", app, e),
            )
        })?;
    } else {
        prompt_open_with_nvim(file)?;
    }
    Ok(())
}

fn determine_application(extension: &str) -> Option<&'static str> {
    match extension.to_lowercase().as_str() {
        "pdf" => Some("okular"),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" => Some("eog"),
        "mp3" | "wav" | "flac" | "aac" | "mp4" | "avi" | "mkv" | "mov" | "webm" => Some("mpv"),
        "zip" | "tar.gz" | "rar" | "7z" => Some("xdg-open"),
        "db" | "sqlite" => Some("sqlite3"),
        "docx" | "xlsx" | "pptx" => Some("libreoffice"),
        "bin" => Some("hexeditor"),
        "rs" | "sh" | "lua" | "config" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "py" | "js"
        | "ts" | "tsx" | "jsx" | "html" | "htm" | "css" | "json" | "yaml" | "yml" | "toml"
        | "md" | "txt" | "go" | "java" | "kt" | "php" | "rb" | "sql" | "asm" | "s" | "zsh"
        | "bash" | "fish" => Some("nvim"),
        _ => None,
    }
}

fn prompt_open_with_nvim(file: &str) -> io::Result<()> {
    print!("> nvim? (y/n): ");
    io::stdout().flush().expect("[*] Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR: Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" {
        Command::new("nvim").arg(file).status().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to open file with nvim: {}", e),
            )
        })?;
    }
    Ok(())
}
