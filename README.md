# rustpass

A secure, cross-platform CLI password manager inspired by Bitwarden and ProtonPass.

## Features

- **Multiple vaults:** Create, select, rename, and delete independent vaults, each protected by its own master password.
- **Strong encryption:** Vaults are encrypted using Argon2id for key derivation and XChaCha20Poly1305 for authenticated encryption.
- **Password entries:** Add, edit, delete, and list entries (name, username, password, notes) in your selected vault.
- **Master password security:** The master password is never stored or logged; always prompted securely using hidden input.
- **Cross-platform:** Works on Linux, macOS, and Windows. Binaries are available for all platforms.
- **Colorful CLI:** User-friendly, colored output for commands, errors, and status messages.
- **Safe storage:** Vaults are stored in your OS's standard data directory, isolated per user.
- **Easy packaging:** Distributed as prebuilt binaries via GitHub Releases.

## Usage

### Vault Management

- **Create a vault:**  
  `rustpass create-vault --name Personal`
- **Delete a vault:**  
  `rustpass delete-vault --name Personal`
- **Rename a vault:**  
  `rustpass edit-vault --old-name Personal --new-name Work`
- **List all vaults:**  
  `rustpass list-vaults`
- **Select a vault to use:**  
  `rustpass select-vault --name Work`
- **Initialize (unlock) a vault:**  
  `rustpass init`  
  _(You will be prompted for the master password)_

### Entry Management

- **Add an entry:**  
  `rustpass add --name Github --username user --password 1234 [--notes "my notes"]`
- **Edit an entry:**  
  `rustpass edit-entry --name Github`  
  _(You will be prompted for new values; leave blank to keep current)_
- **Delete an entry:**  
  `rustpass delete-entry --name Github`
- **List entries:**  
  `rustpass list`
- **Get entry details:**  
  `rustpass get --name Github`
- **Save vault:**  
  `rustpass save`

### Security

- All sensitive operations prompt for the master password using hidden input.
- Vaults are encrypted and authenticated; only the correct master password can unlock them.
- No passwords are ever printed unless explicitly requested (e.g., with `get`).

## Installation

### FOR WINDOWS

*You have to add 'rustpass' PATH on your pc.*
#### [How to do that?](https://superuser.com/questions/1861276/how-to-set-a-folder-to-the-path-environment-variable-in-windows-11)


### Build from source

You can build from source with Rust:

### First, on windows do
```sh
winget install --id=Rustlang.Rustup  -e
```

### First, on Linux do 

```sh
paru -S rustup
or
yay -S rustup
or
sudo pacman -S rustup

```

### Now you can clone!!
```sh
git clone https://github.com/pwdLuiys/rustpass.git
cd rustpass
cargo build --release
```

## Example workflow

```sh
rustpass create-vault --name Personal
rustpass select-vault --name Personal
rustpass init
rustpass add --name Github --username user --password 1234
rustpass list
rustpass edit-entry --name Github
rustpass delete-entry --name Github
rustpass save
```

## License

MIT License © 2025 LUIS HENRIQUE (pwdLuiys)  
See [LICENSE](LICENSE).

