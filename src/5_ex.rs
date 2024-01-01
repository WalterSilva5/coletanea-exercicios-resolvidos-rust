// 5)Ler nome, endereço, telefone e imprimir 

use std::io;

fn main() {
    println!("Digite seu nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler o nome");

    println!("Digite seu endereço: ");
    let mut endereco = String::new();
    io::stdin().read_line(&mut endereco).expect("Falha ao ler o endereço");

    println!("Digite seu telefone: ");
    let mut telefone = String::new();
    io::stdin().read_line(&mut telefone).expect("Falha ao ler o telefone");

    println!("Nome: {}", nome);
    println!("Endereço: {}", endereco);
    println!("Telefone: {}", telefone);
}