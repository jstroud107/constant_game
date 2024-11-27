use std::io;

//You can only enter 4 numbers
//Must be numbers not letters
//loop this until you want to exit

//This is the constant
    const CONSTANT: u16 = 6174;

fn main() {

    loop {

        //This is where you enter the number
        //The first numbers can not be 0
        println!("Enter your number: ");

        let mut entered_number = String::new();

        io::stdin()
            .read_line(&mut entered_number)
            .expect("Failed to enter correct information");

        let entered_number: u16 = match entered_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //If they do not enter numbers or the correct amount or letters than have it ask again
        println!("This is your starting number: {}", entered_number);

        let cutup = entered_number.to_string();
        let digits: Vec<char> = cutup.chars().collect();

        println!("{:?}", digits);        

        //let great = digits.clone().sort();
        // let least = digits.clone().reverse();

        // println!("{:?}", great);
        // println!("{:?}", least);


        let num_one = digits[0] as u16;
        let num_two = digits[1] as u16;
        let num_three = digits[2] as u16;
        let num_four = digits[3] as u16;



        if entered_number == CONSTANT {
            println!("This is correct. You won the game.");
            break;
        } else {
            println!("This is incorrect. You lost the game");
            continue;
        }
    };

}
