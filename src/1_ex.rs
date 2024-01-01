// 1)Receber um nome e imprimir as 4 primeiras letras do nome. 

use std::io;

fn main() {
    println!("Digite seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");
    let nome = nome.trim();
    let nome = nome.to_string();
    if nome.len() < 4 {
        println!("Seu nome é muito curto");
        return;
    }
 
    let nome = &nome[0..4];
    println!("As 4 primeiras letras do seu nome são: {}", nome);
}