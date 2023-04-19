use std::io::Write;
use std::io;
use colored::Colorize;

fn main() {
    house_front()
}

fn house_front() {
    loop {
        let mut input = String::new();

        typewrite("You approach a house. A black tar seeps from underneath the front door. You could go around to the back, look around, or you could try to open the door.");
        prompt("What do you do?");

        io::stdin().read_line(&mut input).expect("Error: Failed to read line.");
        let input = input.trim();

        if input == "open the door" || input == "open door" {
            typewrite("You attempt to open the door first by turning the handle, then by pulling with all your might - but it does not move.");
            break;
        }
        else if input == "look around"{
            typewrite("The house is dilapidated, and looks to have been abandoned for a very long time. It sits somewhere in a swamp, and you don't remember how you got here. ")
        }
        else if input == "go around to back" || input == "go around" || input == "go to back" || input == "go around to the back" {
            typewrite("You shuffle around the white rotting boards of the house to the back. You see another door, as well as a generator ");
            break;
        }
        else {
            println!("{}", "Unrecognized Action".red());
        }
    }      
}

// use for anything that requires a typewriter effect
fn typewrite(words: &str) {
    for c in words.chars() {
        print!("{c}");
        std::io::stdout().flush().expect("Flushing to succeed");
        std::thread::sleep(std::time::Duration::from_millis(35));
    }
    print!("\n");
}

// use for prompts like "what do you do?"
fn prompt(words: &str) {
    println!("{}", words.red().bold());
}