mod doors;
mod labyrinthe;
mod parser;
mod piece;

use labyrinthe::Labyrinthe;
use parser::parse_input;
use std::io::{self, Write};

/// Fonction principale du jeu.
///
/// Demande la taille du labyrinthe, initialise le jeu, puis boucle sur les commandes du joueur.
fn main() {
    let (largeur, hauteur) = demander_dimensions();
    let mut labyrinthe = Labyrinthe::new(largeur, hauteur);

    labyrinthe.afficher_piece_actuelle();
    println!();

    loop {
        println!("#### Player command");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        println!();

        parse_input(&input, &mut labyrinthe);
        println!();
    }
}

/// Demande à l'utilisateur les dimensions du labyrinthe (largeur et hauteur).
///
/// # Retour
/// Un tuple `(largeur, hauteur)` de type `(i32, i32)`
fn demander_dimensions() -> (i32, i32) {
    let largeur = lire_entier("#### Configuration\nVeuillez entrer la largeur du labyrinthe :");
    let hauteur = lire_entier("#### Configuration\nVeuillez entrer la hauteur du labyrinthe :");
    println!();
    (largeur, hauteur)
}

/// Lit un entier depuis l'entrée standard avec un message personnalisé.
///
/// # Paramètres
/// - `message` : le texte à afficher avant la lecture
///
/// # Retour
/// Un `i32` correspondant à l'entrée utilisateur
fn lire_entier(message: &str) -> i32 {
    loop {
        println!("{}", message);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(valeur) = input.trim().parse::<i32>() {
                // Vérifie que la valeur est au moins 2
                if valeur >= 2 {
                    return valeur;
                } else {
                    println!(
                        "#### Erreur\nVeuillez entrer un nombre entier supérieur ou égal à 2.\n"
                    );
                }
            } else {
                println!("#### Erreur\nVeuillez entrer un nombre entier valide.\n");
            }
        }
    }
}
