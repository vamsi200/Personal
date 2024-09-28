use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

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
            let parent_path = format!("\"{}\"", parent.display().to_string());
            println!("[*] File DIR - {}", parent_path);
            let _ = save_file_path(&parent_path);
        }
        if let Err(e) = open_or_prompt(&selected_file) {
            eprintln!("ERROR: {}", e);
        }
    }
}

fn save_file_path(file: &str) -> io::Result<()> {
    fs::write("/tmp/file_path.txt", file)?;
    Ok(())
}

fn open_file(file: &str) -> io::Result<()> {
    open_or_prompt(file)
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
    let mut apps = HashMap::new();
    apps.insert("pdf", "zathura");
    apps.insert("jpg", "eog");
    apps.insert("png", "eog");
    apps.insert("gif", "eog");
    apps.insert("bmp", "eog");
    apps.insert("tiff", "eog");
    apps.insert("jpeg", "eog");
    apps.insert("mp3", "vlc");
    apps.insert("wav", "vlc");
    apps.insert("flac", "vlc");
    apps.insert("aac", "vlc");
    apps.insert("mp4", "vlc");
    apps.insert("avi", "vlc");
    apps.insert("mkv", "vlc");
    apps.insert("mov", "vlc");
    apps.insert("zip", "xdg-open");
    apps.insert("tar.gz", "xdg-open");
    apps.insert("rar", "xdg-open");
    apps.insert("7z", "xdg-open");
    apps.insert("db", "sqlite3");
    apps.insert("sqlite", "sqlite3");
    apps.insert("docx", "libreoffice");
    apps.insert("xlsx", "libreoffice");
    apps.insert("pptx", "libreoffice");
    apps.insert("bin", "hex_editor");
    apps.insert("rs", "nvim");
    apps.insert("sh", "nvim");
    apps.get(extension).copied()
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
