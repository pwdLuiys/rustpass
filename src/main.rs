mod model;
mod crypto;
mod vault;

use std::io::{self, Write};
use anyhow::Result;
use model::{Entry, VaultV1};
use chrono::Utc;

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn prompt_password(msg: &str) -> String {
    rpassword::prompt_password(msg).unwrap()
}

fn main() -> Result<()> {
    println!("RustPass - Minimalist Password Manager");
    println!("1) Initialize vault");
    println!("2) Add entry");
    println!("3) List entries");
    println!("4) Save vault");
    println!("5) Exit");

    let master = prompt_password("Enter master password: ");

    let mut vault: Option<VaultV1> = match vault::load(&master) {
        Ok(v) => Some(v),
        Err(e) if e.to_string().contains("VaultNotInitialized") => None,
        Err(e) => return Err(e),
    };

    loop {
        let choice = prompt("Choose an option: ");
        match choice.as_str() {
            "1" => {
                vault::init(&master)?;
                println!("Vault initialized.");
                vault = Some(vault::load(&master)?);
            }
            "2" => {
                if vault.is_none() {
                    println!("Vault not initialized. Please initialize first (option 1).");
                    continue;
                }
                let mut v = vault.take().unwrap();
                let name = prompt("Entry name: ");
                let username = prompt("Username: ");
                let password = prompt_password("Password: ");
                let notes = prompt("Notes (optional): ");
                let entry = Entry {
                    name,
                    username,
                    password,
                    notes: if notes.is_empty() { None } else { Some(notes) },
                };
                v.entries.push(entry);
                v.last_modified = Utc::now();
                vault::save(&master, &v)?;
                println!("Entry added.");
                vault = Some(v);
            }
            "3" => {
                if vault.is_none() {
                    println!("Vault not initialized. Please initialize first (option 1).");
                    continue;
                }
                let v = vault.as_ref().unwrap();
                for (i, entry) in v.entries.iter().enumerate() {
                    println!("{}: {} / {} / {}", i+1, entry.name, entry.username, entry.password);
                    if let Some(notes) = &entry.notes {
                        println!("   Notes: {}", notes);
                    }
                }
            }
            "4" => {
                if vault.is_none() {
                    println!("Vault not initialized. Please initialize first (option 1).");
                    continue;
                }
                let v = vault.as_ref().unwrap();
                vault::save(&master, v)?;
                println!("Vault saved.");
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
    Ok(())
}
