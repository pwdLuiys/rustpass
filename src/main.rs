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
    println!("RustPass - Gerenciador de Senhas Minimalista");
    println!("1) Inicializar cofre");
    println!("2) Adicionar entrada");
    println!("3) Listar entradas");
    println!("4) Salvar cofre");
    println!("5) Sair");

    let master = prompt_password("Digite a senha mestra: ");

    let mut vault: Option<VaultV1> = match vault::load(&master) {
        Ok(v) => Some(v),
        Err(e) if e.to_string().contains("VaultNotInitialized") => None,
        Err(e) => return Err(e),
    };

    loop {
        let choice = prompt("Escolha uma opção: ");
        match choice.as_str() {
            "1" => {
                vault::init(&master)?;
                println!("Cofre inicializado.");
                vault = Some(vault::load(&master)?);
            }
            "2" => {
                if vault.is_none() {
                    println!("Cofre não inicializado. Inicialize primeiro (opção 1).");
                    continue;
                }
                let mut v = vault.take().unwrap();
                let name = prompt("Nome da entrada: ");
                let username = prompt("Usuário: ");
                let password = prompt_password("Senha: ");
                let notes = prompt("Notas (opcional): ");
                let entry = Entry {
                    name,
                    username,
                    password,
                    notes: if notes.is_empty() { None } else { Some(notes) },
                };
                v.entries.push(entry);
                v.last_modified = Utc::now();
                vault::save(&master, &v)?;
                println!("Entrada adicionada.");
                vault = Some(v);
            }
            "3" => {
                if vault.is_none() {
                    println!("Cofre não inicializado. Inicialize primeiro (opção 1).");
                    continue;
                }
                let v = vault.as_ref().unwrap();
                for (i, entry) in v.entries.iter().enumerate() {
                    println!("{}: {} / {} / {}", i+1, entry.name, entry.username, entry.password);
                    if let Some(notes) = &entry.notes {
                        println!("   Notas: {}", notes);
                    }
                }
            }
            "4" => {
                if vault.is_none() {
                    println!("Cofre não inicializado. Inicialize primeiro (opção 1).");
                    continue;
                }
                let v = vault.as_ref().unwrap();
                vault::save(&master, v)?;
                println!("Cofre salvo.");
            }
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
    Ok(())
}
