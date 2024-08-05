use std::io::{self, Write};
use std::process::{self, Command, ExitStatus, Stdio};
use indicatif::{ProgressBar, ProgressStyle};
use std::os::unix::process::ExitStatusExt;

fn main() {
    println!("[*] Updating Package Database...");

    if let Err(e) = update_db() {
        eprintln!("[*] Error occurred while updating database: {}", e);
        process::exit(1);
    }

    println!("[*] Database updated successfully.\n");

    let tools = vec![
        "bat", "fastfetch", "git", "vlc", "libreoffice-still", "zathura", 
        "hexedit", "ncurses", "tmux", "neovim-nvchad"
    ];

    println!("==> Tools to be installed <===");
    for (i, tool) in tools.iter().enumerate() {
        println!("{}) {}", i + 1, tool);
    }

    print!("\n> Do you want to continue with the installation? (y/n): ");
    io::stdout().flush().expect("[*] Failed to flush stdout");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("[*] Failed to read input");
    let choice = choice.trim().to_lowercase();

    if choice == "y" {

        println!("[*] Installing Neovim...");
        if let Err(e) = install_nvim() {
            eprintln!("[*] Error occurred while installing Neovim: {}", e);
            process::exit(1);
        }

        println!("[*] Neovim installed successfully.\n");

        if let Err(e) = install_nvchad() {
            eprintln!("[*] Error while installing Nvchad: {}", e);
            process::exit(1);
        }

        println!("[*] Nvchad installed successfully.\n");
       
        if let Err(e) = install_tools(&tools) {
            eprintln!("[*] Error occurred while installing tools: {}", e);
            process::exit(1);
        }
    } 
    else {
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

fn install_nvim() -> io::Result<ExitStatus> {
    let nvim_status = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg("neovim")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    
    Ok(nvim_status)
}

fn install_nvchad() -> io::Result<ExitStatus> {
    let nvchad_status = Command::new("git")
        .arg("clone")
        .arg("https://github.com/NvChad/starter")
        .arg("~/.config/nvim")
        .status()?;

    Ok(nvchad_status)
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
                    return Err(io::Error::new(io::ErrorKind::Other, "Failed to install tool"));
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
