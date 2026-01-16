EXPERIENCE DE RUST INTERACTIF 
(v 1.5)

Mettez ce texte dans un ia pour avoir la doc interactive ‼️



--------------------------------------------------------------------------------
📖 Notes de version:

v 1.5
    Ajout d'un triche pour faire continuer le compteur et donner l'illusion que
    le programme tourne toujours.
    (il s'arrete recompile et reprend au nombre sauvegarder !)

v 1.0
    programme de base
--------------------------------------------------------------------------------






=============================================================================== 
🇫🇷 Explication :

Ici le but est d'avoir un code qu'on puisse modifier et 
qu'il se recompile à la volée, pour appliquer les modifications en temps réel.

Comme on est en Rust on utilise cargo watch (une dépendance prévue pour). 
Elle va nous permettre d'avoir cargo qui compile petit bout par petit bout 
notre code,et de surveiller les fichiers pour voir s'ils sont modifiés. 
S'ils sont modifiés, on va juste recompiler la partie concernée et la remettre 
dans le linker sans toucher aux autres parties.

Dans ce test on a un petit programme très simple, on a une boucle qui chaque 
seconde incrémente un compteur et qui affiche une valeur : 
    valeur = compteur * nombre à modifier 
Pour montrer que le script se recompile à la volée, on modifie le nombre et on 
sauvegarde. Le script va se recompiler et se relancer.
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
▶️ Comment l'exécuter :

1-  Avoir Rust sur la machine
        (installer Rust si ce n'est pas déjà fait depuis votre terminal)

2-  Installer cargo watch
        cargo install cargo-watch

3-  Télécharger mon dossier sur la machine (ce READ.me est dedans)

4-  Dans le dossier "src" du dossier,
    ouvrir le "main.rs" dans votre IDE

5-  Avec le terminal, se placer dans le dossier

6-  Lancer la commande : cargo watch -x run

7-  Pendant que le script se lance dans le terminal,
    dans votre IDE, modifier le script à l'emplacement prévu :

        // --- ZONE DE TEST : MODIFIE LA LOGIQUE ICI ---
        let resultat = compteur * 1;
        // ---------------------------------------------

    (le nombre 1 par le nombre que vous voulez, 10 par exemple)

8-  Sauvegarder le fichier et voyez le résultat
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
⚠️ Attention : ceci est une preuve de concept, en aucun cas un produit final !

Déjà, pour que ce soit implémentable dans une application, il faudrait dire au 
système que le cargo watch tourne en même temps dès qu'on lance l'appli.

Ensuite, en fonction des processeurs, de la RAM et de l'OS, la recompilation se 
fait plus ou moins rapidement ! (entre 0,5s et 5s lors de mes tests)

(Personnellement j'ai essayé de rendre le programme prioritaire avec la
commande  "nice" + de changer le linker par le linker overkill de macOS.
Je tourne entre 0,5 et 1s. Voici les cmd terminal pour le faire sur macOS 15
avec processeur x86 :

export CARGO_TARGET_DIR=/tmp/rust_target &&
export RC_TRACE_DEPENDENCIES=1 &&
export XCODE_DEVELOPER_USR_PATH=/Applications/Xcode.app/Contents/Developer/usr/bin &&
LDFLAGS="-fuse-ld=ld"
sudo nice -n -20 sudo -u $(whoami) cargo watch -x run

Cherchez les mêmes commandes qui correspondent à votre config)
===============================================================================  





=============================================================================== 
🇺🇸 Explanation: 

The goal here is to have code that can be modified and recompiled on the fly, 
applying changes in real-time.

Since we are using Rust, we use cargo watch (a dedicated dependency). It allows 
cargo to compile our code incrementally and monitor files for changes. If a change 
is detected, it only recompiles the affected part and updates the linker without 
touching other parts.

In this test, we have a very simple program, a loop that increments a counter every 
second and displays a value: 
    value = counter * number to modify To demonstrate 
on-the-fly recompilation, change the number and save the file. The script will 
recompile and restart automatically.
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
▶️ How to execute:

1-  Have Rust installed on your machine
        (install Rust via your terminal if not already done)

2-  Install cargo watch cargo install cargo-watch

3-  Download this project folder to your machine

4-  In the "src" folder, open "main.rs" in your IDE

5-  Navigate to the project folder using your terminal

6-  Run the command: cargo watch -x run

7-  While the script is running, modify the code in your IDE at the following
    location:

        // --- TEST ZONE: MODIFY LOGIC HERE ---
        let resultat = compteur * 1;
        // ------------------------------------

    (replace the number 1 with any number, e.g., 10)

8-  Save the file and watch the result in the terminal
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
⚠️ Warning: this is a proof of concept, not a final product!

To implement this in a real application, the system would need to trigger
cargo watch automatically upon launch.

Recompilation speed varies depending on your CPU, RAM, and OS!
(between 0.5s and 5s during my tests)

(I personally tried to prioritize the program using the "nice" command and 
switchingto the high-performance macOS linker. I achieved 0.5s to 1s speeds.
Here are theterminal commands for macOS 15 on x86 processors:

export CARGO_TARGET_DIR=/tmp/rust_target &&
export RC_TRACE_DEPENDENCIES=1 &&
export XCODE_DEVELOPER_USR_PATH=/Applications/Xcode.app/Contents/Developer/usr/bin &&
LDFLAGS="-fuse-ld=ld"
sudo nice -n -20 sudo -u $(whoami) cargo watch -x run

Look for similar commands that match your specific configuration)
===============================================================================