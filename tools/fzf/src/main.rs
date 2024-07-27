use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        open_file(&args[1]);
    } else {
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
        println!("[*] File DIR - {selected_file}");

        open_or_prompt(&selected_file);
    }
}

fn open_file(file: &str) {
    open_or_prompt(file);
}

fn open_or_prompt(file: &str) {
    let extension = file.rsplit('.').next().unwrap_or("");

    let application = determine_application(extension);

    if let Some(app) = application {
        Command::new(app)
            .arg(file)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect(&format!("[*] Failed to open file with {}", app));
    } else {
        prompt_open_with_nvim(file);
    }
}

fn determine_application(extension: &str) -> Option<&'static str> {
    let zathura = ["pdf"];
    let eog = ["jpg", "png", "gif", "bmp", "tiff"];
    let vlc = ["mp3", "wav", "flac", "aac", "mp4", "avi", "mkv", "mov"];
    let xdg_open = [
        "pdf", "jpg", "png", "gif", "bmp", "tiff", "mp3", "wav", "flac", "aac", "mp4",
        "avi", "mkv", "mov", "zip", "tar.gz", "rar", "7z"
    ];
    let sqlite3 = ["db", "sqlite"];
    let libreoffice = ["docx", "xlsx", "pptx"];
    let hex_editor = ["bin"];

    fn contains(ext: &str, arr: &[&str]) -> bool {
        arr.contains(&ext)
    }

    match extension {
        ext if contains(ext, &zathura) => Some("zathura"),
        ext if contains(ext, &eog) => Some("eog"),
        ext if contains(ext, &vlc) => Some("vlc"),
        ext if contains(ext, &xdg_open) => Some("xdg-open"),
        ext if contains(ext, &sqlite3) => Some("sqlite3"),
        ext if contains(ext, &libreoffice) => Some("libreoffice"),
        ext if contains(ext, &hex_editor) => Some("hex_editor"),
        ext if ["sh", "txt", "md"].contains(&ext) => Some("nvim"),
        _ => None,
    }
}

fn prompt_open_with_nvim(file: &str) {
    print!("> Do you want to open with nvim? (y/n): ");
    io::stdout().flush().expect("[*] Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("[*] Failed to read input");
    let input = input.trim().to_lowercase();

    if input == "y" {
        Command::new("nvim")
            .arg(file)
            .status()
            .expect("[*] Failed to open file with nvim");
    } else {
        if let Some(parent) = fs::read_dir(file).ok() {
            for entry in parent {
                let entry = entry.expect("[*] Failed to read entry");
                println!("{}", entry.path().display());
            }
        }
    }
}
