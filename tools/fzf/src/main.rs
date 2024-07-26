use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    // Run `rg` and `fzf` to select a file
    let selected_file = Command::new("sh")
        .arg("-c")
        .arg("rg --files $HOME | fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("[*] Failed to start rg and fzf")
        .wait_with_output()
        .expect("[*] Failed to get output from fzf")
        .stdout;

    let selected_file = String::from_utf8_lossy(&selected_file).trim().to_string();

    if selected_file.is_empty() {
        println!("[*] No file selected.");
        return;
    }

    let extension = selected_file
        .rsplit('.')
        .next()
        .unwrap_or("");

    let firefox = ["pdf"];
    let eog = ["jpg", "png", "gif", "bmp", "tiff"];
    let vlc = ["mp3", "wav", "flac", "aac", "mp4", "avi", "mkv", "mov"];
    let xdg_open = ["pdf", "jpg", "png", "gif", "bmp", "tiff", "mp3", "wav", "flac", "aac", "mp4", "avi", "mkv", "mov", "zip", "tar.gz", "rar", "7z"];
    let sqlite3 = ["db", "sqlite"];
    let libreoffice = ["docx", "xlsx", "pptx"];
    let hex_editor = ["bin"];

    fn contains(ext: &str, arr: &[&str]) -> bool {
        arr.contains(&ext)
    }

    let application = match extension {
        ext if contains(ext, &firefox) => Some("firefox"),
        ext if contains(ext, &eog) => Some("eog"),
        ext if contains(ext, &vlc) => Some("vlc"),
        ext if contains(ext, &xdg_open) => Some("xdg-open"),
        ext if contains(ext, &sqlite3) => Some("sqlite3"),
        ext if contains(ext, &libreoffice) => Some("libreoffice"),
        ext if contains(ext, &hex_editor) => Some("hex_editor"),
        ext if ["sh", "txt", "md"].contains(&ext) => Some("nvim"),
        _ => None,
    };

    println!("[*] File Directory :- {}", selected_file);
   

    if let Some(app) = application {
        Command::new(app)
            .arg(&selected_file)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect(&format!("[*] Failed to open file with {}", app));
    } else {
        print!("> Do you want to open with nvim? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("[*] Failed to read input");
        let input = input.trim().to_lowercase();

        if input == "y" {
            Command::new("nvim")
                .arg(&selected_file)
                .status()
                .expect("[*] Failed to open file with nvim[*]");
        } else {
            if let Some(parent) = fs::read_dir(&selected_file).ok() {
                for entry in parent {
                    let entry = entry.expect("[*] Failed to read entry");
                    println!("{}", entry.path().display());
                }
            }         
        }
    }
}



