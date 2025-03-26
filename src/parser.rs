use crate::Labyrinthe;
use crate::Orientation;


/// Affiche les commandes disponibles pour l'utilisateur.
fn help() {
    println!("Commandes disponibles :");
    println!("  go nord    - Aller vers le nord");
    println!("  go sud     - Aller vers le sud");
    println!("  go est     - Aller vers l'est");
    println!("  go ouest   - Aller vers l'ouest");
    println!("  back       - Revenir en arrière");
    println!("  help       - Afficher l'aide");
}

/// Fonction pour analyser et exécuter les commandes de l'utilisateur.
///
/// Cette fonction parse l'entrée de l'utilisateur et effectue les actions appropriées
/// en fonction de la commande donnée, comme se déplacer dans le labyrinthe ou afficher l'aide.
///
/// # Arguments
/// 
/// * `input` - Une chaîne de caractères contenant la commande à analyser.
/// * `labyrinthe` - Une référence mutable à l'objet `Labyrinthe` sur lequel les actions doivent être effectuées.
///
/// # Exemples
/// 
/// ```rust
/// let mut labyrinthe = Labyrinthe::new();
/// parse_input("go nord", &mut labyrinthe);
/// parse_input("back", &mut labyrinthe);
/// ```
///
/// # Commandes valides
/// * `go nord` - Déplace le joueur vers le nord si possible.
/// * `go sud` - Déplace le joueur vers le sud si possible.
/// * `go est` - Déplace le joueur vers l'est si possible.
/// * `go ouest` - Déplace le joueur vers l'ouest si possible.
/// * `back` - Fait revenir le joueur à la dernière position.
/// * `help` - Affiche la liste des commandes disponibles.
///
/// # Panique
/// 
/// Cette fonction panique si une commande invalide est entrée. Utilisez `help` pour voir les commandes valides.
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
