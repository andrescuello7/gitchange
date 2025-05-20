mod account;
use account::account::{User, add_user_to_file, delete_account_by_id, list_users, select_user};
use account::dirs::{get_config_path, ensure_accounts_file_exists};
use account::loader::load_accounts;

use std::io::{self, Write};
use std::process::Command;


// Args of the inputs for recibe commands
pub fn args_write() -> String {
    // Socket of linten events
    io::stdout().flush().unwrap();

    // Mut varible declarate for case of change
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    return input;
}

// Process for run command in system
pub fn run_process(input: String) -> String {
    let mut parts = input.trim().split_whitespace();
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let output = Command::new(cmd)
            .args(&args)
            .output()
            .expect("error in process!");

        // In case of Error
        if output.stdout.is_empty() {
            return String::from_utf8_lossy(&output.stderr).to_string();
        }

        // Return case of success
        return String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        // in case of null command of string input
        return "Don't write commands.".to_string();
    }
}

fn main() {
    ensure_accounts_file_exists();
    println!(
        "\nGIT_CHANGE - Gestion of accounts\n1) Change Account \n2) View Accounts \n3) Add Account \n4) Delete Account\n"
    );
    let options = args_write().trim().to_string();


    if options == "1" {
        list_users();
        let input = args_write().trim().to_string();
        let user: User = select_user(input);

        // TODO i18n: validation of options
        if user.name == String::from("") || user.email == String::from("") {
            return;
        }

        // Run process for configs for config
        run_process(format!("git config --global user.name \"{}\"", user.name));
        run_process(format!("git config --global user.email \"{}\"", user.email));
        run_process(format!("git config --global user.hash \"{}\"", user.hash));

        println!(
            "\nCuenta Cambiada por:\nname: {}email: {}",
            run_process(String::from("git config --global user.name")),
            run_process(String::from("git config --global user.email"))
        );
    } else if options == "2" {
        list_users();
    } else if options == "3" {
        println!("\nAgregar Cuenta:");
        print!("Email: ");
        io::stdout().flush().unwrap();
        let email = args_write().trim().to_string();
    
        print!("UserName: ");
        io::stdout().flush().unwrap();
        let name = args_write().trim().to_string();
    
        print!("Hash Token: ");
        io::stdout().flush().unwrap();
        let hash = args_write().trim().to_string();
    
        // Cargar cuentas para obtener el próximo id
        let path = get_config_path();
        let users: Vec<User> = if std::path::Path::new(&path).exists() {
            load_accounts(&path)
        } else {
            Vec::new()
        };
    
        let new_id = users.iter().map(|u| u.id).max().unwrap_or(0) + 1;
    
        let new_user = User {
            id: new_id,
            email,
            name,
            hash,
        };
    
        add_user_to_file(new_user);
    } else if options == "4" {
        list_users();
        print!("ID: ");
        io::stdout().flush().unwrap();
        let id_account_input = args_write().trim().to_string();

        let id_account: u32 = match id_account_input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("ID inválido, debe ser un número entero.");
                return;
            }
        };
        delete_account_by_id(id_account);

        println!("\nCuenta Eliminada");
    } else {
        println!("Error 400 - No selecciono una opcion correcta");
    }
}
