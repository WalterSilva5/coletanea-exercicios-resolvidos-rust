// 8) b) Receber do teclado um nome e imprimir quantas vezes cada caractere aparece no nome.

use std::io::stdin;

fn main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    let mut count: u32 = 0;
    for c in name.trim().chars(){
        for c2 in name.trim().chars(){
            if c==c2{
                count+=1;
            }
        }
        println!("{} aparece {} vezes", c, count);
        count=0;
    }
}