use std::fs;

/// Enumération des orientations possibles des portes.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Orientation {
    N,
    S,
    E,
    O,
}

/// Enumération des types de pièces du labyrinthe.
#[derive(Debug)]
pub enum TypePiece {
    Debut,
    Fin,
    Normal,
}

/// Structure représentant une pièce du labyrinthe.
#[derive(Debug)]
pub struct Piece {
    pub orientations: Vec<Orientation>,
    pub typage_piece: TypePiece,
}

impl Piece {
    /// Crée une pièce avec un type et une liste d'orientations valides.
    ///
    /// # Panique
    /// Si le nombre de portes est incorrect selon le type de pièce.
    pub fn new(typage_piece: TypePiece, orientations: Vec<Orientation>) -> Self {
        let nombre_portes = orientations.len();
        match typage_piece {
            TypePiece::Fin if nombre_portes != 1 => {
                panic!("Les pièces de type 'Fin' doivent avoir exactement 1 porte.");
            }
            TypePiece::Debut | TypePiece::Normal if nombre_portes > 4 => {
                panic!("Les pièces 'Debut' ou 'Normales' doivent avoir moins de 4 portes.");
            }
            _ => {}
        }

        Piece {
            typage_piece,
            orientations,
        }
    }

    /// Affiche visuellement la pièce et ses portes dans la console.
    pub fn afficher(&self) {
        let mut piece_grille = vec![vec!['X'; 5]; 3];
        piece_grille[1][2] = 'o'; // Centre

        println!("#### Current room");
        match self.typage_piece {
            TypePiece::Debut => println!("You are at the beginning of your journey...\n"),
            TypePiece::Fin => {
                println!("You have reached the exit !\n");
                // Lecture et affichage de l'ASCII art depuis le fichier
                match fs::read_to_string("ascii_chest.txt") {
                    Ok(ascii_art) => println!("{}", ascii_art),
                    Err(e) => println!("Erreur lors de la lecture du fichier ASCII: {}", e),
                }
                std::process::exit(0);
            }
            TypePiece::Normal => println!("Normal boring room...\n"),
        }

        for orientation in &self.orientations {
            match orientation {
                Orientation::N => piece_grille[0][2] = 'o',
                Orientation::S => piece_grille[2][2] = 'o',
                Orientation::E => {
                    piece_grille[1][3] = 'o';
                    piece_grille[1][4] = 'o';
                }
                Orientation::O => {
                    piece_grille[1][0] = 'o';
                    piece_grille[1][1] = 'o';
                }
            }
        }

        for row in piece_grille {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
