#!/bin/bash

# Répertoire parent contenant les dossiers à nettoyer
parent_directory="/home/cheibany/Documents/piscine-rust/solutions/Exam-Rust/"
# Parcours de tous les dossiers dans le répertoire parentt
find "$parent_directory" -type d -name "target" -exec sh -c '(cd "{}" && cargo clean)' \;
    if [ -d "$folder" ]; then
        echo "Nettoyage du dossier : $folder"
        # Se déplacer dans le dossier et exécuter cargo clean
        (cd "$folder" && cargo clean)
    fi
