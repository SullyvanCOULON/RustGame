/// Enumération des orientations possibles des portes.
///
/// Cette énumération permet de spécifier les différentes directions dans lesquelles
/// les portes peuvent être orientées dans une pièce du labyrinthe.
#[derive(Debug, PartialEq, Copy, Clone)]  // Ajoute ici le dérivatif PartialEq pour permettre la comparaison
pub enum Orientation {
    /// Orientation vers le nord
    Nord,
    /// Orientation vers le sud
    Sud,
    /// Orientation vers l'est
    Est,
    /// Orientation vers l'ouest
    Ouest,
}

/// Enumération des types de pièces du labyrinthe.
///
/// Chaque pièce peut être de type `Début`, `Fin` ou `Normal`
/// - `Début` représente la première pièce du labyrinthe.
/// - `Fin` représente la pièce d'arrivée.
/// - `Normal` représente les pièces classiques.
#[derive(Debug)]
pub enum TypePiece {
    /// Pièce de départ
    Debut,
    /// Pièce de fin
    Fin,
    /// Pièce classique
    Normal,
}

/// Struct représentant une pièce du labyrinthe.
///
/// Chaque pièce a un type (Début, Fin, Normal) et une liste d'orientations qui représente
/// les portes dans cette pièce. La structure de la pièce est utilisée pour déterminer
/// les déplacements possibles du joueur.
#[derive(Debug)]
pub struct Piece {
    /// Liste des orientations des portes de la pièce
    /// Cette valeur est publique pour permettre l'accès extérieur à la liste d'orientations
    pub orientations: Vec<Orientation>,
    
    /// Type de la pièce (Début, Fin ou Normal)
    /// Ce champ est privé, car il ne doit pas être modifié directement
    pub typage_piece: TypePiece,
}

impl Piece {
    /// Crée une nouvelle pièce avec un type et un ensemble d'orientations de portes.
    ///
    /// Cette fonction vérifie que le nombre de portes est valide pour le type de pièce.
    ///
    /// # Arguments
    /// 
    /// * `typage_piece` - Le type de la pièce (Début, Fin, Normal).
    /// * `orientations` - Une liste des orientations des portes dans la pièce.
    ///
    /// # Panique
    ///
    /// La fonction panique si le nombre de portes n'est pas valide pour le type de pièce.
    /// - Les pièces de type `Début` et `Fin` doivent avoir exactement 1 porte.
    /// - Les pièces de type `Normal` doivent avoir entre 1 et 4 portes.
    ///
    /// # Exemples
    ///
    /// ```rust
    /// let piece = Piece::new(TypePiece::Normal, vec![Orientation::Nord, Orientation::Sud]);
    /// ```
    pub fn new(typage_piece: TypePiece, orientations: Vec<Orientation>) -> Self {
        let nombre_portes = orientations.len();

        // Vérification que le nombre de portes est valide selon le type de pièce
        match typage_piece {
            TypePiece::Debut | TypePiece::Fin if nombre_portes != 1 => {
                panic!("Les pièces de type 'Début' et 'Fin' doivent avoir exactement 1 porte.");
            }
            TypePiece::Normal if nombre_portes > 4 => {
                panic!("Les pièces 'Normales' doivent avoir entre 1 et 4 portes.");
            }
            _ => {}
        }

        // Création de la pièce
        Piece { typage_piece, orientations }
    }

    /// Affiche la pièce dans la console sous forme de grille avec des portes.
    ///
    /// La fonction place les portes au centre de la pièce, et en fonction des orientations,
    /// elle place les autres portes dans la grille.
    ///
    /// Elle affiche également le type de la pièce pour indiquer au joueur s'il est
    /// dans la pièce de départ, de fin ou une pièce classique.
    ///
    /// # Exemples
    ///
    /// ```rust
    /// let piece = Piece::new(TypePiece::Normal, vec![Orientation::Est, Orientation::Sud]);
    /// piece.afficher();
    /// ```
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
