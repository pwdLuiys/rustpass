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

so this is a testing tool... but

Initialize vault:
```bash
cargo run -- init --master "your-master-password"

```
Add entry:
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



