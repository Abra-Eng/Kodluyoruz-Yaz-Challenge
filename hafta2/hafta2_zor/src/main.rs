use std::io::stdin;

fn main() {
    // Maliyet ve satış fiyatı verilen bir üründen ne kadar satılırsa kâr elde edilebileceğini bulalım.
    // elde edilecek kâr birim fiyattan fazla olmalı

    println!("Maliyeti giriniz: ");
    let cost = read_user_input();
    
    println!("Fiyatı giriniz: ");
    let price = read_user_input();

    let mut sold = 0;
    let mut total_profit = 0.0;
    
    if cost < price {
        // toplam kâr maliyetten fazla olmalı 
        while total_profit <= cost{
            sold += 1;
            
            total_profit = price * sold as f64 - cost * sold as f64;
        }

        println!("Şirketin kâra geçmek için satması gereken ürün adedi: {}", sold);
    } else {
        println!("Şirket kâr elde edemez!");
    }
    

    println!()
}

fn read_user_input() -> f64 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Hata!");
    input.trim().parse().expect("Hata")
}
