
fn fibonacci(n:u64) -> u64{
    if n <= 0 {
        return 0
    }else if n == 1{
        return 1
    }else{
        return fibonacci(n-1) + fibonacci(n-2)
    }
}

 #[warn(unused_imports)]
use std::io;
use colored::Colorize;
fn main(){
    println!("...............................");
    // we need to break out of the programme
    println!( "{}", "type 'exit' to break out of the programme ".red());
    println!("...............................");
    loop{
    println!("input a positive number");
    // we need to call the input and output library into scope
    // creating a new string
    let mut number = String::new();
    io::stdin().read_line(& mut number).expect("Failed to read line");

    //next, we want to breakout of the loop if we type 'exit' eventually
    if number.trim() == "exit"{
        break;
    }

   let number:u64 =match number.trim().parse(){
        Ok(number) => number,
        Err(_) => {
            println!("please input a positive number");
            continue;
        }
    };
    println!("Fibonnaci ({}) is => {}", number, fibonacci(number));
    
}
}


