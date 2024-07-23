use std::io::{self, Write};
use std::process;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use indicatif::ProgressStyle;
use indicatif::ProgressBar;
use std::os::unix::process::ExitStatusExt;

fn main() {
    println!("[*] Updating Package Database..");

    match update_db() {
        Ok(status) => {
            if status.success() {
                println!("[*] Done");
            } else {
                eprintln!("[*] Update Failed");
            }
        }
        Err(e) => {
            eprintln!("[*] Error occurred: {}", e);
        }
    }

    println!("==> Tools to be installed <===");

    let tools = vec!["bat", "fastfetch", "git", "vlc", "libreoffice-still", "zathura", "zathura", "hexedit"];
    for (i, tool) in tools.iter().enumerate() {
        println!("{}) {}", i + 1, tool);
    }

    print!("\n> Do you want to continue? (y/n): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("[*] Failed to read input");
    let choice = choice.trim().to_lowercase();

    if choice == "y" {
        match install_tools(&tools) {
            Ok(status) => {
                if status.success() {
                    println!("[*] Tools installed successfully.");
                } else {
                    eprintln!("[*] Failed to install tools.");
                }
            }
            Err(e) => {
                eprintln!("[*] Error occurred: {}", e);
            }
        }
    } else {
        println!("[*] Installation cancelled.");
        process::exit(0);
    }
}

fn update_db() -> io::Result<ExitStatus> {
    Command::new("sudo")
        .arg("pacman")
        .arg("-Sy")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
}

fn install_tools(tools: &[&str]) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new(tools.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({msg})")
        .progress_chars("#>-"));

    for tool in tools {
        pb.set_message(format!("[*] Installing {}", tool));

        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg(tool) 
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(st) => {
                if st.success() {
                    pb.inc(1);
                } else {
                    pb.println(format!("[*] Failed to install {}", tool));
                    pb.finish_and_clear();
                    return status;
                }
            }
            Err(e) => {
                pb.println(format!("[*] Error: {}", e));
                pb.finish_and_clear();
                return Err(e);
            }
        }
    }

    pb.finish_with_message("[*] All tools installed successfully.");
    Ok(ExitStatus::from_raw(0))
}
