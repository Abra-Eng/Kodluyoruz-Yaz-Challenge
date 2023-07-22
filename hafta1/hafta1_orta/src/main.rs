use std::{io, collections::HashMap};
fn main() {
    // Kullanıcıdan bir metin al
    println!("Lütfen metni giriniz: ");

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Hata");

    // metnin içinde en çok tekrar eden harfi bul ve kaç kere tekrar ettiğini göster
    let most_common_letter = find_most_common_letter(&input);

   // Sonuç
   match most_common_letter {
       Some((letter, count)) => println!("En çok tekrar eden harf: {}, Tekrar Sayısı: {}", letter,count),
       None => println!("Metin bulunamadı"),
   }    
    

}

fn find_most_common_letter(text: &str) -> Option<(char, usize)> {
   // Metnin içindeki harfleri ve tekrar sayılarını anahtar, değer şeklinde tut
   let mut letter_count = HashMap::new();

   // alınan metindeki karakterleri alfabetik olup olmamasına bağlı olarak filtreleme
   for c in text.chars().filter(|c| c.is_alphabetic()){
        // harfin tekrar sayısını bulma ve hashmap'e  ekleme 
        *letter_count.entry(c.to_ascii_lowercase()).or_insert(0) += 1;   
    }

    let mut max_count = 0;
    let mut most_common_letter = None;

    for(&letter, &count) in &letter_count {
        if count > max_count {
            max_count = count;
            most_common_letter = Some(letter);
        } 
    }
 
    most_common_letter.map(|letter| (letter,max_count))

}