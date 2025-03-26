mod parser;
mod piece;

use std::collections::{HashMap, VecDeque};
use piece::{Piece, Orientation, TypePiece};
use parser::parse_input;
use std::io::{self, Write};


struct Labyrinthe {
    pieces: HashMap<(i32, i32), Piece>, // Stocke les pièces avec des coordonnées
    position: (i32, i32),               // Position actuelle du joueur
    historique: VecDeque<(i32, i32)>,   // Historique des positions du joueur
}

impl Labyrinthe {
    fn new() -> Self {
        let mut pieces = HashMap::new();
        
        // Création des pièces
        pieces.insert((0, 0), Piece::new(TypePiece::Debut, vec![Orientation::Est]));
        pieces.insert((1, 0), Piece::new(TypePiece::Normal, vec![Orientation::Ouest, Orientation::Est]));
        pieces.insert((2, 0), Piece::new(TypePiece::Fin, vec![Orientation::Ouest]));
        
        Self {
            pieces,
            position: (0, 0),
            historique: VecDeque::new(),
        }
    }

    fn afficher_piece_actuelle(&self) {
        if let Some(piece) = self.pieces.get(&self.position) {
            piece.afficher();
        } else {
            println!("Vous êtes dans un endroit inconnu.");
        }
    }

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


fn main() {
    let mut labyrinthe = Labyrinthe::new();
    labyrinthe.afficher_piece_actuelle();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        parse_input(&input, &mut labyrinthe);
    }
}
