use std::process::Command;
use std::thread::sleep;
use std::time::Duration;



/// # Le Moteur (Logiciel Maître)
///
/// Ce programme gère le temps et la persistance des données.
/// Il appelle le binaire `Cerveau` à chaque itération.
///
/// ## Responsabilités
/// 1. Incrémenter le compteur global (persistance d'état).
/// 2. Lancer un nouveau processus pour le calcul (Hot Reload).
/// ce processus c'est le "main.rs"
/// 3. Capturer et afficher les résultats ou les erreurs du calcul.
fn main() {
    let mut compteur = 0; //setup le compteur a 0

    // On définit le chemin complet vers le binaire du cerveau
    let cerveau_path = "./target/debug/testrustinteractif";

    //on affiche que c ok
    println!("🚀 Moteur TEMPS RÉEL lancé.");

    loop {

        compteur += 1; //à chaque seconde compteur +1



        // On prépare la commande système pour lancer le cerveau
        // .arg(...) transforme notre nombre en texte pour l'envoyer au cerveau
        // .output() lance le programme et attend qu'il finisse pour récupérer le texte
        let output = Command::new(cerveau_path)
            .arg(compteur.to_string())
            .output();

        // On analyse le résultat du lancement du programme
        match output {

                // Cas où le programme s'est lancé correctement
                Ok(out) => {

                    // out.stdout = ce que le cerveau a écrit avec println!
                    // from_utf8_lossy = convertit les octets (bytes) en texte lisible
                    // .trim() = enlève les espaces et les retours à la ligne inutiles
                    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();

                    // out.stderr = contient les messages d'erreur si le cerveau a planté
                    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();



                    if !stdout.is_empty() {
                        // Si le cerveau a renvoyé du texte, on l'affiche
                        println!("Temps: {}s | Résultat: {}", compteur, stdout);
                    } else if !stderr.is_empty() {
                        //cas erreur 2: le cerveau a renvoyé une erreur Rust (panique)
                        println!("Temps: {}s | ❌ Erreur Rust: {}", compteur, stderr);
                    } else {
                        //cas erreur 3: le cerveau s'est lancé mais n'a rien écrit du tout
                        println!("Temps: {}s | ⏳ Cerveau muet (vérifie println! dans main.rs)", compteur);
                    }

                }

                //cas erreur 1: le fichier binaire n'existe pas (erreur de chemin ou compilation en cours)
                Err(e) => {
                    println!("Temps: {}s | ⚠️ Binaire introuvable: {}", compteur, e);
                }
            }



        sleep(Duration::from_secs(1)); //pause 1s avant le prochain tour

    }

}
