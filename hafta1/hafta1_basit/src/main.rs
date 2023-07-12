use std::io;

fn main() {
    
    let current_year = 2023;
    
    // Doğum yılının alınması 
    println!("Doğum Yılınız: ");
    let mut input = String::new ();
    io::stdin().read_line(&mut input).expect("Okunamadı.");
    
    // trim() - gereksiz boşukları temizler
    // parse() - Bir stringi başka bir değere dönüştürür ve Result tipinde bir değer dönderir.
    let birth_year: i32 = match input.trim().parse() {
        Ok(year) => year,
        Err(_) =>  {
            println!("Geçersiz giriş");
            return;
        }
    };

    // Yaş hesaplaması 
    let age = current_year - birth_year;

    println!("Yaşınız: {age}");

}
