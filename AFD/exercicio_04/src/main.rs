mod automato;

use std::io;
use automato::Automato;

fn main() {
    loop {
        let mut automato: Automato = Automato::new();

        println!("Digite as letras da fita: ");

        let mut fita = String::new();
        io::stdin().read_line(&mut fita).expect("Erro ao ler entrada.");

        if automato.consumir(fita.trim().to_string()) {
            println!("\nNúmero válido.");
        } else {
            println!("\nNúmero inválido.");
        }
    }
}
