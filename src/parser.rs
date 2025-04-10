use crate::Labyrinthe;
use crate::piece::Orientation;
/// 

/// Affiche la liste des commandes disponibles.
fn help() {
    println!("Commandes disponibles :");
    println!("  go n     - Aller vers le nord");
    println!("  go s     - Aller vers le sud");
    println!("  go e     - Aller vers l'est");
    println!("  go o     - Aller vers l'ouest");
    println!("  back     - Revenir en arrière");
    println!("  help     - Afficher l'aide");
    println!("  quit     - Quitter le jeu\n");
}

/// Analyse et exécute une commande texte entrée par l'utilisateur.
///
/// # Paramètres
/// - `input`: chaîne entrée par l'utilisateur.
/// - `labyrinthe`: référence mutable vers le labyrinthe actuel.
pub fn parse_input(input: &str, labyrinthe: &mut Labyrinthe) {
    let mut words = input.trim().split_whitespace();
    let first_word = words.next();
    let second_word = words.next();

    match (first_word, second_word) {
        (Some("go"), Some("n")) => labyrinthe.deplacer(Orientation::N),
        (Some("go"), Some("s")) => labyrinthe.deplacer(Orientation::S),
        (Some("go"), Some("e")) => labyrinthe.deplacer(Orientation::E),
        (Some("go"), Some("o")) => labyrinthe.deplacer(Orientation::O),
        (Some("back"), None) => labyrinthe.revenir(),
        (Some("help"), None) => help(),
        (Some("quit"), None) => {
            println!("#### Quit");
            println!("Merci d’avoir joué !");
            std::process::exit(0);
        },
        _ => println!("Commande invalide. Tapez 'help' pour voir les commandes disponibles.")
    }
}
