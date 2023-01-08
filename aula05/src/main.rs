use std::io;

fn main() {
    let mut s:String = String::new();
    println!("Digite um texto: ");
    io::stdin()
        .read_line(&mut s)
        .expect("Erro lendo mensagem");
    println!("Você digitou {s}");
}
