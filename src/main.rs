/*  1. Name:
       Braxton Meyer
    2. Software Name:
       Personal Portfolio, Character Stat Creator
    3. Software Description:
       This program generates basic stat values for Dungeons and Dragons 5th edition characters
*/


struct Race {
    name:String,
    stat_bonuses:Vec<i32>,
}

struct Character {
    name:String,
    race:Race,
    stats:Vec<i32>,
}

fn get_string(prompt:String) -> String {
    /* This function prompts the user for a string and returns it. created
    with a lot of help from tutorialspoint's Input Output section. */

    // Initialize the string and present the prompt to the user
    let mut line = String::new();
    println!("{}",prompt);

    // Read the line and convert it to a string before returning
    std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn get_int(prompt:String) -> i32 {
    /* This function prompts the user for an int and returns it. created
    with a lot of help from tutorialspoint's Input Output section */

    // Initialize the string that will contain an int and prompt the user
    let mut line = String::new();
    println!("{}",prompt);

    //read the line, convert to an integer, and return
    std::io::stdin().read_line(&mut line).unwrap();
    let integer = line.trim().parse().unwrap();
    return integer;
}

fn select_race() -> Race {

    //Start by initializing race options TODO: Make this draw from a file instead
    let dwarf = Race{
        name:String::from("Dwarf"),
        stat_bonuses:vec![0, 0, 2, 0, 0 ,0]
    };
    let human = Race{
        name:String::from("Human"),
        stat_bonuses:vec![1, 1, 1, 1, 1 ,1]
    };
    let orc = Race{
        name:String::from("Orc"),
        stat_bonuses:vec![2, 0, 1, 0, 0 ,0]
    };

    // Prompt for race choice and return that one.
    println!("Race choices: 1-dwarf, 2-human, 3-orc");
    let prompt = String::from("Enter desired race number. (1-3) > ");
    let selected_race = get_int(prompt);
    // This will loop until a valid race is selected
    loop {
        match selected_race {
            1 => return dwarf,
            2 => return human,
            3 => return orc,
            _=> println!("That option is invalid, please enter a number between 1 and 3."),
        }
    }

}

fn pickstats(mut current_character:Character) -> Character{
    
    // A list of available values an empty list of values that have been used, and a list of each stat's name.
    let values:[i32; 6] = [8,10,12,13,14,15];
    let mut used_values = vec!();
    let statlist = ["STR","DEX","CON","INT","WIS","CHA"];  

    let mut count = 0;
    for i in &mut current_character.stats {
        // initialize the choice in this scope so it can be used after
        let mut choice = 0;
        // This ensures that we only get values from the values list
        let mut valid_input = false;
        while valid_input == false{
            println!("Selecting stat value for {}.",statlist[count]);
            // TODO: remove "available" values from the list after selection
            println!("Available values are {:?}",values);
            let prompt = String::from("Enter desired stat value. (1-6) > ");
            choice = get_int(prompt);
            for value in values {
                // If the choice is a valid value and hasn't been used yet, it is used!
                if value == choice && !used_values.contains(&choice) {
                    valid_input = true;
                    used_values.push(choice);
                    break;
                }
            }
            // Otherwise, the value isn't valid or has already been used
            if valid_input != true && used_values.contains(&choice){
                println!("You've already used that number!");
            }
            else if valid_input !=true {
                println!("That number is not an option.");
            }
            println!("");
        }

        // Add the selected stat value to the character's stat, which is handily the loop iterator
        *i += choice;
        // Unfortunately since the iterator is a value in the statlist, we also need this to keep tack of where we are
        count += 1;
    }

    return current_character;
}

fn apply_racial_bonuses(mut current_character:Character) -> Character{
    
    
    // Create iterator for all the bonuses applied based on race
    let bonus_iter = current_character.race.stat_bonuses.iter();

    // For each bonus, apply to the respective character's stat.
    let mut i = 0;
    for val in bonus_iter {
        current_character.stats[i] += *val;
        i += 1;
    }

    return current_character;
}


fn create_character(race:Race) -> Character {

    // Characters are initialized with a name, race, and empty stat values.
    let mut current_character = Character {
        name:get_string(String::from("Enter your name:")),
        race:race,
        stats:vec![0,0,0,0,0,0],
    };

    current_character = pickstats(current_character);

    current_character = apply_racial_bonuses(current_character);

    return current_character;
}

fn display_character(character:Character){
    println!("NAME: {}",character.name);
    println!("RACE: {}",character.race.name);
    // Would be nicer to put this in a loop, but it would require additional work to get the labels functioning.
    println!("STR: {}",character.stats[0]);
    println!("DEX: {}",character.stats[1]);
    println!("CON: {}",character.stats[2]);
    println!("INT: {}",character.stats[3]);
    println!("WIS: {}",character.stats[4]);
    println!("CHA: {}",character.stats[5]);
}

fn main() {

    println!("Welcome to the character creator! First select a race.");
    let current_race = select_race();

    let current_character = create_character(current_race);

    println!("Character finished, here's your basic stats:");
    println!();
    display_character(current_character);
}

