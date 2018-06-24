extern crate rand;

use std::io;
use rand::Rng;
use std::collections::VecDeque;

const MAXHEALTH : i32 = 100;
const ACTIONS_PER_TURN : usize = 5;
const BASE_INCOME: i32 = 10;
const BASE_DEVELOPMENT_COST : i32 = 100;
const BASE_DEVELOPMENT_INCREMENT : f32 = 1.5;

// Careers a player can have
#[derive(Debug)]
enum Career {
    Politician,
    Soldier,
    Craftsman
}

#[derive(Debug)]
#[derive(PartialEq)]
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

// Returns false if the action does not yet exist
fn action_exists(l_task: TurnTask, queue: &VecDeque<TurnAction>) -> bool {
    for task in queue.iter() {
        if task.turn_task == l_task {
            return true;
        }
    }
    false
}

fn can_afford(bank: i32, cost: i32) -> bool {
    cost <= bank
}

// How much money is earned from working
fn calc_work(player: &Player) -> i32 {
    let mut w = BASE_INCOME + ((player.develop_level + 1_i32) / 2_i32) * (player.skill_level / 5_i32); 
    w = match player.disposition {
        Career::Politician => (w as f32 * 0.9_f32) as i32,
        Career::Craftsman => (w as f32 * 1.1_f32) as i32,
        _ => w
    };
    w
}

// How much it costs to develop
fn calc_develop(player: &Player) -> i32 {
    (BASE_DEVELOPMENT_COST as f32 * BASE_DEVELOPMENT_INCREMENT.powi(player.develop_level)) as i32
}

// Entry point
fn main() {
    
    let mut players = Vec::new();
    let mut is_running = true;

    for _ in 0..8 {
        players.push(GeneratePlayer());
    }

            println!("");
        println!("Guild Sim");
        println!("Goal: Hold the highest political office for 5 turns");
        println!("");

    while is_running {

        let mut taskBuf = VecDeque::new();

        // Print menu
        println!("");
        println!("Options (Bank: ${})", players[0].money);
        println!("1 - Produce Goods (+${})", calc_work(&players[0]));
        println!("2 - Develop Guild (-${}, +Development)", calc_develop(&players[0]));
        println!("3 - Socialize with Player");
        println!("4 - Attack Player");
        println!("5 - Train");
        println!("6 - Run for Office");
        println!("7 - Stats");

        // Queue up actions
        while taskBuf.len() < ACTIONS_PER_TURN {

            // Get input
            println!("Action {} / {}", taskBuf.len() + 1_usize, ACTIONS_PER_TURN);
            let mut input = String::new();
            io::stdin().read_line(&mut input);

            // Process input 
            match input.trim() {
                "1" => taskBuf.push_front(TurnAction { turn_task: TurnTask::Work, target: 0 } ),
                "2" => {
                    if action_exists(TurnTask::Develop, &taskBuf) {
                        println!("Cannot develop more than once per turn!");
                        continue;
                    }
                    taskBuf.push_front(TurnAction { turn_task: TurnTask::Develop, target: 0 } );
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
                    players[c_player].money += income;
                    players[c_player].skill_level += 1;
                    println!("Player {} works, earning {} income.", c_player, income);
                }
                TurnTask::Develop => {
                    let cost = calc_develop(&players[c_player]);
                    if !can_afford(players[c_player].money, cost) {
                        println!("Player {} cannot afford development. It costs {} and they have {}.", c_player, cost, players[c_player].money);
                        continue;
                    }
                    players[c_player].money -= cost;
                    players[c_player].develop_level += 1;
                    println!("Player {} upgrades guild to level {}. They have {} left.", c_player,  players[c_player].develop_level, players[c_player].money);
                }
                _ => {
                    println!("Failed to process unknown action {:?}", elem.turn_task);
                }
            }
        }


    }

}
