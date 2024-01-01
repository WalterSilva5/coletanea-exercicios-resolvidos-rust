// 8) Receber do teclado um nome e imprimir tantas vezes quantos forem seus caracteres. 

use std::io::stdin;

fn main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    let num_chars = name.trim().chars().count();
    for _ in 0..num_chars{
        println!("{}", name.trim());
    }
}