mod parser; // Importation du fichier parser.rs
mod piece;  // Importation du fichier piece.rs

use std::io::{self, Write};
use parser::parse_input; // Importation de la fonction parse_input

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Affiche le prompt immédiatement
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        parse_input(&input); // Appel de la fonction parser
    }
}
