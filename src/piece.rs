// Enumération des orientations possibles des portes
// Enumération des orientations possibles des portes
#[derive(Debug, PartialEq)]  // Ajoute ici le dérivatif PartialEq
pub enum Orientation {
    Nord,
    Sud,
    Est,
    Ouest,
}


#[derive(Debug)]
pub enum TypePiece {
    Debut,
    Fin,
    Normal,
}

// Struct pour représenter une Pièce
#[derive(Debug)]
pub struct Piece {
    pub orientations: Vec<Orientation>, // Rendre ce champ public
    typage_piece: TypePiece,            // Ce champ reste privé
}


impl Piece {
    // Fonction pour créer une nouvelle Pièce
    pub fn new(typage_piece: TypePiece, orientations: Vec<Orientation>) -> Self {
        let nombre_portes = orientations.len();

        // Vérification que le nombre de portes est valide selon le type de pièce
        match typage_piece {
            TypePiece::Debut | TypePiece::Fin if nombre_portes != 1 => {
                panic!("Les pièces de type 'Début' et 'Fin' doivent avoir exactement 1 porte.");
            }
            TypePiece::Normal if nombre_portes < 1 || nombre_portes > 4 => {
                panic!("Les pièces 'Normales' doivent avoir entre 1 et 4 portes.");
            }
            _ => {}
        }

        // Création de la pièce
        Piece { typage_piece, orientations }
    }

    // Fonction pour afficher la pièce dans la console
    pub fn afficher(&self) {
        // Définition de la grille de la pièce (3x5)
        let mut piece_grille = vec![vec!['X'; 5]; 3];

        // Toujours placer une porte au centre de la pièce
        piece_grille[1][2] = 'o'; // Porte au centre

        // Remplissage des autres portes selon les orientations
        for orientation in &self.orientations {
            match orientation {
                Orientation::Nord => piece_grille[0][2] = 'o', // Porte en haut
                Orientation::Sud => piece_grille[2][2] = 'o', // Porte en bas
                Orientation::Est => {
                    piece_grille[1][3] = 'o';
                    piece_grille[1][4] = 'o'; // Porte à droite
                }
                Orientation::Ouest => {
                    piece_grille[1][0] = 'o';
                    piece_grille[1][1] = 'o'; // Porte à gauche
                }
            }
        }

        // Affichage de la grille dans la console
        for row in piece_grille {
            println!("{}", row.iter().collect::<String>());
        }

        // Affichage du type de pièce
        match self.typage_piece {
            TypePiece::Debut => println!("Vous êtes au début du labyrinthe."),
            TypePiece::Fin => println!("Bravo ! Vous êtes à la fin !"),
            TypePiece::Normal => println!("Pièce classique."),
        }

        println!();
    }
}