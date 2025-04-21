use inquire::{Text, Select};
use std::fs;
use std::path::Path;
use include_dir::{include_dir, Dir};

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

fn ask_for_credentials() -> (String, String) {
    let client_id = Text::new("Client ID:").prompt()
        .expect("[synapse-quickstart] Erreur lors de la saisie du Client ID.");
    let client_secret = Text::new("Client Secret:").prompt()
        .expect("[synapse-quickstart] Erreur lors de la saisie du Client Secret.");
    (client_id, client_secret)
}

fn list_embedded_templates(dir: &Dir) -> Vec<String> {
    dir.dirs().map(|d| d.path().file_name().unwrap().to_string_lossy().to_string()).collect()
}

fn select_template(label: &str, templates: Vec<String>) -> String {
    Select::new(label, templates)
        .prompt()
        .expect("[synapse-quickstart] Erreur pendant la sélection de template.")
}

fn copy_embedded_template(template_dir: &Dir, target: &str) {
    for entry in template_dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                let sub_target = format!("{}/{}", target, subdir.path()
                    .file_name().unwrap().to_string_lossy());
                fs::create_dir_all(&sub_target).unwrap();
                copy_embedded_template(subdir, &sub_target);
            }
            include_dir::DirEntry::File(file) => {
                let path = format!("{}/{}", target, file.path()
                    .file_name().unwrap().to_string_lossy());
                fs::write(&path, file.contents()).unwrap();
            }
        }
    }
}

fn write_env(dst: &str, client_id: &str, client_secret: &str) {
    let env_path = format!("{}/.env", dst);
    let contents = format!("SYNAPSE_ID={}\nSYNAPSE_SECRET={}\n", client_id, client_secret);

    fs::write(&env_path, contents)
        .unwrap_or_else(|e| panic!("[synapse-quickstart] Impossible d’écrire dans '{}' : {}", env_path, e));
}

fn print_instructions(dst: &str) {
    println!("\n[synapse-quickstart] Projet généré dans '{}'", dst);
    println!("[synapse-quickstart]  Pour démarrer :");
    println!("[synapse-quickstart]\tOuvre {}/README.md et suis les instructions", dst);
}

fn main() {
    let dst = Text::new("Destination du projet :")
        .prompt()
        .expect("[synapse-quickstart] Erreur pendant la saisie.");

    
    if Path::new(&dst).exists() {
        let overwrite = Select::new(
            "[synapse-quickstart] Le dossier existe déjà. Veux-tu le remplacer ?", 
            vec!["Oui", "Non"]
        ).prompt().expect("[synapse-quickstart] Erreur pendant la sélection.");

        if overwrite == "Non" {
            println!("[synapse-quickstart] Initialisation du projet abandonnée.");
            return;
        }
    }

    let backend_dir = TEMPLATES_DIR.get_dir("backend").unwrap();
    let frontend_dir = TEMPLATES_DIR.get_dir("frontend").unwrap();

    let selected_backend = select_template("Choisis un backend :", list_embedded_templates(backend_dir));
    let selected_frontend = select_template("Choisis un frontend :", list_embedded_templates(frontend_dir));

    let backend_template_dir = TEMPLATES_DIR.get_dir(&format!("backend/{}", selected_backend)).unwrap();
    let frontend_template_dir = TEMPLATES_DIR.get_dir(&format!("frontend/{}", selected_frontend)).unwrap();    

    let backend_target_dir = &dst;
    let frontend_target_dir = format!("{}/client", dst);

    fs::create_dir_all(&backend_target_dir)
        .expect("[synapse-quickstart] Erreur lors de la création du dossier serveur ");
    fs::create_dir_all(&frontend_target_dir)
        .expect("[synapse-quickstart] Erreur lors de la création du dossier client ");

    let (client_id, client_secret) = ask_for_credentials();
    copy_embedded_template(backend_template_dir, &backend_target_dir);
    copy_embedded_template(frontend_template_dir, &frontend_target_dir);
    write_env(&backend_target_dir, &client_id, &client_secret);

    print_instructions(&dst);
}