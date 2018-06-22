extern crate rand;

use std::io;
use rand::Rng;

const MAXHEALTH : i32 = 100;

// Careers a player can have
#[derive(Debug)]
enum Career {
    Politician,
    Soldier,
    Craftsman
}

// Represents an office position
struct Office {
    name: String,
    level: i32,
    salary: i32
}

// Represents a player in the guild
struct Player {
    health: i32,
    money : i32,
    skill_level : i32,
    develop_level : i32,
    disposition: Career,
    combat_level : i32
}

// Generates a random player
fn GeneratePlayer() -> Player {

    let career = match rand::thread_rng().gen_range(0_i32, 3_i32) {
        0 => Career::Politician,
        1 => Career::Soldier,
        _ => Career::Craftsman,
    };

    let player = Player {
        health: MAXHEALTH,
        money: (200 + rand::thread_rng().gen_range(0_i32, 801_i32) as i32),
        skill_level: 1,
        develop_level: 1,
        disposition: career,
        combat_level : 1
    };

    player
}

// Entry point
fn main() {
    
    let mut players = Vec::new();
    let mut is_running = true;

    for _ in 0..8 {
        players.push(GeneratePlayer());
    }

    while is_running {

        println!("");
        println!("Options");
        println!("1 - Work at Factory");
        println!("2 - Develop Factory");
        println!("3 - Socialize with Player");
        println!("4 - Attack Player");
        println!("5 - Train");
        println!("6 - Run for Office");
        println!("7 - Stats");

        let mut input = String::new();
        io::stdin().read_line(&mut input);

        match input.trim() {
            "1" => {
                println!("Werk");
            }

            "7" => {
                println!("Stats");
                for i in 0..8 {
                    println!("Player {}: ${} | {} HP | {} Combat | {} Skill | {} Develop | {:?}", i, 
                        players[i].money, players[i].health, 
                        players[i].combat_level, players[i].skill_level, players[i].develop_level, players[i].disposition);
                }
            }

            _ => {
                println!("Invalid option or not implemented yet: {}", input);
                continue;
            }
        }


    }

}
