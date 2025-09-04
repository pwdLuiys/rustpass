mod model;
mod crypto;
mod vault;

use clap::{Parser, Subcommand, Args};
use anyhow::Result;
use model::Entry;
use chrono::Utc;
use colored::*;

#[derive(Parser)]
#[command(name = "rustpass")]
#[command(about = "Minimalist Password Manager (Bitwarden/ProtonPass style)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new vault
    Init(MasterOpt),
    /// Add a new entry
    Add(AddOpt),
    /// List all entries (passwords hidden)
    List(MasterOpt),
    /// Get entry details (password revealed)
    Get(GetOpt),
    /// Save the vault (noop for compatibility)
    Save(MasterOpt),
}

#[derive(Args, Clone)]
struct MasterOpt {
    /// Master password (required)
    #[arg(long)]
    master: String,
}

#[derive(Args, Clone)]
struct AddOpt {
    /// Master password (required)
    #[arg(long)]
    master: String,
    /// Entry name
    #[arg(long)]
    name: String,
    /// Username
    #[arg(long)]
    username: String,
    /// Password
    #[arg(long)]
    password: String,
    /// Notes (optional)
    #[arg(long)]
    notes: Option<String>,
}

#[derive(Args, Clone)]
struct GetOpt {
    /// Master password (required)
    #[arg(long)]
    master: String,
    /// Entry name
    #[arg(long)]
    name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(master_opt) => {
            let master = master_opt.master;
            match vault::init(&master) {
                Ok(_) => println!("{}", "Vault initialized.".green()),
                Err(e) => println!("{} {}", "Error initializing vault:".red(), e),
            }
        }
        Commands::Add(opt) => {
            let master = opt.master;
            let mut v = match vault::load(&master) {
                Ok(v) => v,
                Err(e) => {
                    if e.to_string().contains("Incorrect master password") {
                        println!("{}", "Error: The master password is incorrect or the vault is corrupted.".red());
                    } else if e.to_string().contains("Vault not found") {
                        println!("{}", "Warning: No vault found. Please initialize your vault first (init command).".yellow());
                    } else {
                        println!("{} {}", "Error:".red(), e);
                    }
                    return Ok(());
                }
            };
            let entry = Entry {
                name: opt.name,
                username: opt.username,
                password: opt.password,
                notes: opt.notes,
            };
            v.entries.push(entry);
            v.last_modified = Utc::now();
            vault::save(&master, &v)?;
            println!("{}", "Entry added.".green());
        }
        Commands::List(master_opt) => {
            let master = master_opt.master;
            let v = match vault::load(&master) {
                Ok(v) => v,
                Err(e) => {
                    if e.to_string().contains("Incorrect master password") {
                        println!("{}", "Error: The master password is incorrect or the vault is corrupted.".red());
                    } else if e.to_string().contains("Vault not found") {
                        println!("{}", "Warning: No vault found. Please initialize your vault first (init command).".yellow());
                    } else {
                        println!("{} {}", "Error:".red(), e);
                    }
                    return Ok(());
                }
            };
            println!("{}", "Entries:".blue().bold());
            for (i, entry) in v.entries.iter().enumerate() {
                println!(
                    "{}: {} / {} / {}",
                    (i + 1).to_string().cyan(),
                    entry.name.cyan(),
                    entry.username.cyan(),
                    "********".cyan()
                );
                if let Some(notes) = &entry.notes {
                    println!("   {}", format!("Notes: {}", notes).cyan());
                }
            }
        }
        Commands::Get(opt) => {
            let master = opt.master;
            let v = match vault::load(&master) {
                Ok(v) => v,
                Err(e) => {
                    if e.to_string().contains("Incorrect master password") {
                        println!("{}", "Error: The master password is incorrect or the vault is corrupted.".red());
                    } else if e.to_string().contains("Vault not found") {
                        println!("{}", "Warning: No vault found. Please initialize your vault first (init command).".yellow());
                    } else {
                        println!("{} {}", "Error:".red(), e);
                    }
                    return Ok(());
                }
            };
            if let Some(entry) = vault::find_entry(&v, &opt.name) {
                println!(
                    "{} / {} / {}",
                    entry.name.green().bold(),
                    entry.username.green(),
                    entry.password.green()
                );
                if let Some(notes) = &entry.notes {
                    println!("   {}", format!("Notes: {}", notes).green());
                }
            } else {
                println!("{}", "Entry not found.".yellow());
            }
        }
        Commands::Save(master_opt) => {
            let master = master_opt.master;
            let v = match vault::load(&master) {
                Ok(v) => v,
                Err(e) => {
                    if e.to_string().contains("Incorrect master password") {
                        println!("{}", "Error: The master password is incorrect or the vault is corrupted.".red());
                    } else if e.to_string().contains("Vault not found") {
                        println!("{}", "Warning: No vault found. Please initialize your vault first (init command).".yellow());
                    } else {
                        println!("{} {}", "Error:".red(), e);
                    }
                    return Ok(());
                }
            };
            vault::save(&master, &v)?;
            println!("{}", "Vault saved.".green());
        }
    }
    Ok(())
}
