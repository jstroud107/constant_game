use std::io;

//You can only enter 4 numbers
//Must be numbers not letters
//loop this until you want to exit

//This is the constant
const CONSTANT: u16 = 6174;

//This breaks the number into a vector to get the high and low digits
fn breaking(n:u16) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn combining(n:Vec<u32>) -> String {
    n.into_iter()
        .map(|c| std::char::from_digit(c,10).unwrap())
        .collect()
}

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

        //Make an if statement to get the length of the numbers if it is over four or under redo
        //Make it where you can't start off with a 0

        //If they do not enter numbers or the correct amount or letters than have it ask again
        println!("This is your starting number: {}", entered_number);

        let mut high = breaking(entered_number);
        high.sort_by(|a,b| b.cmp(a));

        let mut low = breaking(entered_number);
        low.sort_by(|a,b| a.cmp(b));

        let x = combining(high);
        let y = combining(low);

        println!("{}", x);
        println!("{}", y);


        if entered_number == CONSTANT {
            println!("This is correct. You won the game.");
            break;
        } else {
            println!("This is incorrect. You lost the game");
            continue;
        }
    };

}
