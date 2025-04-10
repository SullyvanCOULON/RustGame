use crate::piece::{Orientation, TypePiece, Piece};
use crate::musique::exec_music;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IteratorRandom;

use std::collections::HashMap;

/// Génère une orientation aléatoire vers une pièce voisine non encore visitée.
///
/// # Paramètres
/// - `position`: coordonnées actuelles.
/// - `visited_map`: carte des pièces déjà visitées.
/// - `hauteur`: hauteur du labyrinthe.
/// - `largeur`: largeur du labyrinthe.
/// - `depassement`: permet de dépasser les bornes si `true`.
///
/// # Retour
/// Une `Some(Orientation)` vers une pièce non visitée si possible, sinon `None`.
pub fn random_orientation(
    position: (i32, i32),
    visited_map: &HashMap<(i32, i32), bool>,
    hauteur: i32,
    largeur: i32,
    depassement: bool
) -> Option<Orientation> {
    let mut orientations = vec![
        (Orientation::N, (position.0, position.1 - 1)),
        (Orientation::E, (position.0 + 1, position.1)),
        (Orientation::S, (position.0, position.1 + 1)),
        (Orientation::O, (position.0 - 1, position.1)),
    ];

    // Filtrer les orientations hors limites si depassement interdit
    if !depassement {
        let min_x = -largeur / 2;
        let max_x = largeur / 2;
        let min_y = -hauteur / 2;
        let max_y = hauteur / 2;

        orientations.retain(|(_, (x, y))| *x >= min_x && *x < max_x && *y >= min_y && *y < max_y);
    }

    // Ne garder que les directions menant à une pièce non visitée
    orientations.retain(|(_, pos)| !visited_map.get(pos).copied().unwrap_or(false));

    orientations.choose(&mut thread_rng()).map(|(o, _)| *o)
}

/// Donne les coordonnées de la pièce située dans une direction donnée depuis une position.
///
/// # Paramètres
/// - `(x, y)`: coordonnées de départ.
/// - `direction`: direction à suivre.
///
/// # Retour
/// Coordonnées de la pièce voisine dans la direction indiquée.
pub fn get_next_room((x,y): (i32,i32), direction: Orientation) -> (i32, i32) {
    match direction {
        Orientation::N => (x, y - 1),
        Orientation::S => (x, y + 1),
        Orientation::E => (x + 1, y),
        Orientation::O => (x - 1, y),
    }
}

/// Retourne l'orientation opposée à celle donnée.
///
/// # Paramètres
/// - `orientation`: direction initiale.
///
/// # Retour
/// L'orientation inverse.
pub fn invert_orientation(orientation: Orientation) -> Orientation {
    match orientation {
        Orientation::N => Orientation::S,
        Orientation::S => Orientation::N,
        Orientation::E => Orientation::O,
        Orientation::O => Orientation::E,
    }
}

/// Génère les portes reliant les pièces du labyrinthe.
///
/// Cette fonction applique un algorithme proche de DFS aléatoire pour générer un graphe connexe.
/// Elle garantit l'existence d'une pièce de début et d'une pièce de fin.
///
/// # Paramètres
/// - `grid`: grille des pièces à connecter.
/// - `largeur`: largeur du labyrinthe.
/// - `hauteur`: hauteur du labyrinthe.
pub fn generer_portes(grid: &mut HashMap<(i32, i32), Piece>, largeur: i32, hauteur: i32) {
    let mut visited_map: HashMap<(i32, i32), bool> = HashMap::new();
    let mut has_exit = false;

    // Initialiser la carte des pièces non visitées
    for x in -largeur/2..=(largeur/2) {
        for y in -hauteur/2..=(hauteur/2) {
            visited_map.insert((x, y), false);
        }
    }

    let mut suivants: Vec<(i32, i32)> = Vec::new();
    //let mut position_generator = (0, 0);
    let mut rng = rand::thread_rng();

    // Définir la première pièce comme point de départ
    let start_orientation = random_orientation((0,0), &visited_map, hauteur, largeur, false);
    if let Some(first_piece) = grid.get_mut(&(0,0)) {
        first_piece.typage_piece = TypePiece::Debut;
        first_piece.orientations.push(start_orientation.expect("REASON"));
    }
    visited_map.insert((0, 0), true);
    suivants.push(get_next_room((0,0), grid.get(&(0, 0)).unwrap().orientations[0]));

    let mut last_orientation = start_orientation;
    
    while visited_map.values().any(|&v| !v) {
        while let Some((x, y)) = suivants.pop() {
            
            if visited_map.get(&(x, y)) == Some(&true) {
                continue;
            }
            visited_map.insert((x, y), true);

            // Ajouter une porte retour vers la pièce précédente
            let back_door = invert_orientation(last_orientation.expect("REASON"));
            grid.get_mut(&(x, y)).unwrap().orientations.push(back_door);

            // Générer une nouvelle orientation
            if let Some(next_orientation) = random_orientation((x, y), &visited_map, hauteur, largeur, false) {
                let next_room = get_next_room((x, y), next_orientation);
                if visited_map.get(&next_room) == Some(&false) {
                    grid.get_mut(&(x,y)).unwrap().orientations.push(next_orientation);
                    last_orientation = Some(next_orientation);
                    suivants.push(next_room);
                    
                }
            } else {
                break;
            }
        }

        // Connexion d'une nouvelle pièce non visitée à une voisine déjà visitée
        if let Some((&new_pos, _)) = visited_map.iter()
            .filter(|(&(x, y), &visited)| !visited &&
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .any(|p| visited_map.get(p) == Some(&true)))
            .choose(&mut rng)
        {
            if let Some(&(visited_x, visited_y, dir)) = [
                (new_pos.0 - 1, new_pos.1, Orientation::O),
                (new_pos.0 + 1, new_pos.1, Orientation::E),
                (new_pos.0, new_pos.1 - 1, Orientation::N),
                (new_pos.0, new_pos.1 + 1, Orientation::S),
            ].iter().find(|(vx, vy, _)| visited_map.get(&(*vx, *vy)) == Some(&true)) {
                let visited_pos = (visited_x, visited_y);
                let back_door = invert_orientation(dir);

                grid.get_mut(&visited_pos).unwrap().orientations.push(back_door);
                grid.get_mut(&new_pos).unwrap().orientations.push(dir);
                visited_map.insert(new_pos, true);
            }

            // Possiblement générer une pièce de fin si hors limites
            if let Some(dir) = random_orientation(new_pos, &visited_map, hauteur, largeur, !has_exit) {
                let next_r = get_next_room(new_pos, dir);
                let back_door = invert_orientation(dir);

                if next_r.0 < -largeur / 2 || next_r.0 > largeur / 2 ||
                   next_r.1 < -hauteur / 2 || next_r.1 > hauteur / 2 {
                    grid.get_mut(&new_pos).unwrap().orientations.push(dir);
                    grid.insert(next_r, Piece {
                        typage_piece: TypePiece::Fin,
                        orientations: vec![back_door],
                        musique: exec_music("do", "majeur", 3, "lent")
                    });
                    has_exit = true;
                } else {
                    grid.get_mut(&new_pos).unwrap().orientations.push(dir);
                    last_orientation = Some(dir);
                    suivants.push(next_r);
                }
            }
        }
    }
}
