use std::time::Duration;
use std::thread;

fn main() {
    let mut compteur = 0;

    loop {
        compteur += 1;
        
        // --- ZONE DE TEST : MODIFIE LA LOGIQUE ICI ---
        let resultat = compteur * 1;
        // ---------------------------------------------
        
        println!("Itération {} : Le résultat est {}", compteur, resultat);
        
        thread::sleep(Duration::from_secs(1));
    }
}