use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");
    let mut s:String = String::new();
    let banner = "Digite números separados por vígula\n\
    o resultado será a soma dos números (exemplo: 1, 2, 3, 44)";
    println!("{}", banner);
    io::stdin()
        .read_line(&mut s)
        .expect("Erro lendo mensagem");

    let nums: Vec<i32> = s.split(",")
        .map(|c|c.trim().parse().expect("Erro converter para inteiro"))
        .collect();

    println!("Você digitou {:?}", nums);
    let result: i32 = nums.iter().sum();
    println!("Resultado {:?}", result);
    println!("{}", "-".repeat(40));
}

