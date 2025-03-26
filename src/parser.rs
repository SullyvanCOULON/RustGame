mod piece; // Importation du fichier piece.rs

use std::io::{self, Write};
use piece::Orientation; // Import de l'énumération Orientation

// Fonction pour gérer la commande "go"
fn go(direction: Orientation) {
    println!("Vous allez vers {:?}", direction);
}

// Fonction pour gérer la commande "back"
fn back() {
    println!("Vous retournez en arrière.");
}

// Fonction pour afficher l'aide
fn help() {
    println!("Commandes disponibles :");
    println!("  go nord    - Aller vers le nord");
    println!("  go sud     - Aller vers le sud");
    println!("  go est     - Aller vers l'est");
    println!("  go ouest   - Aller vers l'ouest");
    println!("  back       - Revenir en arrière");
    println!("  help       - Afficher l'aide");
}

// Fonction pour parser l'entrée utilisateur
fn parse_input(input: &str) {
    let mut words = input.trim().split_whitespace();
    
    let first_word = words.next();
    let second_word = words.next();

    match (first_word, second_word) {
        (Some("go"), Some("nord")) => go(Orientation::Nord),
        (Some("go"), Some("sud")) => go(Orientation::Sud),
        (Some("go"), Some("est")) => go(Orientation::Est),
        (Some("go"), Some("ouest")) => go(Orientation::Ouest),
        (Some("back"), None) => back(),
        (Some("help"), None) => help(),
        _ => println!("Commande invalide. Tapez 'help' pour voir les commandes disponibles."),
    }
}
