fn main() {
    // Pega uma string com palavras separadas por whitespace e usa ela para gerar nomes de carros

    let NAMES = "Vector Reference String Mutable Trait Main Fizz Buzz";

    for name in NAMES.split_whitespace().collect::<Vec<_>>() {
        println!("Ford {}", name);
    }
}
