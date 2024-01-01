// 4)Escrever seu nome na tela 10 vezes. Um nome por linha. 

use std::io;

fn main() {
    println!("Digite seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    for _ in 0..10 {
        println!("{}", nome);
    }
}