use crate::model::VaultV1;
use crate::crypto::{derive_key, encrypt, decrypt};
use chrono::Utc;
use ciborium::{ser, de};
use directories::ProjectDirs;
use rand::RngCore;
use anyhow::{Result, Context};
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;

const VAULT_FILE: &str = "vault.cbor";
const SALT_FILE: &str = "salt.bin";

fn vault_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "rustpass", "RustPass").context("Could not determine project directory")?;
    let dir = proj_dirs.data_dir();
    create_dir_all(dir)?;
    Ok(dir.join(VAULT_FILE))
}

fn salt_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "rustpass", "RustPass").context("Could not determine project directory")?;
    let dir = proj_dirs.data_dir();
    create_dir_all(dir)?;
    Ok(dir.join(SALT_FILE))
}

#[allow(clippy::result_large_err)]
pub fn init(master: &str) -> Result<()> {
    let salt = {
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        salt
    };
    let key = derive_key(master, &salt);
    let vault = VaultV1 {
        created_at: Utc::now(),
        last_modified: Utc::now(),
        entries: Vec::new(),
    };
    let mut buf = Vec::new();
    ser::into_writer(&vault, &mut buf)?;
    let (ciphertext, nonce) = encrypt(&key, &buf);

    let mut vault_data = Vec::new();
    vault_data.extend_from_slice(&nonce);
    vault_data.extend_from_slice(&ciphertext);

    let vault_file = vault_path()?;
    let mut f = File::create(vault_file)?;
    f.write_all(&vault_data)?;

    let salt_file = salt_path()?;
    let mut sf = File::create(salt_file)?;
    sf.write_all(&salt)?;

    Ok(())
}

#[allow(clippy::result_large_err)]
pub fn load(master: &str) -> Result<VaultV1> {
    let salt_file = salt_path()?;
    if !salt_file.exists() {
        anyhow::bail!("VaultNotInitialized");
    }
    let mut salt = [0u8; 16];
    File::open(salt_file)?.read_exact(&mut salt)?;

    let key = derive_key(master, &salt);

    let vault_file = vault_path()?;
    if !vault_file.exists() {
        anyhow::bail!("VaultNotInitialized");
    }
    let mut vault_data = Vec::new();
    File::open(vault_file)?.read_to_end(&mut vault_data)?;

    let nonce = &vault_data[..24];
    let ciphertext = &vault_data[24..];

    let plaintext = decrypt(&key, ciphertext, nonce.try_into().unwrap())?;
    let vault: VaultV1 = de::from_reader(plaintext.as_slice())?;
    Ok(vault)
}

#[allow(clippy::result_large_err)]
pub fn save(master: &str, vault: &VaultV1) -> Result<()> {
    let salt_file = salt_path()?;
    let mut salt = [0u8; 16];
    File::open(salt_file)?.read_exact(&mut salt)?;

    let key = derive_key(master, &salt);

    let mut buf = Vec::new();
    ser::into_writer(vault, &mut buf)?;
    let (ciphertext, nonce) = encrypt(&key, &buf);

    let mut vault_data = Vec::new();
    vault_data.extend_from_slice(&nonce);
    vault_data.extend_from_slice(&ciphertext);

    let vault_file = vault_path()?;
    let mut f = File::create(vault_file)?;
    f.write_all(&vault_data)?;

    Ok(())
}
