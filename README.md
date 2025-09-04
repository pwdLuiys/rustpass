# RustPass

A **minimalist password manager** written in Rust, inspired by Bitwarden and other modern vaults.  
**RustPass** uses strong cryptography (`Argon2` + `XChaCha20Poly1305`) to protect your secrets with a single master password.

---

## shits that this basic thing have
- Initialize a secure vault with a master password
- Key derivation using **Argon2**
- Authenticated encryption with **XChaCha20Poly1305**
- Support for multiple entries (name, username, password, notes)
- Local, secure encrypted storage
- Minimal interactive CLI

---

## how do i install? (why dfuck u want ts anyway?)

Clone the repository:

```bash
git clone https://github.com/pwdLuiys/rustpass
cd rustpass
```

Build it
```bash
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
~/.local/share/RustPass/vault.cbor
~/.local/share/RustPass/salt.bin

```

for testing obviously u do a
```bash
cargo test

```

thats it :)

