// Enumération des orientations possibles des portes
#[derive(Debug)]
enum Orientation {
    Nord,
    Sud,
    Est,
    Ouest,
}

#[derive(Debug)]
enum Type_Piece {
    Debut,
    Fin,
    Normal,
}

// Struct pour représenter une Pièce
#[derive(Debug)]
struct Piece {
    orientations: Vec<Orientation>, // Liste des orientations des portes
    type_p : Type_Piece, //type de la pièce
}

impl Piece {
    // Fonction pour créer une nouvelle Pièce
    fn new(type_p: TypePiece, orientations: Vec<Orientation>) -> Self {
        let nombre_portes = orientations.len();

        // Vérification que le nombre de portes est valide selon le type de pièce
        match type_p {
            TypePiece::Debut | TypePiece::Fin if nombre_portes != 1 => {
                panic!("Les pièces de type 'Début' et 'Fin' doivent avoir exactement 1 porte.");
            }
            TypePiece::Normal if nombre_portes < 1 || nombre_portes > 4 => {
                panic!("Les pièces 'Normales' doivent avoir entre 1 et 4 portes.");
            }
            _ => {}
        }

        // Création de la pièce
        Piece { type_p, orientations }
    }

    // Fonction pour afficher la pièce dans la console
    fn afficher(&self) {
        // Définition de la grille de la pièce (6x6)
        let mut piece_grille = vec![vec!['X'; 5]; 3];

        // Toujours placer une porte au centre de la pièce
        piece_grille[1][2] = 'o'; // Porte au centre

        // Remplissage des autres portes selon les orientations
        for orientation in &self.orientations {
            match orientation {
                Orientation::Nord => {
                    // Placer une porte en haut)
                    piece_grille[0][2] = 'o';
                },
                Orientation::Sud => {
                    // Placer une porte en bas
                    piece_grille[2][2] = 'o';
                },
                Orientation::Est => {
                    // Placer une porte à droite
                    piece_grille[1][3] = 'o';
                    piece_grille[1][4] = 'o';
                },
                Orientation::Ouest => {
                    // Placer une porte à gauche
                    piece_grille[1][0] = 'o';
                    piece_grille[1][1] = 'o';
                },
            }
        }

        // Affichage de la grille dans la console
        for row in piece_grille {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
