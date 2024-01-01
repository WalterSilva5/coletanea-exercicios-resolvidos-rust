// 3)Receber um nome e imprimir as letras na posição par 

use std::io;

fn main(){
    println!("Digite seu nome: ");
    let mut nome = String::new();

    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome.");

    let chars: Vec<char> = nome.chars().collect();
    let mut chars_result: Vec<char> = Vec::new();

    for (i, c) in chars.iter().enumerate() {
        if i % 2 != 0 {
            chars_result.push(*c);
        }
    }

    println!("{:?}", chars_result);
}