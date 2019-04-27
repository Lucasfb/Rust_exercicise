fn main() {
    // Classico exercicio de fizzbuzz
    // Mostra n numeros na tela
    // Numeros multiplos de 3 sao substituidos por Fizz e multiplos de 5 são substituidos por Buzz. Multiplos de ambos sao substituidos por Fizzbuzz
    
    let mut next : String; // Variavel nao inicializada
    let fizz = "Fizz";
    let buzz = "Buzz";
    let fizzbuzz = "Fizzbuzz";

    let n : u32 = 100; // Numero de iteracoes

    for i in 1..n {
        if i%3 == 0 && i%5 == 0 {
            next = fizzbuzz.to_string();
        } else if i%3 == 0 {
            next = fizz.to_string();
        } else if i%5 == 0 {
            next = buzz.to_string();
        } else {
            next = i.to_string();
        }        
    println!("{}",next);
    }    
}