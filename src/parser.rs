use crate::piece::Orientation; // Import correct de l'énumération Orientation
use crate::Labyrinthe;

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


pub fn parse_input(input: &str, labyrinthe: &mut Labyrinthe) {
    let mut words = input.trim().split_whitespace();
    let first_word = words.next();
    let second_word = words.next();

    match (first_word, second_word) {
        (Some("go"), Some("nord")) => {
            labyrinthe.deplacer(Orientation::Nord);
        }
        (Some("go"), Some("sud")) => {
            labyrinthe.deplacer(Orientation::Sud);
        }
        (Some("go"), Some("est")) => {
            labyrinthe.deplacer(Orientation::Est);
        }
        (Some("go"), Some("ouest")) => {
            labyrinthe.deplacer(Orientation::Ouest);
        }
        (Some("back"), None) => labyrinthe.revenir(),
        (Some("help"), None) => help(),
        _ => println!("Commande invalide. Tapez 'help' pour voir les commandes disponibles."),
    }
}
