// 7) Digite um nome , calcule e mostre quantas letras tem. 

use std::io::stdin;

fn main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    let num_chars = name.trim().chars().count();
    println!("Seu nome tem {} letras", num_chars);
}


//versão alternativa usando um for e um contador

fn _other_main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    let mut count: u32 = 0;
    for _ in name.trim().chars(){
        count+=1;
    }
    println!("Seu nome tem {} letras", count);
}