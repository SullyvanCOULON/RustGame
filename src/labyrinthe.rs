use crate::Piece;
use crate::TypePiece;
use crate::Orientation;
use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IteratorRandom;

use std::collections::{HashMap, VecDeque};

/// Structure représentant le labyrinthe.
///
/// Le labyrinthe contient une collection de pièces, la position actuelle du joueur,
/// et un historique des positions précédentes du joueur.
pub struct Labyrinthe {
    /// Map des coordonnées des pièces, où chaque clé est une paire de coordonnées `(x, y)` 
    /// et la valeur correspond à une pièce.
    grid: HashMap<(i32, i32), Piece>,
    
    /// La position actuelle du joueur dans le labyrinthe.
    position: (i32, i32),
    
    /// Historique des positions précédentes du joueur. Utilisé pour la commande 'revenir'.
    historique: VecDeque<(i32, i32)>,
}

impl Labyrinthe {
    /// Crée un nouveau labyrinthe avec des pièces de départ, de fin et normales.
    ///
    /// Cette fonction initialise les pièces du labyrinthe en ajoutant des coordonnées et des
    /// types de pièces à la collection `grid`. Elle initialise également la position du joueur
    /// au point de départ `(0, 0)` et un historique vide.
    ///
    /// # Retour
    ///
    /// Retourne une nouvelle instance de `Labyrinthe`.
    pub fn new(largeur: i32, hauteur: i32) -> Self {
        let mut grid = Self::generer_labyrinthe(largeur, hauteur);
        Self::generer_portes(&mut grid, largeur, hauteur);
        Self {
            grid,
            position: (0, 0),
            historique: VecDeque::new(),
        }
    }    

    fn random_orientation(
        position: (i32, i32),
        visited_map: &HashMap<(i32, i32), bool>,
        hauteur: i32,
        largeur: i32,
        depassement: bool
    ) -> Option<Orientation> {
        let mut orientations = vec![
            (Orientation::Nord, (position.0, position.1 - 1)),
            (Orientation::Est, (position.0 + 1, position.1)),
            (Orientation::Sud, (position.0, position.1 + 1)),
            (Orientation::Ouest, (position.0 - 1, position.1)),
        ];
    
        // Vérification des limites uniquement si `depassement` est false
        if !depassement {
            let min_x = -largeur / 2;
            let max_x = largeur / 2;
            let min_y = -hauteur / 2;
            let max_y = hauteur / 2;
    
            orientations.retain(|(_, (x, y))| *x >= min_x && *x < max_x && *y >= min_y && *y < max_y);
        }
    
        // Filtrer les orientations en fonction des salles non visitées
        orientations.retain(|(_, pos)| !visited_map.get(pos).copied().unwrap_or(false));

        // Sélectionne aléatoirement une orientation possible ou retourne None si aucune
        let choosed = orientations.choose(&mut thread_rng()).map(|(o, _)| *o);
        choosed
    }
    
         
    fn get_next_room((x,y): (i32,i32), direction: Orientation) -> (i32, i32) {
        match direction {
            Orientation::Nord => (x, y - 1),
            Orientation::Sud => (x, y + 1),
            Orientation::Est => (x + 1, y),
            Orientation::Ouest => (x - 1, y),
        }
    }

    fn invert_orientation(orientation: Orientation) -> Orientation {
        match orientation {
            Orientation::Nord => Orientation::Sud,
            Orientation::Sud => Orientation::Nord,
            Orientation::Est => Orientation::Ouest,
            Orientation::Ouest => Orientation::Est,
        }
    }

    fn generer_labyrinthe(largeur: i32, hauteur: i32) -> HashMap<(i32, i32), Piece> {
        let mut grid = HashMap::new();
    
        for x in -largeur/2..=largeur/2 {
            for y in -hauteur/2..=hauteur/2 {
                grid.insert((x, y), Piece::new(TypePiece::Normal, vec![]));
            }
        }
        grid
    }
    
    fn generer_portes(grid: &mut HashMap<(i32, i32), Piece>, largeur: i32, hauteur: i32) {
        let mut visited_map: HashMap<(i32, i32), bool> = HashMap::new();
        let mut has_exit = false;

        for x in -largeur/2..(largeur/2)+1 {
            for y in -hauteur/2..(hauteur/2)+1 {
                visited_map.insert((x, y), false);
            }
        }

        let mut suivants: Vec<(i32, i32)> = Vec::new();
        let mut position_generator = (0, 0);
        let mut rng = rand::thread_rng();

        // Générer la première pièce
        let start_orientation = Self::random_orientation((0,0), &visited_map, hauteur, largeur, false);
        let first_piece = grid.get(&(0, 0));
        if let Some(first_piece) = grid.get_mut(&(0,0)) {
            first_piece.typage_piece = TypePiece::Debut;
        }        
        if let Some(first_piece) = grid.get_mut(&(0,0)) {
            first_piece.orientations.push(start_orientation.expect("REASON"));
        }        
        visited_map.insert((0, 0), true);
        let piece_debut = grid.get(&(0, 0));
        suivants.push(Self::get_next_room((0,0), piece_debut.unwrap().orientations[0]));

        let mut last_orientation = start_orientation;
        
        while visited_map.values().any(|&v| !v) { 
            while !suivants.is_empty() {
                let Some((x, y)) = suivants.pop() else { continue };
                position_generator = (x,y);
                if visited_map.get(&(x, y)) == Some(&true) {
                    continue;
                }
                visited_map.insert((x, y), true);
        
                let mut back_door = Self::invert_orientation(last_orientation.expect("REASON"));
                grid.get_mut(&(x, y)).unwrap().orientations.push(back_door);

                if let Some(next_orientation) = Self::random_orientation((x, y), &visited_map, hauteur, largeur, false) {
                    let next_room = Self::get_next_room((x, y), next_orientation);
                    if visited_map.get(&next_room) == Some(&false) {

                        if let Some(piece) = grid.get_mut(&next_room) {
                            grid.get_mut(&(x,y)).unwrap().orientations.push(next_orientation);
                        }
                        last_orientation = Some(next_orientation);
                        suivants.push(next_room);
                        position_generator = (x,y);
                    }
                } else {
                    break; // Sortie propre de la boucle si aucune orientation disponible
                }
            }
            if let Some((&new_pos, _)) = visited_map.iter()
                .filter(|(&(x, y), &visited)| !visited && // Prendre une pièce non visitée
                    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] // Vérifier ses voisins
                        .iter()
                        .any(|p| visited_map.get(p) == Some(&true)))
                .choose(&mut rng) // Sélectionner une telle pièce au hasard
            {
                
                // Trouver une salle voisine déjà visitée pour la connecter
                if let Some(&(visited_x, visited_y, dir)) = [(new_pos.0 - 1, new_pos.1, Orientation::Ouest),
                                                             (new_pos.0 + 1, new_pos.1, Orientation::Est),
                                                             (new_pos.0, new_pos.1 - 1, Orientation::Nord),
                                                             (new_pos.0, new_pos.1 + 1, Orientation::Sud)]
                    .iter()
                    .find(|(vx, vy, _)| visited_map.get(&(*vx, *vy)) == Some(&true)) 
                {
                    let visited_pos = (visited_x, visited_y);
                    let back_door = Self::invert_orientation(dir);

                    // Ajouter la porte entre les pièces
                    if let Some(piece) = grid.get_mut(&visited_pos) {
                        piece.orientations.push(back_door);
                    }
                    if let Some(piece) = grid.get_mut(&new_pos) {
                        piece.orientations.push(dir);
                    }
                    visited_map.insert(new_pos, true);
                }
                if let Some(dir) = Self::random_orientation(new_pos, &visited_map, hauteur, largeur, !has_exit) {
                    let next_r = Self::get_next_room(new_pos, dir);
                    let back_door = Self::invert_orientation(dir);
                    

                    // Vérifier si la nouvelle position est en dehors des limites
                    if (next_r.0 < -largeur / 2 || next_r.0 > largeur / 2 ||
                        next_r.1 < -hauteur / 2 || next_r.1 > hauteur / 2)
                    {
                        grid.get_mut(&new_pos).unwrap().orientations.push(dir);
                        // Créer et marquer la pièce comme finale
                        grid.insert(next_r, Piece {
                            typage_piece: TypePiece::Fin,
                            orientations: vec![back_door],
                        });
                        has_exit = true;
                    } else {
                        // Ajouter la porte à la pièce déjà visitée
                        if let Some(piece) = grid.get_mut(&new_pos) {
                            piece.orientations.push(dir);
                            last_orientation = Some(dir);

                            // Ajouter la nouvelle pièce à la liste des suivantes
                            suivants.push(next_r);
                        }
                    }
                }
            }
        }
    }

    /// Affiche la pièce actuelle dans le labyrinthe.
    ///
    /// Cette fonction récupère la pièce correspondant à la position actuelle du joueur
    /// et l'affiche. Si la pièce n'existe pas, elle affiche un message indiquant un endroit inconnu.
    pub fn afficher_piece_actuelle(&self) {
        if let Some(piece) = self.grid.get(&self.position) {
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
    pub fn deplacer(&mut self, direction: Orientation) {
        
        let (x, y) = self.position;
        let nouvelle_position = Self::get_next_room((x,y), direction);

        if let Some(piece) = self.grid.get(&(x,y)) {
            if piece.orientations.contains(&direction) {
                // Avant de se déplacer, on empile la position actuelle
                self.historique.push_back(self.position);

                // On met à jour la position du joueur et on affiche la nouvelle pièce
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
    pub fn revenir(&mut self) {
        // Si l'historique contient une position précédente, on revient à cette position
        if let Some(position_precedente) = self.historique.pop_back() {
            self.position = position_precedente;
            self.afficher_piece_actuelle();
        } else {
            println!("Il n'y a pas de position précédente. Vous êtes déjà à l'origine.");
        }
    }
}