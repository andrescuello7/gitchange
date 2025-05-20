use std::fs;
use std::path::PathBuf;
use std::io::Write;
use crate::account::account::User;

pub fn load_accounts(path: &PathBuf) -> Vec<User> {
    if !path.exists() {
        // Creamos el archivo vacío con un array JSON vacío
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = fs::File::create(path).expect("No se pudo crear el archivo");
        file.write_all(b"[]").expect("No se pudo escribir en el archivo");
    }

    let data = fs::read_to_string(path).expect("No se pudo leer el archivo");
    serde_json::from_str(&data).expect("No se pudo deserializar el archivo")
}
