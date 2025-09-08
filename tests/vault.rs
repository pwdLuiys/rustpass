use rustpass::vault;
use rustpass::model::Entry;
use std::fs;
use chrono::Utc;

fn cleanup() {
    // Remove vault and salt files if they exist
    let proj_dirs = directories::ProjectDirs::from("com", "rustpass", "RustPass").unwrap();
    let dir = proj_dirs.data_dir();
    let vault_file = dir.join("vault.cbor");
    let salt_file = dir.join("salt.bin");
    let _ = fs::remove_file(vault_file);
    let _ = fs::remove_file(salt_file);
}

#[test]
fn test_init_creates_files() {
    cleanup();
    let master = "unit-test-master";
    vault::init(master).expect("Vault init should succeed");

    let proj_dirs = directories::ProjectDirs::from("com", "rustpass", "RustPass").unwrap();
    let dir = proj_dirs.data_dir();
    assert!(dir.join("vault.cbor").exists(), "vault.cbor should exist after init");
    assert!(dir.join("salt.bin").exists(), "salt.bin should exist after init");

    cleanup();
}

#[test]
fn test_add_entry_save_load_integrity() {
    cleanup();
    let master = "unit-test-master";
    vault::init(master).expect("Vault init should succeed");

    let mut vault_data = vault::load(master).expect("Should load vault after init");
    let entry = Entry {
        name: "TestEntry".to_string(),
        username: "user".to_string(),
        password: "pass".to_string(),
        notes: Some("note".to_string()),
        id: 0, // Replace with a valid id value as per Entry struct definition
    };
    vault_data.entries.push(entry.clone());
    vault_data.last_modified = Utc::now();
    vault::save(master, &vault_data).expect("Should save vault");

    let loaded = vault::load(master).expect("Should load vault after save");
    assert_eq!(loaded.entries.len(), 1);
    let loaded_entry = &loaded.entries[0];
    assert_eq!(loaded_entry.name, entry.name);
    assert_eq!(loaded_entry.username, entry.username);
    assert_eq!(loaded_entry.password, entry.password);
    assert_eq!(loaded_entry.notes, entry.notes);

    cleanup();
}
