mod automato;

use std::io;
use automato::Automato;

fn main() {
    loop {
        let mut automato: Automato = Automato::new();

        println!("Digite as letras da fita: ");

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao receber entrada.");

        let fita = entrada.trim();

        if automato.consumir(fita) {
            println!("\nIdentificador válido.");
        } else {
            println!("\nIdentificador inválido.");
        }
    }
}
