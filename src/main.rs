/// Module principal qui gère les pièces et le labyrinthe.
mod parser;
mod labyrinthe;
mod piece;

use parser::parse_input;
use std::io::{self, Write};
use labyrinthe::Labyrinthe;
use piece::{Piece, Orientation, TypePiece};

/// Fonction principale du jeu.
///
/// La fonction `main` initialise le labyrinthe, affiche la première pièce et
/// entre ensuite dans une boucle où elle attend l'entrée de l'utilisateur.
fn main() {
    let mut labyrinthe = Labyrinthe::new();
    labyrinthe.afficher_piece_actuelle();

    loop {
        // Demander à l'utilisateur une commande
        print!("> ");
        io::stdout().flush().unwrap();  // Assurer que le prompt soit affiché
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Traiter la commande de l'utilisateur
        parse_input(&input, &mut labyrinthe);
    }
}
