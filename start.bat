@echo off
REM Lance cargo watch dans un nouveau terminal avec haute priorité
start "Cargo Watch" /HIGH cmd /k "cargo watch -x build"

REM Attend 3 secondes que cargo watch démarre
timeout /t 3 /nobreak >nul

REM Lance le moteur dans le terminal actuel
echo 🚀 Lancement du moteur...
cargo run --bin moteur
