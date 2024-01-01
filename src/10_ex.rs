// 10)Receber um nome no teclado e imprimir quantas letras "A" tem o nome. 

use std::io::stdin;

fn main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    let mut count: u32 = 0;
    for c in name.trim().chars(){
        if c=='A' || c=='a'{
            count+=1;
        }
    }
    println!("{} tem {} letras 'A'", name.trim(), count);
}