/// Module principal qui gère les pièces et le labyrinthe.
mod parser;
mod piece;

use std::collections::{HashMap, VecDeque};
use piece::{Piece, Orientation, TypePiece};
use parser::parse_input;
use std::io::{self, Write};

/// Structure représentant le labyrinthe.
///
/// Le labyrinthe contient une collection de pièces, la position actuelle du joueur,
/// et un historique des positions précédentes du joueur.
struct Labyrinthe {
    /// Map des coordonnées des pièces, où chaque clé est une paire de coordonnées `(x, y)` 
    /// et la valeur correspond à une pièce.
    pieces: HashMap<(i32, i32), Piece>,
    
    /// La position actuelle du joueur dans le labyrinthe.
    position: (i32, i32),
    
    /// Historique des positions précédentes du joueur. Utilisé pour la commande 'revenir'.
    historique: VecDeque<(i32, i32)>,
}

impl Labyrinthe {
    /// Crée un nouveau labyrinthe avec des pièces de départ, de fin et normales.
    ///
    /// Cette fonction initialise les pièces du labyrinthe en ajoutant des coordonnées et des
    /// types de pièces à la collection `pieces`. Elle initialise également la position du joueur
    /// au point de départ `(0, 0)` et un historique vide.
    ///
    /// # Retour
    ///
    /// Retourne une nouvelle instance de `Labyrinthe`.
    fn new() -> Self {
        let mut pieces = HashMap::new();
        
        // Création des pièces avec leurs orientations respectives
        pieces.insert((0, 0), Piece::new(TypePiece::Debut, vec![Orientation::Est]));
        pieces.insert((1, 0), Piece::new(TypePiece::Normal, vec![Orientation::Ouest, Orientation::Est]));
        pieces.insert((2, 0), Piece::new(TypePiece::Fin, vec![Orientation::Ouest]));
        
        Self {
            pieces,
            position: (0, 0),
            historique: VecDeque::new(),
        }
    }

    /// Affiche la pièce actuelle dans le labyrinthe.
    ///
    /// Cette fonction récupère la pièce correspondant à la position actuelle du joueur
    /// et l'affiche. Si la pièce n'existe pas, elle affiche un message indiquant un endroit inconnu.
    fn afficher_piece_actuelle(&self) {
        if let Some(piece) = self.pieces.get(&self.position) {
            piece.afficher();
        } else {
            println!("Vous êtes dans un endroit inconnu.");
        }
    }

    /// Déplace le joueur dans le labyrinthe dans la direction spécifiée.
    ///
    /// Avant de se déplacer, cette fonction empile la position actuelle dans l'historique
    /// du joueur. Ensuite, elle vérifie si une porte dans la direction spécifiée existe
    /// et permet le déplacement.
    ///
    /// # Arguments
    ///
    /// * `direction` - La direction vers laquelle le joueur souhaite se déplacer (Nord, Sud, Est, Ouest).
    fn deplacer(&mut self, direction: Orientation) {
        // Avant de se déplacer, on empile la position actuelle
        self.historique.push_back(self.position);
        
        let (x, y) = self.position;
        let nouvelle_position = match direction {
            Orientation::Nord => (x, y - 1),
            Orientation::Sud => (x, y + 1),
            Orientation::Est => (x + 1, y),
            Orientation::Ouest => (x - 1, y),
        };

        if let Some(piece) = self.pieces.get(&nouvelle_position) {
            if piece.orientations.contains(&direction) {
                self.position = nouvelle_position;
                self.afficher_piece_actuelle();
            } else {
                println!("Il n'y a pas de porte par là !");
            }
        } else {
            println!("Vous ne pouvez pas aller dans cette direction.");
        }
    }

    /// Permet au joueur de revenir à sa position précédente.
    ///
    /// Cette fonction vérifie si l'historique contient une position précédente.
    /// Si c'est le cas, elle ramène le joueur à cette position. Sinon, elle affiche
    /// un message indiquant qu'il n'y a pas de position précédente.
    fn revenir(&mut self) {
        // Si l'historique contient une position précédente, on revient à cette position
        if let Some(position_precedente) = self.historique.pop_back() {
            self.position = position_precedente;
            self.afficher_piece_actuelle();
        } else {
            println!("Il n'y a pas de position précédente. Vous êtes déjà à l'origine.");
        }
    }
}

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
