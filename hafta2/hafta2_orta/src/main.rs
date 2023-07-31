use std::io::stdin;

fn main() {
    // Kullanıcıdan alınan bir kelimenin haflerini büyük harfe dönüştürme
    
    println!("Metni giriniz: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Hata!");

    let converted_word = input.to_uppercase();

    println!("Dönüştürülmüş hali: {converted_word}"); 
}
