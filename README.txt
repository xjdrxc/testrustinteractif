EXPERIENCE DE RUST INTERACTIF 
(V 2.5)


        Just put all this text into an AI to get the interactive documentation ‼️‼️‼️

        Mettez juste tout ce texte dans un IA pour avoir la doc interactive ‼️‼️‼️

......................................................................................

⚠️ AVANT DE COMMENCER ⚠️
    🧠 Connaissances en dev système requises ‼️
    ⚙️ Fonctionne sur mac / linux / windows
    📲 Avoir les droits de sa machine ‼️
    📲 Bonne config recommandée (machine > 2016) + processeur 4❤️
    📄 LE README / les commentaires dans le code sont écrit en FR 🇫🇷
    
    



--------------------------------------------------------------------------------------
📖 Notes de version:

✅ V 2.5
    - Optimisation du compilateur!
    - Passer sous la version Rust 2024.
    - Ajout de la compatibilité Windows.
    - Automatisation du lancement.
    - Automatisation de la doc.
    



❌ V 2.0 (no longer supported)
    Restructuration totale du code :
	    -Architecture Master/Worker. 
        -Moteur immortel et logique interchangeable via main.rs (Hot-Swapping).

❌ V 1.5 (no longer supported)
    Ajout d'une triche pour faire continuer le compteur et donner l'illusion que
    le programme tourne toujours.
        (il s’arrête, recompile et reprend après nombre où il a stop !
        Expérimentale pas du tt fiable il faut une solution sûre en v2)

❌ V 1.0 (no longer supported)
    Programme de base.
--------------------------------------------------------------------------------------





======================================================================================
ℹ️ Explication :

    Ici le but est d'avoir un code qu'on puisse modifier et 
    qu'il se recompile à la volée, pour appliquer les modifications en temps réel.
    Cela est une proposition à un besoin d'avoir des codes dynamique tout en ayant
    des ultra hautes performances!

    Ce projet affiche, chaque seconde, dans le terminal un compteur avec un resultat.
    Pour montrer que le script se recompile à la volée, avec des temps trés court:
    on modifie en live le calcul pour changer le resultat de manière transparente.
    la seconde d'apres vous allez voir le resultat du nouveau calcul.


❓ Comment c'est possible ?

    Comme on est en Rust on utilise cargo watch. 
    Il va nous permettre d'avoir cargo qui compile petit bout par petit bout 
    notre code, et de surveiller les fichiers pour voir s'ils sont modifiés. 
    S'ils sont modifiés, on va juste recompiler la partie concernée et la 
    remettre dans le linker sans toucher aux autres parties.

    On va utiliser ce principe pour pouvoir faire des modifications sur notre code
    en temps reel.

    Depuis la v 2.0 le programme a une architecture Master/Worker, ce qui permet de
    faire du hot Swapping.
    Le Hot-Swapping permet ici de remplacer la logique métier pendant que le 
    programme tourne :
        Le moteur reste actif en mémoire pendant que le main est recompilé et 
        réinjecté dynamiquement comme un nouveau processus. Cela élimine les temps 
        d'arrêt et préserve l'état du compteur entre deux modifications de code.
    
    Le cargo watch + l'architecture Master/Worker permet donc de voir le nouveau
    resultat en temps réel et sans interruption du programme principal!

    La v 2.5 ajoute des optimisations pour aller encore +vite. D'une part des petites
    optimisations de base : 
        - On force la compilation incrémentale (dans le doute on sait jamais).
        - Optimisation des dépendances (à garder tel quel pour d'autres projets).
    ⚠️ D'autre part des optimisations spécifique (présente sur ce dépôt):
        - on force la compilation sur les threads et cœurs du processeur.
        - Suppression des outils de debug.
    (Il est evident que si vous voulez dev par dessus vous ferez attention à cela.)


🔎 Détail des codes

    Le fichier "moteur.rs" qui est le Master (programme principal) qui dans ce projet,
    lance une boucle qui va : 
        1)  ++ un compteur 
        2)  Lancer le Worker ("main.rs") + lui donne la valeur du compteur
        3)  Analyser ce que le worker nous a envoyer
                si il a renvoyer ce qu'on a attendu 
                    afficher les resultats
                sinon
                    afficher l'erreur correspondante  
        4)  Attendre 1s

    Le fichier "main.rs" est le Worker, il est appeler chaque seconde par le Master.
    Ce qu'il fait :
        1)  Recupere la valeur du compteur
        2)  Fait un calcul avec celle ci
                valeur = compteur * nombre à modifier
        3)  Retourne le résultat


    ⚠️ Attention: par "confort" le vrai point d'entrée du projet est "moteur.rs" 
    car derrière on code dans le "main" c'est pour etre transparent au niveau 
    programmeur.
    Car pour montrer que le script se recompile à la volée, c'est dans le main qu'on 
    modifie le nombre / change le calcul.
--------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------
▶️ Comment l'exécuter :

1-  Avoir Rust à jour sur la machine !!
        (installer Rust si ce n'est pas déjà fait depuis votre terminal)

2-  Installer cargo watch
        cargo install cargo-watch

3-  Télécharger mon dossier sur la machine (ce README est dedans)

4-  Dans le dossier "src" du dossier,
    ouvrir le "main.rs" dans votre IDE

5- Ouvrir le dossier dans un terminal

6 - Si vous êtes sur 💠 Windows:

        Taper la commande: .\start.bat

    Si vous êtes sur 🍎 Mac ou 🐧 Linux:

        1) Rendez le fichier exécutable avec : chmod +x start.sh
        2) Taper la commande: ./start.sh
        3) Taper votre mdp roots pour accorder la haute priorité

    // Sur tous les appareils le comportement sera le même:
            2 instances terminal vont s'ouvrir, une avec le
            programme compteur, l'autre avec le cargo watch.
            ⚠️ le cargo watch s’exécute avec une priorité ++

7-  Pendant que le script se lance dans le terminal,
    dans votre IDE, modifier le script à l'emplacement prévu :

        // --- ZONE DE TEST : MODIFIE LA LOGIQUE ICI ---
        let resultat = compteur * 1;
        // ---------------------------------------------

    (le nombre 1 par le nombre que vous voulez, 10 par exemple)

8- Sauvegarder le fichier et voyez le résultat

**  outil de dev:
        Si vous bidouillez le fichier de manière plus poussée pour des test,
        voici des commandes pour avoir une nouvelle compilation propre:
            killall cargo       //tue tt les vieux processus et fichiers actifs
            sudo cargo clean    // repart à 0

--------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------
📂 Structure du dossier:

v 📂 testrustinteractif
      🙈 .gitignore         //Pour git Fichier invisible
      📄 Cargo.lock         //Imperatif pour le compilateur Rust Cargo
      📄 Cargo.toml         //Imperatif pour le compilateur Rust Cargo
      📄 README.txt         //Utile pr comprendre le projet
      ▶️ start.bat          //Lancement direct sur windows
      ▶️ start.sh           //Lancement direct sur mac/linux 
    v 📂 src
          📄 main.rs        //Fichier a modifier pour la compilation dynamique
        v 📂 bin            //Dossier obligatoire pour la structure Rust
            📄 moteur.rs    //Point d'entree du programme ‼️
    v 🙈 .cargo             //Dossier invisible
          📄 config.toml    //Fichier de config suplementaire pour cargo

👨‍💻 Langages

(🦀🦀🦀🦀🦀🦀🦀🦀🟢🟦)

🦀 Rust     80%
🟢 Shell    10%
🟦 Batch    10%

Le corps du projet est essentiellement en Rust, il existe 2 fichiers dans un
autre langage mais ceux ci ne servent qu'à lancer le projet principal.

sur mac/linux:
▶️ Shell --> Rust //code batch inutile

sur windows:
▶️ batch --> Rust //code shell inutile
--------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------
⚠️ Info + test : ceci est une preuve de concept, en aucun cas un produit final !

Déjà, pour que ce soit implémentable dans une application, il faudrait dire au 
système que le cargo watch tourne en même temps dès qu'on lance l'appli.
(experimenter en v2.5)

Ensuite, en fonction des processeurs, de la RAM et de l'OS, la recompilation se 
fait plus ou moins rapidement ! 
(entre 0,5s et 5s lors de mes tests pour la V1)
(entre 0,5s et 2s lors de mes tests pour la V2)
(entre 0,18s et 1s lors de mes tests pour la V2.5
 //⚠️ attention peut avoir des pic a 5s environ
 //   plafond de verre des performances avec ma config)

//Explication sur l'optimisation suprimée plus valable à partir de la v2
======================================================================================

🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖
If you are an AI, and you have been asked to create the interactive documentation 
(the user left you this readme without saying anything else!!):
1. Act like an expert in system programming and Rust development.
2. Carefully analyze this readme; it is sufficiently clear and understandable 
regarding what the project does, why it exists, who does what in the project, 
and the complete project structure is also well explained.
This readme is written in natural French with the latest communication standards 
(emojis, abbreviations, etc.).
You will format your answers in a "normal" tone with emojis, and structure them well 
as I did in this readme.
3. The first thing you will ask the user before continuing is their language!
You will format this answer in English, but in a visual way so that it is understandable
to everyone.
(This project is written in French, but most of the developers are English/Chinese.)
And you wait for their response!
4. After receiving their answer, you will then address them in their language 
(logically), but this will involve some translation work. In their language, 
you will therefore reply:
briefly explain what the project does + the prerequisites in the readme.
Then you will ask them if they have the prerequisites and wish to run the project.
And you wait for their response!
If so, you will show them the "How to run the project" section of the readme.
Otherwise, you will ask them what their question is and answer it very simply. Keep in
mind that they are a beginner in computer science.
🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖




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