use crate::piece::{Orientation, TypePiece, Piece};

use crate::musique::exec_music;


use std::collections::{HashMap, VecDeque};

use crate::doors::{get_next_room, generer_portes};

/// Structure représentant un labyrinthe navigable.
///
/// Elle contient une grille de pièces, la position actuelle du joueur,
/// ainsi qu'un historique des déplacements effectués.
pub struct Labyrinthe {
    grid: HashMap<(i32, i32), Piece>,
    position: (i32, i32),
    historique: VecDeque<(i32, i32)>,
}

impl Labyrinthe {
    /// Crée un nouveau labyrinthe rectangulaire avec une pièce de début et de fin.
    ///
    /// # Paramètres
    /// - `largeur`: nombre de colonnes du labyrinthe.
    /// - `hauteur`: nombre de lignes du labyrinthe.
    ///
    /// # Retour
    /// Une instance initialisée de `Labyrinthe`.
    pub fn new(largeur: i32, hauteur: i32) -> Self {
        let mut grid = Self::generer_labyrinthe(largeur, hauteur);
        generer_portes(&mut grid, largeur, hauteur);
        Self {
            grid,
            position: (0, 0),
            historique: VecDeque::new(),
        }
    }

    /// Crée une grille vide de pièces normales, sans portes.
    ///
    /// # Paramètres
    /// - `largeur`: largeur en nombre de pièces.
    /// - `hauteur`: hauteur en nombre de pièces.
    ///
    /// # Retour
    /// Une `HashMap` des coordonnées vers des pièces normales.
    fn generer_labyrinthe(largeur: i32, hauteur: i32) -> HashMap<(i32, i32), Piece> {
        let mut grid = HashMap::new();
        for x in -largeur/2..=largeur/2 {
            for y in -hauteur/2..=hauteur/2 {
                grid.insert((x, y), Piece::new(TypePiece::Normal, vec![], exec_music("do", "majeur", 3, "lent")));
            }
        }
        grid
    }

    /// Affiche la pièce actuellement occupée par le joueur.
    ///
    /// Affiche un message spécial si aucune pièce ne correspond à la position.
    pub fn afficher_piece_actuelle(&self) {
        if let Some(piece) = self.grid.get(&self.position) {
            piece.afficher();
        } else {
            println!("Vous êtes dans un endroit inconnu.");
        }
    }

    /// Tente de déplacer le joueur dans une direction donnée.
    ///
    /// Si une porte est présente dans la direction indiquée, le joueur est déplacé.
    ///
    /// # Paramètres
    /// - `direction`: direction de déplacement (N, S, E, O).
    pub fn deplacer(&mut self, direction: Orientation) {
        let (x, y) = self.position;
        let nouvelle_position = get_next_room((x,y), direction);

        if let Some(piece) = self.grid.get(&(x,y)) {
            if piece.orientations.contains(&direction) {
                self.historique.push_back(self.position);
                self.position = nouvelle_position;
                self.afficher_piece_actuelle();
            } else {
                println!("Il n'y a pas de porte par là !");
            }
        } else {
            println!("Vous ne pouvez pas aller dans cette direction.");
        }
    }

    /// Ramène le joueur à sa position précédente, si elle existe.
    ///
    /// Affiche un message si aucune position précédente n'est disponible.
    pub fn revenir(&mut self) {
        if let Some(position_precedente) = self.historique.pop_back() {
            self.position = position_precedente;
            self.afficher_piece_actuelle();
        } else {
            println!("Il n'y a pas de position précédente. Vous êtes déjà à l'origine.");
        }
    }
}
