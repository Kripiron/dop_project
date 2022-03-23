use std::io;

fn main() {
    println!("Угадай загаданное число");

    println!("Пожалуйста введите ваше число");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    println!("Ваша отгадка {}",  guess);
}