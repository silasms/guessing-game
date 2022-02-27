use std::io; /* Biblioteca feita para entrada e saída(input/output) */
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Falied to read line");

        let guess: u32 = match guess.trim().parse() {  // Permite esse match porque o parse retorna um Result(Ok ou Err)
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) { //Aqui nessa linha compara o guess com o secret_number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;   
            },
        }
    }
}
