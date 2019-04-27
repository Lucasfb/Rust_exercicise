// Gera um nome de modelo/versao que parece ser de algum componente ou aparelho eletronico
// Exemplos: TX550X, GG668U, HJY5531
// Formando: 2 ou 3 letras, seguido por 3 ou 4 numeros, seguido por uma letra (opcional). Todas as letras sao maiusculas
// Os dois primeiros numeros sao diferentes de zero, enquanto os dois ultimos tem 50% de chance de serem 0

extern crate rand;

fn new_model_name() -> String {    
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();

    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVXWYZ".chars().collect();
    let numeros_normais: Vec<char> = "123456789".chars().collect();
    let numeros_ultimos: Vec<char> = "000000000123456789".chars().collect();

    let mut model_name_vec : Vec<Option<&char>> = Vec::new();

    model_name_vec.push(letters.choose(&mut rng)); // Primeira letra
    model_name_vec.push(letters.choose(&mut rng)); // Segunda letras
    model_name_vec.push(if rand::random() {letters.choose(&mut rng)} else {None}); // Terceira letra
    model_name_vec.push(if rand::random(){Some(&'-')} else {None}); // "-"
    model_name_vec.push(numeros_normais.choose(&mut rng)); // Primeiro numero
    model_name_vec.push(numeros_normais.choose(&mut rng)); // Segundo numero
    model_name_vec.push(numeros_ultimos.choose(&mut rng)); // Terceiro numero
    model_name_vec.push(if rand::random() {numeros_ultimos.choose(&mut rng)} else {None}); // Quarto numero
    model_name_vec.push(if rand::random() {letters.choose(&mut rng)} else {None}); // Letra no fim

    let model_name = model_name_vec.into_iter().flatten().collect();

    // Return
    model_name 
}

fn main(){
    use std::env;

    let args: Vec<String> = env::args().collect();
    let mut n_repeticoes: i32 = 1;

    if args.len() >= 2{
        println!("Excess arguments discarded" );
        n_repeticoes = args[1].parse().expect("Not an integer");
    } else if args.len() == 1{
        n_repeticoes = args[1].parse().expect("Not an integer");
    }

    for _ in 0..n_repeticoes {
        println!("{}", new_model_name());
    }
}