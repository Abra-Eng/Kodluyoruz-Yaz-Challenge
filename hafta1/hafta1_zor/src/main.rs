use std::io;

fn main() {
    //Bir tam sayı dizisi oluşturun. KUllanıcıdan alalım
    println!("Tam sayı dizisini giriniz:(sayıları arasına boşluk bırakınız) ");
    let my_array = read_user_input();

    let numbers:Vec<i32> = my_array
        .split(' ')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    //Kullanıcıdan abir hedef sayı alın. 
    println!("Hedef sayıyı giriniz: ");
    let target_num= read_user_input().trim().parse().expect("hata");

    //Dizinin içinden sayılar seçerek hedef sayıya ulaşın. Farklı kombinasyonlarla ulaşılabilmeli.
    if let Some(combination) = find_combination(&numbers, target_num) {
        println!("Hedefe ulaşan kombinasyon: {:?}", combination);
    } else {
        println!("Hedef sayıya ulaşılamadı! Tam sayı dizisi ile hedef sayıya ulaşılamıyor.");
    }

}
// Kullanıcıdan bilgi alma
fn read_user_input() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Okunamadı.");
    input
}

fn find_combination(numbers: &[i32], target: i32) -> Option<Vec<i32>>{
    let mut combination = Vec::new();

    find_combination_recursive(numbers, target, 0, &mut combination)
}

fn find_combination_recursive(numbers: &[i32], target: i32, start: usize, combination: &mut Vec<i32>) -> Option<Vec<i32>>{
    if target == 0 {
        // Hedef 0 ise geri döndürür
        return Some(combination.clone());
    } 

    for i in start..numbers.len() {
        if numbers[i] <= target {
            combination.push(numbers[i]);
            //özyinelemeli bir şekilde hedef sayıya ulaşmak için hedef sayıdan küçük olan değerleri 
            //hedef sayıdan çıkartırız. start değerini o anki sayının indisi olarak belirleriz.
            let result = find_combination_recursive(numbers, target - numbers[i], i + 1, combination);
            if result.is_some() {
                return result;
            }
            // Hedef değere ulaşılamadı ise eklenen sayı çıkarır ve farklı kombinasyon deneriz.
            combination.pop();
        }
    }
    // Hedefe ulaşılmadıysa None döndürürüz
    None
}