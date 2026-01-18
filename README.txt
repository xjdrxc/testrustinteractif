EXPERIENCE DE RUST INTERACTIF 
(V 2.0)

⚠️ AVANT DE COMMENCER ⚠️
    - LE README / les commentaires dans le code sont écrit en Fr 🇫🇷
    - Fonctionne actuellement sur mac et linux (🚫 Windows)
    - Mettez ce texte dans un ia pour avoir la doc interactive ‼️
    



--------------------------------------------------------------------------------
📖 Notes de version:

✅ V 2.0
    Restructuration totale du code !
	    -Architecture Master/Worker. 
        -Moteur immortel et logique interchangeable via main.rs (Hot-Swapping).

❌ V 1.5 
    Ajout d'une triche pour faire continuer le compteur et donner l'illusion que
    le programme tourne toujours.
        (il s'arrete recompile et reprend aprés nombre où il a stop !
        experimentale pas du tt fiable il faut une solution sur en v2)

❌ V 1.0
    programme de base 
--------------------------------------------------------------------------------






================================================================================ 
🇫🇷 Explication :

Ici le but est d'avoir un code qu'on puisse modifier et 
qu'il se recompile à la volée, pour appliquer les modifications en temps réel.

Comme on est en Rust on utilise cargo watch (une dépendance prévue pour). 
Elle va nous permettre d'avoir cargo qui compile petit bout par petit bout 
notre code,et de surveiller les fichiers pour voir s'ils sont modifiés. 
S'ils sont modifiés, on va juste recompiler la partie concernée et la remettre 
dans le linker sans toucher aux autres parties.

Le but ici est très simple, on a une boucle qui chaque 
seconde incrémente un compteur et qui affiche une valeur : 
    valeur = compteur * nombre à modifier 
Pour montrer que le script se recompile à la volée, on modifie le nombre et on 
sauvegarde. Le script va se recompiler et se relancer.

La V 2.0 restructure totalement le code !
Ici nous avons toujours cargo watch qui surveille en permanence les modifications 
du dossier et recompile si besoin.
au-dessus du cargo watch, nous executons un moteur "moteur.rs" (qui est le vrai 
programme principal), qui va se charger d'appeler un processus.
C'est ce processus qu'on va modifier et qui va induire les modifications 
en temps réel et sans interruption du programme principale!
(Attention par "confort" le vrai point d'entree du projet est "moteur.rs" car 
derriere on code ce qu'on veut dans le main c'est pour etre transparant au 
niveau programmeur)
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

5-  Avec une premiere instance terminal, se placer dans le dossier

6-  Lancer la commande : cargo watch -x build
    (cette cmd ne redonne pas la main a l'utilisateur)

7-  Avec une seconde instance terminal, se placer dans le dossier

8-  Lancer la commande : cargo run --bin moteur

9-  Pendant que le script se lance dans le terminal,
    dans votre IDE, modifier le script à l'emplacement prévu :

        // --- ZONE DE TEST : MODIFIE LA LOGIQUE ICI ---
        let resultat = compteur * 1;
        // ---------------------------------------------

    (le nombre 1 par le nombre que vous voulez, 10 par exemple)

10- Sauvegarder le fichier et voyez le résultat
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
📂 Structure du dossier:

v 📂 testrustinteractif 
      📄 Cargo.lock         //imperatif pour le compilateur Rust Cargo
      📄 Cargo.toml         //imperatif pour le compilateur Rust Cargo
      📄 README.txt         //utile pr comprendre le projet
    v 📂 src
          📄 main.rs        //fichier a modifier pour la compilation dynamique
        v 📂 bin            //fichier obligatoire pour la structure Rust
            📄 moteur.rs    //Point d'entree du processus ‼️
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
⚠️ Info + test : ceci est une preuve de concept, en aucun cas un produit final !

Déjà, pour que ce soit implémentable dans une application, il faudrait dire au 
système que le cargo watch tourne en même temps dès qu'on lance l'appli.

Ensuite, en fonction des processeurs, de la RAM et de l'OS, la recompilation se 
fait plus ou moins rapidement ! 
(entre 0,5s et 5s lors de mes tests pour la V1)
(entre 0,5s et 2s lors de mes tests pour la V2)

//Explication sur l'optimisation suprimer plus valable en V2
================================================================================





Git :
Il manque cruellement des outils pour nettoyer l’historique simplement ‼️
Imaginez un vieux commit contenant une faille de sécurité critique : vous n’avez
aucun moyen de le faire disparaître proprement sans affecter le reste du projet.
Pour le supprimer, vous perdez une journée à vous battre avec le terminal, Git 
corrompt votre historique au passage, pour finir par bricoler des solutions
manuelles.

Git est une mini-blockchain, pas l’outil de versioning interactif qu’on veut 
nous vendre.

Il est temps de repenser sa logique pour qu’il soit vraiment utile.