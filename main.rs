mod parser; // Importation du fichier parser.rs

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Affiche le prompt immédiatement
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        parse_input(&input);
    }
}