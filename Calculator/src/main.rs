use std::io::{self, Write};

fn main() {
    
    loop {

        println!("1.Add");
        println!("2.Sub");
        println!("3.Mul");
        println!("4.Div");
        println!("5.Mod");
        
        println!("6.Exit");

        let choice = read_i32("Choice an option: ");

        if choice == 6 {
            println!("Good Bye!");
            break;
        }

        let a = read_i32("Enter a number: ");
        let b = read_i32("Enter a number: ");

        let res = match choice {

            1 => a + b , 
            2 => a - b ,
            3 => a * b ,
            4 => {
                if b == 0 {
                    println!("Cannot divide by zero!");
                    continue;
                }
                a / b

            }
            5 => a % b ,
            _ => {
                println!("Invalid choice!");
                continue;
            }

        };

        println!("Result: {}" , res);
    }
    


}


fn read_i32(msg: &str) -> i32 {

    let mut input = String::new();
    print!("{}" , msg);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()


}
