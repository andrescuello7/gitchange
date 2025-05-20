use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::account::dirs::get_config_path;
use crate::account::loader::load_accounts;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub hash: String,
}


pub fn select_user(input: String) -> User {
    let path = get_config_path();
    let users: Vec<User> = load_accounts(&path);

    if let Ok(id) = input.trim().parse::<u32>() {
        for user in users {
            if user.id == id {
                return user;
            }
        }
    }

    println!("Operación inválida. ID no encontrado.");
    User {
        id: 0,
        name: "".into(),
        email: "".into(),
        hash: "".into(),
    }
}


pub fn add_user_to_file(user: User) {
    let path = get_config_path();
    let mut users: Vec<User> = if Path::new(&path).exists() {
        load_accounts(&path)
    } else {
        Vec::new()
    };

    // Generar un nuevo ID
    let new_id = (users.iter().map(|u| u.id).max().unwrap_or(0)) + 1;

    let new_user = User {
        id: new_id,
        ..user
    };

    users.push(new_user.clone());

    // Escribir de nuevo
    let json = serde_json::to_string_pretty(&users).expect("No se pudo serializar JSON");
    let mut file = File::create(path).expect("No se pudo crear el archivo");
    file.write_all(json.as_bytes()).expect("No se pudo escribir el archivo");

    println!(
        "\n\nUsuario agregado correctamente.\n- ID: {}\n- Email: {}\n- Nombre: {}\n- Hash: {}\n\n",
        new_user.id, new_user.email, new_user.name, new_user.hash
    );
}


pub fn list_users() {
    let path = get_config_path();

    if !Path::new(&path).exists() {
        println!("No hay usuarios registrados todavía.");
        return;
    }

    let accounts: Vec<User> = load_accounts(&path);

    let current_user_output = Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .expect("No se pudo obtener el usuario de Git");

    let current_user_name = String::from_utf8_lossy(&current_user_output.stdout)
        .trim()
        .to_string();

    println!("\nCuentas disponibles:");

    for account in &accounts {
        if account.name == current_user_name {
            println!(
                "[{}] - Name: \x1b[92m{} (actual)\x1b[0m\n    - Email: {}\n    - Hash: {}\n",
                account.id, account.name, account.email, account.hash
            );
        } else {
            println!(
                "[{}] - Name: {}\n    - Email: {}\n    - Hash: {}\n",
                account.id, account.name, account.email, account.hash
            );
        }
    }
    
    println!("Git Actual: {}\n", current_user_name);
}

pub fn delete_account_by_id(target_id: u32) {
    let path = get_config_path();

    if !Path::new(&path).exists() {
        println!("El archivo no existe.");
        return;
    }

    let mut users: Vec<User> = load_accounts(&path);
    let original_len = users.len();

    users.retain(|u| u.id != target_id);

    if users.len() < original_len {
        let json = serde_json::to_string_pretty(&users).expect("No se pudo serializar JSON");
        let mut file = File::create(path).expect("No se pudo sobrescribir el archivo");
        file.write_all(json.as_bytes()).expect("No se pudo escribir el archivo");

        println!("Usuario con ID {} eliminado correctamente.", target_id);
    } else {
        println!("No se encontró ningún usuario con ID {}.", target_id);
    }
}
