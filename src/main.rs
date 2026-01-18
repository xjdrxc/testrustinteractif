//v 2.0

use std::env;

/// # Le Cerveau (Logiciel Esclave)
/// 
/// Ce programme fait partie d'une architecture Master/Worker. 
/// Il est conçu pour être appelé par le `Moteur` à chaque seconde.
///
/// ## Fonctionnement
///     1.  Récupère le compteur global envoyé par le moteur via les arguments 
///         de la ligne de commande (`args[1]`).
///
///     2.  Effectue une transformation mathématique (la logique métier).
///
///     3. Renvoie le résultat sur la sortie standard (stdout) pour que le Moteur le capture.
///
/// ## Arguments
/// * `args[1]` : Une chaîne de caractères représentant l'entier du compteur actuel.
///
/// ## Exemple d'appel manuel
/// ```bash
/// ./target/debug/testrustinteractif 10
/// ```
fn main() {
    // On récupère les arguments passés par le moteur (nom du programme + msg envoyés)
    let args: Vec<String> = env::args().collect();
    
    // Si on a reçu un chiffre (le compteur) 
    if args.len() > 1 { //si il y a plus d'un argument c forcement le compteur 
        // compteur = le compteur envoyer mais il est sous forme de texte
        // donc le deuxieme arg est converti en nombre "parse()"
        // unwrap recuperation de la valeur, si elle est pas correct = 0
        let compteur: i32 = args[1].parse().unwrap_or(0);





        // --- ZONE DE TEST : MODIFIE LA LOGIQUE ICI ---
        let resultat = compteur * 1; 
        // ---------------------------------------------

                //attention ne prend que les entiers




                
        // On affiche le résultat pour que le moteur puisse le lire
        // (ce sera repris par le moteur)
        println!("{}", resultat); 
    }
}