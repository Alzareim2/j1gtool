use std::fs::File;
use std::io::prelude::*;
use serde_json::json;

pub fn is_first_launch() -> bool {
    !file_exists("config.json")
}

pub fn create_files_with_content() {
    if !file_exists("config.json") {
        create_file_with_content("config.json", &json!({
            "api_key": ""

        }));
    }

    
}


fn file_exists(filename: &str) -> bool {
    std::path::Path::new(filename).exists()
}

fn create_file_with_content(filename: &str, content: &serde_json::Value) {
    match File::create(filename) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.to_string().as_bytes()) {
                println!("Erreur lors de l'écriture du contenu dans {}: {}", filename, e);
            } else {
                println!("Le fichier {} a été créé avec succès.", filename);
            }
        }
        Err(e) => println!("Erreur lors de la création du fichier {}: {}", filename, e),
    }
}

