use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;  // Pour récupérer les variables d'environnement
use crate::models::password::{PasswordEntryManager, PasswordEntry};  // Importer la struct et les méthodes
use std::fs::{self, File};
use std::path::Path;

pub fn start() {
    // Charger les variables d'environnement depuis le fichier .env
    dotenv().ok();

    // Récupérer le mot de passe maître à partir de la variable d'environnement
    let master_password = env::var("MASTER_PASSWORD").expect("MASTER_PASSWORD non défini dans le fichier .env");
    let file_path = "passwords.json";

    // Vérifier si le fichier existe, sinon en créer un
    if !Path::new(file_path).exists() {
        // Créer le fichier si nécessaire
        File::create(file_path).expect("Impossible de créer le fichier passwords.json");
    }

    let matches = Command::new("rkp")
        .version("1.0")
        .author("Liam Esteffe")
        .about("Gestionnaire de mot de passe")
        .subcommand(Command::new("add")
            .about("Ajouter un mot de passe")
            .arg(Arg::new("name").required(true).help("Nom du site"))
            .arg(Arg::new("username_or_email").required(true).help("Nom d'utilisateur ou email"))
            .arg(Arg::new("password").required(true).help("Mot de passe")))
        .subcommand(Command::new("list").about("Lister les mots de passe"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        // Récupérer les arguments de la commande add
        let name = matches.get_one::<String>("name").unwrap();
        let username_or_email = matches.get_one::<String>("username_or_email").unwrap();
        let password = matches.get_one::<String>("password").unwrap();

        // Charger le gestionnaire de mots de passe
        let mut manager = PasswordEntryManager::load_from_file(file_path, &master_password).unwrap_or_else(|_| PasswordEntryManager::new());

        // Créer une nouvelle entrée de mot de passe
        let entry = PasswordEntry {
            name: name.clone(),
            username_or_email: username_or_email.clone(),
            password: password.clone(),
        };

        // Ajouter l'entrée au gestionnaire
        manager.add_entry(entry);

        // Sauvegarder les mots de passe dans le fichier
        manager.save_to_file(file_path, &master_password).expect("Échec de la sauvegarde des mots de passe");

        println!("Mot de passe ajouté : {}, {}, {}", name, username_or_email, password);
    } else if let Some(_) = matches.subcommand_matches("list") {
        // Charger le gestionnaire de mots de passe
        let manager = PasswordEntryManager::load_from_file(file_path, &master_password).unwrap_or_else(|_| PasswordEntryManager::new());

        // Afficher les mots de passe
        if manager.entries.is_empty() {
            println!("Aucune entrée disponible.");
        } else {
            for entry in &manager.entries {
                println!(
                    "Nom : {}, Utilisateur : {}, Mot de passe : {}",
                    entry.name, entry.username_or_email, entry.password
                );
            }
        }
    } else {
        println!("Commande invalide. Utilisez `add` ou `list`.");
    }
}