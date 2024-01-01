//6) Ler nome, sexo e idade. Se sexo for feminino e idade menor que 25.
//Imprimir o nome da pessoa
//e a palavra ACEITA. Caso contrario imprimir NAO ACEITA.


use std::io::stdin;

fn main(){
    println!("Digite seu nome: ");
    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Falha ao ler o nome");
    
    println!("Digite seu sexo (M/F): ");
    let mut gender_str:String = String::new();
    stdin().read_line(&mut gender_str).expect("Falha ao ler o sexo");
    
    let gender: char = gender_str.trim().parse().expect("Falha ao converter o sexo para char");

    println!("Digite sua idade");
    let mut age_str: String = String::new();
    stdin().read_line(&mut age_str).expect("Falha ao ler idade");
    let age: u32 = age_str.trim().parse().expect("Falha ao converter a idade para u32");
    if age<25 && gender =='f'{
        println!("ACEITA");
        return;
    }
    println!("NÃO ACEITA");
}
