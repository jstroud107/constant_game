use std::{io, char};

//You can only enter 4 numbers
//Must be numbers not letters
//loop this until you want to exit

//This is the constant
const CONSTANT: u32 = 6174;

//This breaks the number into a vector to get the high and low digits
fn breaking(n:u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

//This changes the Vector back into the number to be used
fn combining(n:Vec<u32>) -> String {
    n.into_iter()
        .map(|c| char::from_digit(c,10).unwrap())
        .collect()
}

fn iter_agg(v:u32)-> u32{
    let mut high = breaking(v);
    high.sort_by(|a,b| b.cmp(a));

    let mut low = breaking(v);
    low.sort_by(|a,b| a.cmp(b));

    let x = combining(high);
    let y = combining(low);

    let num_high: u32 = x.parse().unwrap();
    let num_low: u32 = y.parse().unwrap();

    let value: u32 = num_high - num_low;

    return value
}

fn main() {

    let mut count = 1;
    
    //This is where you enter the number
    //The first numbers can not be 0
    println!("Enter your number: ");

    let mut entered_number = String::new();

    io::stdin()
        .read_line(&mut entered_number)
        .expect("Failed to enter correct information");

    //Going to have to do a match or something so this doesn't crash
    let mut entered_number: u32 = entered_number.trim().parse().expect("You have to enter a number!");

    loop {    

        //Make an if statement to get the length of the numbers if it is over four or under redo
        //Make it where you can't start off with a 0

        //If they do not enter numbers or the correct amount or letters than have it ask again
        println!("This is your starting number: {}", entered_number);

        entered_number = iter_agg(entered_number);

        //This section has to keep repeating itself until it gets the right iteration

        println!("{}", entered_number);

        //This is the end of the iteration part which I'm going to make into a function

        if entered_number == CONSTANT {
            //This is going to show how many times it took for it to be right
            println!("It took this many iterations {}", count);
            break;
        } else {
            println!("This is iteration {}", count);
            count += 1;
            continue;
        }
    };

}
