# 🔐 RustPass

A **minimalist password manager** written in Rust, inspired by Bitwarden and other modern vaults.  
**RustPass** uses strong cryptography (`Argon2` + `XChaCha20Poly1305`) to protect your secrets with a single master password.

---

## 🚀 Features
- Initialize a secure vault with a master password
- Key derivation using **Argon2**
- Authenticated encryption with **XChaCha20Poly1305**
- Support for multiple entries (name, username, password, notes)
- Local, secure encrypted storage
- Minimal interactive CLI

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/rustpass.git
cd rustpass
```bash
Build: 

```bash
cargo build --release
```bash
The binary will be generated at:

```bash
target/release/rustpass
```bash
(Optional) Move it to your PATH:

```bash
sudo mv target/release/rustpass /usr/local/bin/ (Linux)
```bash
🖥️ Usage

Run the program:

```bash
cargo run
```bash
You’ll see the main menu:

```bash
RustPass - Minimalist Password Manager
1) Initialize vault
2) Add entry
3) List entries
4) Save vault
5) Exit
```bash
🔑 1. Initialize vault

Creates a new encrypted vault protected by your master password.

➕ 2. Add entry

Adds a credential (name, username, password, notes).

📜 3. List entries

Lists all stored entries in the vault.

💾 4. Save vault

Saves the vault back to disk.

🚪 5. Exit

Closes the program.

(Simple, no?)

🔒 Storage

By default, rustpass stores:

The encrypted vault in CBOR format

A unique salt in a separate file

Default paths:

```bash
~/.local/share/RustPass/vault.cbor
~/.local/share/RustPass/salt.bin
```bash
🧪 Testing

Run tests (future support planned):

cargo test

🌍 Roadmap
Export/Import to JSON/CSV
Direct commands (rustpass add, rustpass list, etc.)
TUI (ncurses) or GUI integration
Cross-platform builds (Linux, macOS, Windows)

🤝 Contributing
Fork the repository

Create a feature branch (git checkout -b my-feature)

Commit your changes (git commit -m "feat: add my feature")

Push to your branch (git push origin my-feature)

Open a Pull Request 🚀 (maybe i need u? xxxxxx)
