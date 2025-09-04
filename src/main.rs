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
    println!("6) Advanced options");
    println!("7) Generate strong password");

    let master = prompt_password("Enter master password: ");

    let mut vault: Option<VaultV1> = match vault::load(&master) {
        Ok(v) => Some(v),
        Err(e) if e.to_string().contains("Incorrect master password") => {
            println!("Error: The master password is incorrect or the vault is corrupted. Please try again.");
            return Ok(());
        }
        Err(e) if e.to_string().contains("Vault not found") => {
            println!("Error: No vault found. Please initialize your vault first (option 1).");
            None
        }
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
                println!("Entries:");
                for (i, entry) in v.entries.iter().enumerate() {
                    println!("{}: {} / {} / {}", i+1, entry.name, entry.username, "********");
                    if let Some(notes) = &entry.notes {
                        println!("   Notes: {}", notes);
                    }
                }
                let reveal = prompt("Type 'get' to reveal passwords, or press Enter to continue: ");
                if reveal.trim() == "get" {
                    for (i, entry) in v.entries.iter().enumerate() {
                        println!("{}: {} / {} / {}", i+1, entry.name, entry.username, entry.password);
                        if let Some(notes) = &entry.notes {
                            println!("   Notes: {}", notes);
                        }
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
            "6" => {
                // Advanced submenu
                loop {
                    println!("\nAdvanced options:");
                    println!("a) Search entry");
                    println!("b) Remove entry");
                    println!("c) Edit entry");
                    println!("d) Back");
                    let adv = prompt("Choose: ");
                    match adv.as_str() {
                        "a" => {
                            if vault.is_none() {
                                println!("Vault not initialized. Please initialize first (option 1).");
                                continue;
                            }
                            let v = vault.as_ref().unwrap();
                            let name = prompt("Entry name to search: ");
                            if let Some(entry) = vault::find_entry(v, &name) {
                                println!("Found: {} / {} / {}", entry.name, entry.username, entry.password);
                                if let Some(notes) = &entry.notes {
                                    println!("   Notes: {}", notes);
                                }
                            } else {
                                println!("Entry not found.");
                            }
                        }
                        "b" => {
                            if vault.is_none() {
                                println!("Vault not initialized. Please initialize first (option 1).");
                                continue;
                            }
                            let mut v = vault.take().unwrap();
                            let name = prompt("Entry name to remove: ");
                            if vault::remove_entry(&mut v, &name) {
                                v.last_modified = Utc::now();
                                vault::save(&master, &v)?;
                                println!("Entry removed.");
                            } else {
                                println!("Entry not found.");
                            }
                            vault = Some(v);
                        }
                        "c" => {
                            if vault.is_none() {
                                println!("Vault not initialized. Please initialize first (option 1).");
                                continue;
                            }
                            let mut v = vault.take().unwrap();
                            let name = prompt("Entry name to edit: ");
                            if let Some(old) = vault::find_entry(&v, &name) {
                                println!("Editing entry: {} / {} / {}", old.name, old.username, old.password);
                                let new_name = prompt(&format!("New name [{}]: ", old.name));
                                let new_username = prompt(&format!("New username [{}]: ", old.username));
                                let new_password = prompt_password(&format!("New password [{}]: ", old.password));
                                let new_notes = prompt(&format!("New notes [{}]: ", old.notes.as_deref().unwrap_or("")));
                                let entry = Entry {
                                    name: if new_name.is_empty() { old.name.clone() } else { new_name },
                                    username: if new_username.is_empty() { old.username.clone() } else { new_username },
                                    password: if new_password.is_empty() { old.password.clone() } else { new_password },
                                    notes: if new_notes.is_empty() { old.notes.clone() } else { Some(new_notes) },
                                };
                                if vault::edit_entry(&mut v, &name, entry) {
                                    v.last_modified = Utc::now();
                                    vault::save(&master, &v)?;
                                    println!("Entry updated.");
                                }
                            } else {
                                println!("Entry not found.");
                            }
                            vault = Some(v);
                        }
                        "d" => break,
                        _ => println!("Invalid option."),
                    }
                }
            }
            "7" => {
                use rand::seq::SliceRandom;

                let length = loop {
                    let len_str = prompt("Desired password length (max 33): ");
                    match len_str.parse::<usize>() {
                        Ok(l) if l > 0 && l <= 33 => {
                            break l;
                        }
                        _ => println!("Please enter a valid number between 1 and 33."),
                    }
                };

                // Build a strong password
                let mut rng = rand::thread_rng();
                let lowercase: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
                let uppercase: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                let digits: &[u8] = b"0123456789";
                let symbols: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?/|";
                let all: Vec<u8> = [lowercase, uppercase, digits, symbols].concat();

                // Ensure at least one of each type
                let mut password = Vec::with_capacity(length);
                password.push(*lowercase.choose(&mut rng).unwrap());
                password.push(*uppercase.choose(&mut rng).unwrap());
                password.push(*digits.choose(&mut rng).unwrap());
                password.push(*symbols.choose(&mut rng).unwrap());
                for _ in 4..length {
                    password.push(*all.choose(&mut rng).unwrap());
                }
                password.shuffle(&mut rng);
                let password_str = String::from_utf8(password).unwrap();

                println!("Generated password: {}", password_str);

                let use_choice = prompt("Use this password in (1) existing entry, (2) new entry, or (3) do nothing? [1/2/3]: ");
                match use_choice.trim() {
                    "1" => {
                        if vault.is_none() {
                            println!("Vault not initialized. Please initialize first (option 1).");
                            continue;
                        }
                        let mut v = vault.take().unwrap();
                        let name = prompt("Entry name to update password: ");
                        if let Some(entry) = vault::find_entry_mut(&mut v, &name) {
                            entry.password = password_str.clone();
                            v.last_modified = Utc::now();
                            vault::save(&master, &v)?;
                            println!("Password updated for entry '{}'.", name);
                        } else {
                            println!("Entry not found.");
                        }
                        vault = Some(v);
                    }
                    "2" => {
                        if vault.is_none() {
                            println!("Vault not initialized. Please initialize first (option 1).");
                            continue;
                        }
                        let mut v = vault.take().unwrap();
                        let name = prompt("Entry name: ");
                        let username = prompt("Username: ");
                        let notes = prompt("Notes (optional): ");
                        let entry = Entry {
                            name,
                            username,
                            password: password_str.clone(),
                            notes: if notes.is_empty() { None } else { Some(notes) },
                        };
                        v.entries.push(entry);
                        v.last_modified = Utc::now();
                        vault::save(&master, &v)?;
                        println!("Entry added with generated password.");
                        vault = Some(v);
                    }
                    _ => {
                        println!("Password not used.");
                    }
                }
            }
            _ => println!("Invalid option."),
        }
    }
    Ok(())
}
