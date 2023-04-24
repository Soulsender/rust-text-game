use std::io::Write;
use std::io;
use colored::Colorize;

fn main() {

    title();
    house_front();
}

// ===================
// STORY FUNCTIONS 
// ===================

fn house_back() {
    // explain location
    typewrite("The back of the house is disgusting. Broken lamps, splinted shovels, and junk line the chipped and crusted picket fence. You see another door into the house, as well as a generator. You could also go back to the front of the house.");

    loop {
        // reset the user input
        let mut input = String::new();
        
        // prompt, get input, and sanitize
        prompt("What do you do?");
        io::stdin().read_line(&mut input).expect("Error: Failed to read line.");
        let input = input.trim();

        if input == "look around" {
            typewrite("The back of the house is disgusting. Broken lamps, splinted shovels, and junk line the chipped and crusted picket fence. Nobody has been here for a very long time. The trees create an ominous circle of darkness, and you cannot see past the courtyard of junk and fence, You see another door into the house, as well as a generator.");
            break;
        }
        else if input == "go around to front" || input == "go around" || input == "go to front" || input == "go around to the front" || input == "go to the front" {
            typewrite("You shuffle around the white rotting boards of the house to the front.");
            house_front();
            break;
        }
        else if input == "go to generator" || input == "use generator" {

            typewrite("You somehow recognize the generator as an old model, although you don't know why you know that. It takes three pulls until the machine starts. It begins to groan and shake as the the fuel is ignited. You recognize that it won't run forever, but it should be stable for the time being.");
            break;
        }
        else {
            // called if invalid action is used
            println!("{}", "Unrecognized Action".red());
        }
    }      
}

fn house_front() {
    // explain location
    typewrite("You approach a house. A black, foul-smelling tar seeps from underneath the front door. It is dark, but you are unable to hazard a guess for the exact time of day. You could go around to the back, look around, or you could try to open the door.");

    loop {
        // reset the user input
        let mut input = String::new();

        // prompt, get input, and sanitize
        prompt("What do you do?");
        io::stdin().read_line(&mut input).expect("Error: Failed to read line.");
        let input = input.trim();

        if input == "look around"{
            typewrite("The house is dilapidated, and looks to have been abandoned for a very long time. It sits somewhere in a swamp, and you don't remember how you got here. The house is hideous and feels like a relic of another time. You could go around to the back or you could try to open the door.");
        }
        else if input == "open the door" || input == "open door" {
            typewrite("You attempt to open the door first by turning the handle, then by pulling with all your might - but it does not move. It does not feel as if it is locked, but rather that it is being blocked.");
        }
        else if input == "go around to back" || input == "go around" || input == "go to back" || input == "go around to the back" {
            typewrite("You shuffle around the white rotting boards of the house to the back.");
            house_back();
            break;
        }
        else {
            // called if invalid action is used
            println!("{}", "Unrecognized Action".red());
        }
    }      
}


// ===================
// TECHNICAL FUNCTIONS 
// ===================


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

// show the title sequence
fn title() {
    print!("{}[2J", 27 as char);
    println!("{}", "RUST ADVENTURE GAME".blue().bold());
    println!("{}", "By Soulsender".blue());
    println!("{}", "The general syntax is: \"go to front\", \"look around\", \"open door\", \"use object\".\n\n\n".blue());
    std::io::stdout().flush().expect("Flushing to succeed");
    std::thread::sleep(std::time::Duration::from_millis(3000));
}