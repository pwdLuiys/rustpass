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
