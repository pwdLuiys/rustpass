# rustpass

A secure, cross-platform CLI password manager inspired by Bitwarden and ProtonPass.

## Features

- **Multiple vaults:** Create, select, rename, and delete multiple vaults, each with its own master password.
- **Master password encryption:** All vaults are encrypted using Argon2id and XChaCha20Poly1305. Passwords are never stored or shown in plaintext except when explicitly requested.
- **Entry management:** Add, edit, delete, and list password entries for each vault.
- **Notes support:** Store optional notes with each entry.
- **Search and reveal:** Search for entries and reveal passwords securely.
- **Colorful CLI:** Clear, colored output for errors, warnings, and success messages.
- **Cross-platform:** Works on Linux, macOS, and Windows.
- **Secure input:** Master password is always requested securely (hidden input).
- **No password in history:** Passwords are never passed via command-line arguments.
- **Packaging:** Prebuilt binaries available for all major platforms.

## Commands

- `create-vault --name <NAME>`: Create a new vault.
- `delete-vault --name <NAME>`: Delete a vault.
- `edit-vault --old-name <OLD> --new-name <NEW>`: Rename a vault.
- `list-vaults`: List all vaults.
- `select-vault --name <NAME>`: Select a vault to use.
- `init`: Unlock the selected vault (verifies master password).
- `add --name <NAME> --username <USERNAME> --password <PASSWORD> [--notes <NOTES>]`: Add a new entry.
- `edit-entry --name <NAME>`: Edit an entry interactively.
- `delete-entry --name <NAME>`: Delete an entry.
- `list`: List all entries (passwords hidden).
- `get --name <NAME>`: Show details of an entry (password revealed).
- `save`: Save the current vault.

## Usage Examples

```sh
rustpass create-vault --name Personal
rustpass select-vault --name Personal
rustpass init
rustpass add --name Github --username user --password 1234
rustpass list
rustpass get --name Github
rustpass edit-entry --name Github
rustpass delete-entry --name Github
rustpass save
```

## Security

- All vaults are encrypted with a master password using Argon2id (key derivation) and XChaCha20Poly1305 (encryption).
- Master password is never stored or logged.
- Passwords are never passed via command-line arguments; always prompted securely.
- Each vault is stored in a separate encrypted file in the user's data directory.

## Installation

### Download prebuilt binaries

Go to [GitHub Releases](https://github.com/pwdLuiys/rustpass/releases) and download the binary for your platform:

- `rustpass-x86_64-unknown-linux-gnu.tar.gz` (Linux)
- `rustpass-x86_64-apple-darwin.zip` (macOS)
- `rustpass-x86_64-pc-windows-msvc.zip` (Windows)

Extract the archive and move the `rustpass` binary to a directory in your `$PATH` (e.g., `/usr/local/bin` or `C:\Program Files`).

### Build from source

```sh
git clone https://github.com/pwdLuiys/rustpass.git
cd rustpass
cargo build --release
```
the binary will be generated at
```bash
target/release/rustpass

```
but if you want, u can move ofc
(how?):

```bash
sudo mv target/release/rustpass /usr/local/bin/

```
how do i use this tool?

So in this VERY SIMPLE and TESTING version, u do a 
```bash
cargo run
```

u gonna see some options

(But first, the app gonna ask u a "main password, just put whatever you want and dont forget it") {but if u did forget just do a cargo clean, and cargo build again [IN THIS VERSION FOR TESTING]}

And yeah, u have to press (1) before actually using the tool (idk why i did this)

"RustPass - Minimalist Password Manager
1) Initialize vault
2) Add entry
3) List entries
4) Save vault
5) Exit"

 1. Initialize vault
Creates a new encrypted vault protected by your master password.

 2. Add entry
Adds a credential (name, username, password, notes).

 3. List entries
Lists all stored entries in the vault.

 4. Save vault
Saves the vault back to disk.

 5. Exit
Closes the program.


Storage, by default rustpass stores "The encrypted vault in CBOR format -
A unique salt in a separate file"

and the path by default is: 
```bash
cargo run -- add --master "your-master-password" --name "gmail" --username "me@gmail.com" --password "mypassword"

```
List entrie
```bash
cargo run -- list --master "your-master-password"
```
Get a entrie (this allow u to see the passwords in the list [reveals them])
```bash
cargo run -- get --master "your-master-password" --name "gmail"
```



