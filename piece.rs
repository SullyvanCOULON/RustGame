// Enumération des orientations possibles des portes
#[derive(Debug)]
enum Orientation {
    Nord,
    Sud,
    Est,
    Ouest,
}

// Struct pour représenter une Pièce
#[derive(Debug)]
struct Piece {
    orientations: Vec<Orientation>, // Liste des orientations des portes
}

impl Piece {
    // Fonction pour créer une nouvelle Pièce
    fn new(orientations: Vec<Orientation>) -> Self 
    {
        // S'assurer que le nombre de portes est entre 1 et 4
        if orientations.len() < 1 && orientations.len() > 4 
        {
            panic!("Le nombre de portes doit être entre 1 et 4.");
        }
        // Vérification que le nombre d'orientations correspond au nombre de portes
        Piece 
        {
            orientations,
        }
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
