mod model;
mod crypto;
mod vault;
use rand::RngCore;

use clap::{Parser, Subcommand};
use anyhow::Result;
use model::Entry;
use chrono::Utc;
use colored::*;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustpass")]
#[command(about = "Minimalist Password Manager (Bitwarden/ProtonPass style)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new vault
    CreateVault {
        #[arg(long)]
        name: String,
    },
    /// Delete a vault
    DeleteVault {
        #[arg(long)]
        name: String,
    },
    /// Edit vault name
    EditVault {
        #[arg(long)]
        old_name: String,
        #[arg(long)]
        new_name: String,
    },
    /// List all vaults
    ListVaults,
    /// Select a vault to use
    SelectVault {
        #[arg(long)]
        name: String,
    },
    /// Initialize (login) to a vault
    Init,
    /// Add a new entry to the selected vault
    Add {
        #[arg(long)]
        name: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Edit an entry in the selected vault
    EditEntry {
        #[arg(long)]
        id: Option<u32>,
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete an entry from the selected vault
    DeleteEntry {
        #[arg(long)]
        id: Option<u32>,
        #[arg(long)]
        name: Option<String>,
    },
    /// List all entries in the selected vault
    List,
    /// Get entry details from the selected vault
    Get {
        #[arg(long)]
        id: Option<u32>,
        #[arg(long)]
        name: Option<String>,
    },
    /// Save the selected vault
    Save,
}

fn vaults_dir() -> PathBuf {
    let proj_dirs = directories::ProjectDirs::from("com", "rustpass", "RustPass").unwrap();
    let dir = proj_dirs.data_dir();
    fs::create_dir_all(dir).unwrap();
    dir.to_path_buf()
}

fn vault_file(name: &str) -> PathBuf {
    vaults_dir().join(format!("vault_{}.cbor", name))
}

fn salt_file(name: &str) -> PathBuf {
    vaults_dir().join(format!("salt_{}.bin", name))
}

fn current_vault_file() -> PathBuf {
    vaults_dir().join(".current_vault")
}

fn set_current_vault(name: &str) {
    fs::write(current_vault_file(), name).unwrap();
}

fn get_current_vault() -> Option<String> {
    fs::read_to_string(current_vault_file()).ok().map(|s| s.trim().to_string())
}

fn vault_exists(name: &str) -> bool {
    vault_file(name).exists() && salt_file(name).exists()
}

fn prompt_password(msg: &str) -> String {
    rpassword::prompt_password(msg).unwrap()
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn print_usage_examples() {
    println!("{}", "Commands:".blue().bold());
    println!("  rustpass create-vault --name <NAME>");
    println!("  rustpass delete-vault --name <NAME>");
    println!("  rustpass edit-vault --old-name <OLD> --new-name <NEW>");
    println!("  rustpass list-vaults");
    println!("  rustpass select-vault --name <NAME>");
    println!("  rustpass init");
    println!("  rustpass add --name <NAME> --username <USERNAME> --password <PASSWORD> [--notes <NOTES>]");
    println!("  rustpass edit-entry --name <NAME>");
    println!("  rustpass delete-entry --name <NAME>");
    println!("  rustpass list");
    println!("  rustpass get --name <NAME>");
    println!("  rustpass save");
    println!();
    println!("{}", "Examples:".blue().bold());
    println!("  rustpass create-vault --name Personal");
    println!("  rustpass select-vault --name Personal");
    println!("  rustpass init");
    println!("  rustpass add --name Github --username user --password 1234");
    println!("  rustpass edit-entry --name Github");
    println!("  rustpass delete-entry --name Github");
    println!("  rustpass list");
    println!("  rustpass get --name Github");
    println!("  rustpass save");
    println!();
}

fn find_entry_by_id(vault: &model::VaultV1, id: u32) -> Option<&model::Entry> {
    vault.entries.iter().find(|e| e.id == id)
}
fn find_entry_mut_by_id(vault: &mut model::VaultV1, id: u32) -> Option<&mut model::Entry> {
    vault.entries.iter_mut().find(|e| e.id == id)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if std::env::args().len() == 1 {
        print_usage_examples();
        return Ok(());
    }

    match cli.command {
        Commands::CreateVault { name } => {
            if vault_exists(&name) {
                println!("{}", "Vault already exists.".yellow());
                return Ok(());
            }
            let master = prompt_password("Set master password for this vault: ");
            let salt = {
                let mut salt = [0u8; 16];
                rand::thread_rng().fill_bytes(&mut salt);
                salt
            };
            let key = crypto::derive_key(&master, &salt);
            let vault = model::VaultV1 {
                created_at: Utc::now(),
                last_modified: Utc::now(),
                entries: Vec::new(),
            };
            let mut buf = Vec::new();
            ciborium::ser::into_writer(&vault, &mut buf)?;
            let (ciphertext, nonce) = crypto::encrypt(&key, &buf);

            let mut vault_data = Vec::new();
            vault_data.extend_from_slice(&nonce);
            vault_data.extend_from_slice(&ciphertext);

            fs::write(vault_file(&name), &vault_data)?;
            fs::write(salt_file(&name), &salt)?;
            println!("{}", "Vault created.".green());
        }
        Commands::DeleteVault { name } => {
            if !vault_exists(&name) {
                println!("{}", "Vault not found.".yellow());
                return Ok(());
            }
            println!("{}", format!("You are about to delete vault '{}'. This action is irreversible and all data will be lost.", name).red().bold());
            let confirm = prompt("Are you sure you want to delete this vault? (y/n): ");
            if !confirm.trim().eq_ignore_ascii_case("y") {
                println!("{}", "Vault deletion cancelled.".yellow());
                return Ok(());
            }
            let master = prompt_password("Enter master password for this vault: ");
            // Try to decrypt to verify password
            let salt = match fs::read(salt_file(&name)) {
                Ok(s) => s,
                Err(_) => {
                    println!("{}", "Vault not found or corrupted.".red());
                    return Ok(());
                }
            };
            let key = crypto::derive_key(&master, &salt);
            let vault_data = match fs::read(vault_file(&name)) {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Vault not found or corrupted.".red());
                    return Ok(());
                }
            };
            let nonce = &vault_data[..24];
            let ciphertext = &vault_data[24..];
            if crypto::decrypt(&key, ciphertext, nonce.try_into().unwrap()).is_err() {
                println!("{}", "Incorrect master password. Vault not deleted.".red());
                return Ok(());
            }
            let vf = vault_file(&name);
            let sf = salt_file(&name);
            let mut deleted = false;
            if vf.exists() {
                fs::remove_file(vf)?;
                deleted = true;
            }
            if sf.exists() {
                fs::remove_file(sf)?;
                deleted = true;
            }
            if deleted {
                println!("{}", "Vault deleted.".green());
                if get_current_vault().as_deref() == Some(&name) {
                    let _ = fs::remove_file(current_vault_file());
                }
            } else {
                println!("{}", "Vault not found.".yellow());
            }
        }
        Commands::EditVault { old_name, new_name } => {
            if !vault_exists(&old_name) {
                println!("{}", "Vault not found.".yellow());
                return Ok(());
            }
            if vault_exists(&new_name) {
                println!("{}", "A vault with the new name already exists.".yellow());
                return Ok(());
            }
            fs::rename(vault_file(&old_name), vault_file(&new_name))?;
            fs::rename(salt_file(&old_name), salt_file(&new_name))?;
            if get_current_vault().as_deref() == Some(&old_name) {
                set_current_vault(&new_name);
            }
            println!("{}", "Vault renamed.".green());
        }
        Commands::ListVaults => {
            let dir = vaults_dir();
            let mut found = false;
            println!("{}", "Available vaults:".blue().bold());
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let fname = entry.file_name().to_string_lossy().to_string();
                if fname.starts_with("vault_") && fname.ends_with(".cbor") {
                    let name = fname.trim_start_matches("vault_").trim_end_matches(".cbor");
                    println!("{}", name.cyan());
                    found = true;
                }
            }
            if !found {
                println!("{}", "No vaults found.".yellow());
            }
        }
        Commands::SelectVault { name } => {
            if !vault_exists(&name) {
                println!("{}", "Vault not found.".yellow());
                return Ok(());
            }
            set_current_vault(&name);
            println!("{}", format!("Vault '{}' selected.", name).green());
        }
        Commands::Init => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            let master = prompt_password(&format!("Master password for vault '{}': ", vault_name));
            let salt = match fs::read(salt_file(&vault_name)) {
                Ok(s) => s,
                Err(_) => {
                    println!("{}", "Vault not found or corrupted.".red());
                    return Ok(());
                }
            };
            let key = crypto::derive_key(&master, &salt);
            let vault_data = match fs::read(vault_file(&vault_name)) {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Vault not found or corrupted.".red());
                    return Ok(());
                }
            };
            let nonce = &vault_data[..24];
            let ciphertext = &vault_data[24..];
            let plaintext = match crypto::decrypt(&key, ciphertext, nonce.try_into().unwrap()) {
                Ok(p) => p,
                Err(_) => {
                    println!("{}", "Incorrect master password or corrupted vault.".red());
                    return Ok(());
                }
            };
            let _vault: model::VaultV1 = ciborium::de::from_reader(plaintext.as_slice())?;
            println!("{}", format!("Vault '{}' unlocked.", vault_name).green());
        }
        Commands::Add { name, username, password, notes } => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            println!("{}", format!(
                "Adding entry '{}' for user '{}' to vault '{}'.",
                name, username, vault_name
            ).blue().bold());
            let master = prompt_password("Master password: ");
            let mut v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            let next_id = v.entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;
            let entry = Entry {
                id: next_id,
                name,
                username,
                password,
                notes,
            };
            v.entries.push(entry);
            v.last_modified = Utc::now();
            vault::save_named(&vault_name, &master, &v)?;
            println!("{}", "Entry added.".green());
        }
        Commands::List => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            let master = prompt_password("Master password: ");
            let v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            println!("{}", "Entries:".blue().bold());
            for entry in &v.entries {
                println!(
                    "[{}] {} / {} / {}",
                    entry.id.to_string().cyan(),
                    entry.name.cyan(),
                    entry.username.cyan(),
                    "********".cyan()
                );
                if let Some(notes) = &entry.notes {
                    println!("   {}", format!("Notes: {}", notes).cyan());
                }
            }
        }
        Commands::Get { id, name } => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            let master = prompt_password("Master password: ");
            let v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            let entry = if let Some(id) = id {
                find_entry_by_id(&v, id)
            } else if let Some(name) = name {
                vault::find_entry(&v, &name)
            } else {
                None
            };
            if let Some(entry) = entry {
                println!(
                    "[{}] {} / {} / {}",
                    entry.id.to_string().green().bold(),
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
        Commands::EditEntry { id, name } => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            println!("{}", format!("Editing entry in vault '{}'.", vault_name).blue().bold());
            let master = prompt_password("Master password: ");
            let mut v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            let entry = if let Some(id) = id {
                find_entry_mut_by_id(&mut v, id)
            } else if let Some(name) = name {
                vault::find_entry_mut(&mut v, &name)
            } else {
                None
            };
            if let Some(entry) = entry {
                println!("Current values:");
                println!("ID: {}", entry.id);
                println!("Name: {}", entry.name);
                println!("Username: {}", entry.username);
                println!("Password: {}", entry.password);
                println!("Notes: {}", entry.notes.as_deref().unwrap_or(""));
                let new_name = prompt(&format!("New name [{}]: ", entry.name));
                let new_username = prompt(&format!("New username [{}]: ", entry.username));
                let new_password = prompt_password(&format!("New password [{}]: ", entry.password));
                let new_notes = prompt(&format!("New notes [{}]: ", entry.notes.as_deref().unwrap_or("")));
                entry.name = if new_name.is_empty() { entry.name.clone() } else { new_name };
                entry.username = if new_username.is_empty() { entry.username.clone() } else { new_username };
                entry.password = if new_password.is_empty() { entry.password.clone() } else { new_password };
                entry.notes = if new_notes.is_empty() { entry.notes.clone() } else { Some(new_notes) };
                v.last_modified = Utc::now();
                vault::save_named(&vault_name, &master, &v)?;
                println!("{}", "Entry updated.".green());
            } else {
                println!("{}", "Entry not found.".yellow());
            }
        }
        Commands::DeleteEntry { id, name } => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            println!("{}", format!("Deleting entry from vault '{}'.", vault_name).blue().bold());
            let master = prompt_password("Master password: ");
            let mut v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            if let Some(id_val) = id {
                let before = v.entries.len();
                v.entries.retain(|e| e.id != id_val);
                if v.entries.len() < before {
                    v.last_modified = Utc::now();
                    vault::save_named(&vault_name, &master, &v)?;
                    println!("{}", "Entry deleted.".green());
                } else {
                    println!("{}", "Entry not found.".yellow());
                }
            } else if let Some(ref name_val) = name {
                let matches: Vec<&Entry> = v.entries.iter().filter(|e| e.name.eq_ignore_ascii_case(name_val)).collect();
                if matches.is_empty() {
                    println!("{}", "Entry not found.".yellow());
                } else if matches.len() > 1 {
                    println!("{}", format!("Warning: There are {} entries with the name '{}'.", matches.len(), name_val).yellow());
                    println!("Do you want to delete ALL of them? (y/n)");
                    println!("Tip: To delete a specific entry, use --id <ID> instead of --name.");
                    let confirm = prompt("");
                    if confirm.trim().eq_ignore_ascii_case("y") {
                        v.entries.retain(|e| !e.name.eq_ignore_ascii_case(name_val));
                        v.last_modified = Utc::now();
                        vault::save_named(&vault_name, &master, &v)?;
                        println!("{}", "All matching entries deleted.".green());
                    } else {
                        println!("{}", "No entries deleted.".yellow());
                    }
                } else {
                    v.entries.retain(|e| !e.name.eq_ignore_ascii_case(name_val));
                    v.last_modified = Utc::now();
                    vault::save_named(&vault_name, &master, &v)?;
                    println!("{}", "Entry deleted.".green());
                }
            } else {
                println!("{}", "Please provide either --id or --name to delete an entry.".yellow());
            }
        }
        Commands::Save => {
            let vault_name = match get_current_vault() {
                Some(name) => name,
                None => {
                    println!("{}", "No vault selected. Use 'select-vault --name <NAME>' first.".red());
                    return Ok(());
                }
            };
            let master = prompt_password("Master password: ");
            let v = vault::load_named(&vault_name, &master)
                .map_err(|e| {
                    println!("{} {}", "Error:".red(), e);
                    e
                })?;
            vault::save_named(&vault_name, &master, &v)?;
            println!("{}", "Vault saved.".green());
        }
    }
    Ok(())
}
