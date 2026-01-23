#!/bin/bash

# Ouvre un nouveau terminal qui lance cargo watch avec nice -15
osascript -e 'tell app "Terminal"
    do script "cd \"'"$(pwd)"'\" && sudo nice -15 cargo watch -x build"
end tell'

# Attend 2 secondes que cargo watch démarre
sleep 2

# Lance le moteur dans le terminal actuel
echo "🚀 Lancement du moteur..."
cargo run --bin moteur
