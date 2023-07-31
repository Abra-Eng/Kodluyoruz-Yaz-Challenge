use std::io::stdin;

fn main() {
    // Kullanıcıdan bir sayı alaım ve asal mı değil mi kontrol edelim
 
    let mut input = String::new();
    println!("Bir sayı giriniz: ");
    stdin().read_line(&mut input).expect("Hata");

    // stringi tamsayı dönüştürme
    let number: i32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Hata!");
            return;
        }
    };
    
    let result = is_prime(number);
    
    match result {
        true => println!("{number} bir asal sayıdır."),
        false => println!("{number} bir asal sayı değildir.")
    }
    

}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    } 

    for i in 2..num  {
        if num % i == 0 {
            return false;
        }
    }

    true
}
