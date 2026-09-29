mod automato;

use std::io;
use automato::Automato;

fn main() {
    loop {
        let mut automato: Automato = Automato::new();

        println!("Digite as letras da fita: ");

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro ao ler entrada.");

        let fita: u32 = entrada.trim().parse().expect("Digite um número.");

        if automato.consumir(fita) {
            println!("\nNúmero válido.");
        } else {
            println!("\nNúmero inválido.");
        }
    }
}
