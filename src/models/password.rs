use aes::cipher::{generic_array, typenum};
use serde::{Serialize, Deserialize};
use std::result::Result;
use std::fs;
use std::path::Path;
use aes_gcm::aead::{Aead, KeyInit};   // Pour le chiffrement et déchiffrement
use rand::RngCore;                    // Pour générer des bytes aléatoires
use aes_gcm::{Aes256Gcm, Nonce};
use generic_array::GenericArray;
use typenum::{U12, U32}; // Pour spécifier les tailles

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub name: String,
    pub username_or_email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntryManager {
    pub entries : Vec<PasswordEntry>,
}

impl PasswordEntryManager {
    pub fn new() -> Self {
        PasswordEntryManager { entries: Vec::new() }
    }

    pub fn load_from_file(file_path: &str, master_password: &str) -> Result<PasswordEntryManager, String> {
        if Path::new(file_path).exists() {
            let encrypted_data = fs::read_to_string(file_path).map_err(|_| "Impossible de lire le fichier.")?;
            let decrypted_data = PasswordEntryManager::decrypt_data(&encrypted_data, master_password)?;
            let password_manager: PasswordEntryManager = serde_json::from_str(&decrypted_data).map_err(|_| "Données invalides.")?;
            Ok(password_manager)
        } else {
            Ok(PasswordEntryManager::new())
        }
    }
    
    pub fn save_to_file(&self, file_path: &str, master_password: &str) -> Result<(), String> {
        let json_data = serde_json::to_string_pretty(self).map_err(|_| "Erreur lors de la sérialisation.")?;
        let encrypted_data = PasswordEntryManager::encrypt_data(&json_data, master_password)?;
        fs::write(file_path, encrypted_data).map_err(|_| "Impossible d'écrire dans le fichier.")?;
        Ok(())
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }

    fn encrypt_data(data: &str, password: &str) -> Result<String, String> {
        let key = PasswordEntryManager::derive_key(password);
        let cipher = Aes256Gcm::new(&key);
        let nonce = PasswordEntryManager::generate_nonce(); // 12 octets aléatoires
        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|_| "Erreur lors du chiffrement.")?;
        let mut result = base64::encode(nonce);
        result.push(':');
        result.push_str(&base64::encode(ciphertext));
        Ok(result)
    }

    fn decrypt_data(data: &str, password: &str) -> Result<String, String> {
        let key = PasswordEntryManager::derive_key(password);
        let cipher = Aes256Gcm::new(&key);
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 2 {
            return Err("Données chiffrées invalides.".to_string());
        }
        let nonce = base64::decode(parts[0]).map_err(|_| "Nonce invalide.")?;
        let ciphertext = base64::decode(parts[1]).map_err(|_| "Texte chiffré invalide.")?;
        let plaintext = cipher
            .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
            .map_err(|_| "Erreur lors du déchiffrement.")?;
        String::from_utf8(plaintext).map_err(|_| "Données déchiffrées invalides.".to_string())
    }

// Fonction pour dériver une clé à partir d'un mot de passe
    fn derive_key(password: &str) -> GenericArray<u8, U32> {
        let mut key = [0u8; 32]; // Taille de clé AES-256
        let password_bytes = password.as_bytes();
        
        for (i, &byte) in password_bytes.iter().enumerate().take(32) {
            key[i] = byte;
        }

        // Retourner la clé comme un GenericArray de taille U32 (32 octets)
        *GenericArray::from_slice(&key)
    }

    fn generate_nonce() -> Nonce<U12> {
        let mut rng = rand::thread_rng();
        
        // Crée un GenericArray de taille U12
        let mut nonce = GenericArray::<u8, U12>::default(); 
    
        // Remplir nonce avec des octets aléatoires
        rng.fill_bytes(&mut nonce);
    
        // Convertir en Nonce et déférencer
        *Nonce::from_slice(&nonce)
    }
}
