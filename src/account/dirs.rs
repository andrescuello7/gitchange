use std::fs;
use std::path::PathBuf;
use std::fs::{File};
use std::io::Write;

pub fn get_config_path() -> PathBuf {
    let mut path = dirs_next::home_dir().unwrap();
    path.push(".config/git-change");
    fs::create_dir_all(&path).unwrap();
    path.push("accounts.json");
    path
}

pub fn ensure_accounts_file_exists() {
    let path = get_config_path();

    // Crear carpeta si no existe
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("No se pudo crear el directorio para accounts.json");
        }
    }

    // Crear archivo vacío si no existe
    if !path.exists() {
        let mut file = File::create(&path).expect("No se pudo crear accounts.json");
        file.write_all(b"[]").expect("No se pudo escribir en accounts.json");
        println!("\n- Archivo accounts.json creado vacío en \n~/.config/git-change/.");
    }
}