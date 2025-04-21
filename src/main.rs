use inquire::{Text, Select};
use std::fs;
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};

fn ask_for_credentials() -> (String, String) {
    let client_id = Text::new("Client ID:").prompt().expect("Erreur lors de la saisie du Client ID.");
    let client_secret = Text::new("Client Secret:").prompt().expect("Erreur lors de la saisie du Client Secret.");
    (client_id, client_secret)
}

fn list_templates(dir: &str) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("Impossible de lire le dossier des templates '{}'", dir))
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    e.file_name().into_string().ok()
                } else {
                    None
                }
            })
        })
        .collect()
}

fn select_template(label: &str, templates: Vec<String>) -> String {
    Select::new(label, templates)
        .prompt()
        .expect("Erreur pendant la sélection de template.")
}

fn copy_template(template: &str, dst: &str) {
    let mut options = CopyOptions::new();
    options.copy_inside = true;

    let template_path = Path::new(template);
    if !template_path.exists() {
        panic!("Le template '{}' n'existe pas.", template);
    }

    let entries: Vec<_> = fs::read_dir(template_path)
        .unwrap_or_else(|e| panic!("Erreur lors de la lecture du dossier '{}': {}", template, e))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();

        fs_extra::copy_items(&entries, dst, &options)
            .unwrap_or_else(|e| panic!("Erreur lors de la copie du template : {}", e));
}

fn write_env(dst: &str, client_id: &str, client_secret: &str) {
    let env_path = format!("{}/.env", dst);
    let contents = format!("CLIENT_ID={}\nCLIENT_SECRET={}\n", client_id, client_secret);

    fs::write(&env_path, contents)
        .unwrap_or_else(|e| panic!("Impossible d’écrire dans '{}' : {}", env_path, e));
}

fn print_instructions(dst: &str) {
    println!("\n✅ Projet généré dans '{}'", dst);
    println!("➡️  Pour démarrer :");
    println!("   Ouvre {}/README.md et suis les instructions", dst);
}

fn main() {
    // Choix de la destination
    let dst = Text::new("Destination du projet :")
        .prompt()
        .expect("Erreur pendant la saisie.");

    if Path::new(&dst).exists() {
        eprintln!("❌ Le dossier de destination '{}' existe déjà.", dst);
        return;
    }

    // Récupération des templates
    let backend_templates = list_templates("templates/backend");
    let frontend_templates = list_templates("templates/frontend");

    // Sélection via menu
    let selected_backend = select_template("Choisis un backend :", backend_templates);
    let selected_frontend = select_template("Choisis un frontend :", frontend_templates);

    // Création des chemins
    let template_server_path = format!("templates/backend/{}", selected_backend);
    let template_client_path = format!("templates/frontend/{}", selected_frontend);
    let target_server_path = &dst;
    let target_client_path = format!("{}/client", dst);

    // Création des dossiers
    fs::create_dir_all(&target_server_path).expect("Erreur lors de la création du dossier serveur ");
    fs::create_dir_all(&target_client_path).expect("Erreur lors de la création du dossier client ");

    // Génération des fichiers
    let (client_id, client_secret) = ask_for_credentials();
    copy_template(&template_server_path, &target_server_path);
    copy_template(&template_client_path, &target_client_path);
    write_env(&target_server_path, &client_id, &client_secret);

    print_instructions(&dst);
}
