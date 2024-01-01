// 9) Receber do teclado uma mensagem e imprimir quantas letras A, E, I, O, U tem esta mensagem.
// Considerar minúscula e maiúscula. 

use std::io::stdin;

fn main(){
    println!("Digite uma mensagem: ");
    let mut msg: String = String::new();
    stdin().read_line(&mut msg).expect("Falha ao ler a mensagem");
    let mut count: u32 = 0;
    for c in msg.trim().chars(){
        if c=='a' || c=='A' || c=='e' || c=='E' || c=='i' || c=='I' || c=='o' || c=='O' || c=='u' || c=='U'{
            count+=1;
        }
    }
    println!("A mensagem tem {} vogais", count);
}