extern crate rand;

use std::io;
use rand::Rng;
use std::collections::VecDeque;

const MAXHEALTH : i32 = 100;
const ACTIONS_PER_TURN : usize = 5;
const BASE_INCOME: i32 = 10;

// Careers a player can have
#[derive(Debug)]
enum Career {
    Politician,
    Soldier,
    Craftsman
}

#[derive(Debug)]
enum TurnTask {
    Work,
    Develop,
    Socialize,
    Attack,
    Train,
    Office
}

struct TurnAction {
    turn_task : TurnTask,
    target : i32
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

// How much money is earned from working
fn calc_work(player: &Player) -> i32 {
    BASE_INCOME + ((player.develop_level + 1_i32) / 2_i32) * (player.skill_level / 5_i32) 
}

// Entry point
fn main() {
    
    let mut players = Vec::new();
    let mut is_running = true;

    for _ in 0..8 {
        players.push(GeneratePlayer());
    }

    while is_running {

        let mut taskBuf = VecDeque::new();

        // Print menu
        println!("");
        println!("Options");
        println!("1 - Work at Factory (${})", calc_work(&players[0]));
        println!("2 - Develop Factory");
        println!("3 - Socialize with Player");
        println!("4 - Attack Player");
        println!("5 - Train");
        println!("6 - Run for Office");
        println!("7 - Stats");

        // Queue up actions
        while taskBuf.len() < ACTIONS_PER_TURN {

            // Get input
            println!("Action {} / {}", taskBuf.len(), ACTIONS_PER_TURN);
            let mut input = String::new();
            io::stdin().read_line(&mut input);

            // Process input 
            match input.trim() {
                "1" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Work, target: 0 } ),
                "2" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Develop, target: 0 } ),
                "3" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Socialize, target: 0 } ),
                "4" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Attack, target: 0 } ),
                "5" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Train, target: 0 } ),
                "6" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Office, target: 0 } ),
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
                }
            }
        }

        // Process actions
        // Player 0 is us
        let c_player = 0; 
        while !taskBuf.is_empty() {
            let elem = taskBuf.pop_back().unwrap();
            match elem.turn_task {
                TurnTask::Work => {
                    let income = calc_work(&players[c_player]);
                    players[c_player].money += income
                    players[c_player].skill_level += 1;
                    println!("Player {} works, earning {} income.", c_player, income);
                }
                _ => {
                    println!("Failed to process unknown action {:?}", elem.turn_task);
                }
            }
        }


    }

}
