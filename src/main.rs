extern crate rand;

use std::io;
use rand::Rng;
use std::collections::VecDeque;

const MAXHEALTH: i32 = 100;
const ACTIONS_PER_TURN: usize = 5;
const BASE_INCOME: i32 = 5;
const BASE_DEVELOPMENT_COST: i32 = 100;
const BASE_DEVELOPMENT_INCREMENT: f32 = 1.5;
const BASE_ATTACK: i32 = 3;
const BASE_TRAINING_COST: i32 = 25;
const BASE_TRAINING_INCREMENT: f32 = 1.1;
const BASE_ENTERTAIN_COST: i32 = 100;
const BASE_ENTERTAIN_FAVOR_INCREASE: i32 = 2;

// Careers a player can have
#[derive(Debug)]
enum Career {
    Politician,
    Soldier,
    Craftsman,
}

#[derive(Debug, PartialEq)]
enum TurnTask {
    Work,
    Develop,
    Socialize,
    Attack,
    Train,
    Office,
}

#[derive(Debug, PartialEq)]
enum AIType {
    Player,
    BasicWorkAI
}

struct TurnAction {
    turn_task: TurnTask,
    target: usize,
}

// Represents an office position
struct Office {
    name: String,
    level: i32,
    salary: i32,
}

// Represents a player in the guild
struct Player {
    health: i32,
    money: i32,
    skill_level: i32,
    develop_level: i32,
    disposition: Career,
    combat_level: i32,
    turn_func: fn(c_player: usize, players: &Vec<Player>) -> VecDeque<TurnAction>,
    ai_type: AIType,
    relationships: Vec<i32>
}

// Generates a random player
fn generate_player(ai_type: AIType) -> Player {
    let career = match rand::thread_rng().gen_range(0_i32, 3_i32) {
        0 => Career::Politician,
        1 => Career::Soldier,
        _ => Career::Craftsman,
    };

    let player = Player {
        health: MAXHEALTH,
        //money: (250 + rand::thread_rng().gen_range(0_i32, 801_i32) as i32),
        money: 250,
        skill_level: 1,
        develop_level: 1,
        disposition: career,
        combat_level: 1,
        turn_func: match ai_type {
            AIType::Player => do_player,
            _ => do_ai_basic
        },
        ai_type: ai_type,
        relationships: Vec::new()
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

// Returns true or false if the item can be afforded
fn can_afford(bank: i32, cost: i32) -> bool {
    cost <= bank
}

// How much money is earned from working
fn calc_work(player: &Player) -> i32 {
    let mut w =
        BASE_INCOME + ((player.develop_level + 1_i32) / 2_i32) * (player.skill_level / 5_i32);
    w = match player.disposition {
        Career::Politician => (w as f32 * 0.9_f32) as i32,
        Career::Craftsman => (w as f32 * 1.1_f32) as i32,
        _ => w,
    };
    w
}

// How much it costs to develop
fn calc_develop(player: &Player) -> i32 {
    (BASE_DEVELOPMENT_COST as f32 * BASE_DEVELOPMENT_INCREMENT.powi(player.develop_level)) as i32
}

// Calculate attack power
fn calc_attack(player: &Player) -> i32 {
    let mut a = BASE_ATTACK + (player.combat_level / 3);
    a = match player.disposition {
        Career::Politician => (a as f32 * 0.95_f32) as i32,
        Career::Soldier => (a as f32 * 1.05_f32) as i32,
        _ => a,
    };
    a
}

// Calculate training cost
fn calc_training(player: &Player) -> i32 {
    (BASE_TRAINING_COST as f32 * BASE_TRAINING_INCREMENT.powi(player.combat_level)) as i32
}

// Calculate socialize cost
fn calc_socialize(player: &Player) -> i32 {
    BASE_ENTERTAIN_COST
}

// Player Function
// Player controls this person
fn do_player(c_player: usize, players: &Vec<Player>) -> VecDeque<TurnAction> {
    let mut task_buf = VecDeque::new();

    // Print menu
    println!("");
    println!("Options (Bank: ${})", players[c_player].money);
    println!("1 - Produce Goods (+${})", calc_work(&players[c_player]));
    println!(
        "2 - Develop Guild (-${}, +Development)",
        calc_develop(&players[c_player])
    );
    println!("3 - Socialize with Player (-${})", calc_socialize(&players[c_player]));
    println!("4 - Attack Player ({} damage)", calc_attack(&players[c_player]));
    println!("5 - Train (-${}, +Attack)", calc_training(&players[c_player]));
    println!("6 - Run for Office");
    println!("7 - Stats");

    // Queue up actions
    while task_buf.len() < ACTIONS_PER_TURN {
        // Get input
        println!("Action {} / {}", task_buf.len() + 1_usize, ACTIONS_PER_TURN);
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        // Process input
        match input.trim() {
            "1" => task_buf.push_front(TurnAction {
                turn_task: TurnTask::Work,
                target: 0,
            }),
            "2" => {
                /*if action_exists(TurnTask::Develop, &task_buf) {
                    println!("Cannot develop more than once per turn!");
                    continue;
                }*/
                task_buf.push_front(TurnAction {
                    turn_task: TurnTask::Develop,
                    target: 0,
                });
            },
            "3" => {
                println!("Type in which player to socialize with");
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");

                let trimmed = input_text.trim();
                let ctarget = match trimmed.parse::<i32>() {
                    Ok(i) => i,
                    Err(..) => -1,
                };
                if ctarget < 0 || ctarget > players.len() as i32 {
                    println!("Target player ID is out of bounds!");
                    continue;
                }
                task_buf.push_front(TurnAction {
                    turn_task: TurnTask::Socialize,
                    target: ctarget as usize,
                });
            },
            "4" => {
                println!("Type in which player to attack");
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");

                let trimmed = input_text.trim();
                let ctarget = match trimmed.parse::<i32>() {
                    Ok(i) => i,
                    Err(..) => -1,
                };
                if ctarget == c_player as i32 {
                    println!("You cannot attack yourself!");
                    continue;
                }
                if ctarget < 0 || ctarget > players.len() as i32 {
                    println!("Target player ID is out of bounds!");
                    continue;
                }
                task_buf.push_front(TurnAction {
                    turn_task: TurnTask::Attack,
                    target: ctarget as usize,
                });
            }
            "5" => {
                task_buf.push_front(TurnAction {
                    turn_task: TurnTask::Train,
                    target: 0,
                });
            }
            "7" => {
                println!("Stats");
                for i in 0..players.len() {
                    println!(
                        "Player {}: ${} | {} HP | {} Combat | {} Skill | {} Develop | {:?} | {:?}",
                        i,
                        players[i].money,
                        players[i].health,
                        players[i].combat_level,
                        players[i].skill_level,
                        players[i].develop_level,
                        players[i].disposition,
                        players[i].ai_type
                    );
                }
            }

            _ => {
                println!("Invalid option or not implemented yet: {}", input);
            }
        }
    }

    task_buf
}

// AI Function
// This AI is stupid and works all day
// Selects actions for that player's turn
fn do_ai_basic(c_player: usize, players: &Vec<Player>) -> VecDeque<TurnAction> {
    let mut task_buf = VecDeque::new();
    while task_buf.len() < ACTIONS_PER_TURN {
        task_buf.push_back(TurnAction {
            turn_task: TurnTask::Work,
            target: 0,
        });
    }
    task_buf
}

// Entry point
fn main() {
    let mut players = Vec::new();
    let mut is_running = true;

    players.push(generate_player(AIType::Player));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));
    players.push(generate_player(AIType::BasicWorkAI));

    for i in 0..players.len() {
        for _ in 0..players.len() {
            players[i].relationships.push(20_i32 + rand::thread_rng().gen_range(0_i32, 60_i32));
        }
    }

    println!("");
    println!("Guild Sim");
    println!("Goal: Hold the highest political office for 5 turns");
    println!("");

    while is_running {
        let mut task_buf : VecDeque<TurnAction> = VecDeque::new();
        println!("Press enter to begin next turn");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        // Process actions
        for c_player in 0..players.len() {
            // If the player is dead, do nothing
            if players[c_player].health <= 0 {
                continue;
            }

            // Run AI or player turn task to get actions
            let mut task_buf = (players[c_player].turn_func)(c_player, &players);

            // Process each action for this player
            while !task_buf.is_empty() {
                let elem = task_buf.pop_back().unwrap();
                match elem.turn_task {
                    // Working is the simpliest task
                    TurnTask::Work => {
                        let income = calc_work(&players[c_player]);
                        players[c_player].money += income;
                        players[c_player].skill_level += 1;
                        println!("Player {} works, earning {} income.", c_player, income);
                    }
                    // Develops the guild the player owns
                    TurnTask::Develop => {
                        /*if action_exists(TurnTask::Develop, &task_buf) {
                            println!("Player {} cannot develop more than once per turn!", c_player);
                            continue;
                        }*/
                        let cost = calc_develop(&players[c_player]);
                        if !can_afford(players[c_player].money, cost) {
                            println!("Player {} cannot afford development. It costs {} and they have {}.", c_player, cost, players[c_player].money);
                            continue;
                        }
                        players[c_player].money -= cost;
                        players[c_player].develop_level += 1;
                        println!(
                            "Player {} upgrades guild to level {}. They have {} left.",
                            c_player, players[c_player].develop_level, players[c_player].money
                        );
                    }
                    // Training the player's attack
                    TurnTask::Train => {
                        let cost = calc_training(&players[c_player]);
                        if !can_afford(players[c_player].money, cost) {
                            println!("Player {} cannot afford training. It costs {} and they have {}.", c_player, cost, players[c_player].money);
                            continue;
                        }
                        players[c_player].money -= cost;
                        players[c_player].combat_level += 1;
                        println!(
                            "Player {} trains guild to level {}. They have {} left.",
                            c_player, players[c_player].combat_level, players[c_player].money
                        );
                    }
                    // Attack another player
                    TurnTask::Attack => {
                        let damage = calc_attack(&players[c_player]);
                        if players[elem.target].health < -100 {
                            players[elem.target].health -= damage;
                            println!("Player {} INSISTS on STILL beating player {} to a pulp EVEN THOUGH THERE'S NOTHING LEFT OF THEM TO ATTACK. ", c_player, elem.target);
                        } else if players[elem.target].health < -50 {
                            players[elem.target].health -= damage;
                            println!("Player {} STILL CONTINUES to attack for player {}'s dead body for {} damage. ", c_player, elem.target, damage);
                        } else if players[elem.target].health < -20 {
                            players[elem.target].health -= damage;
                            println!("Player {} continues attacking player {}'s dead body for {} damage. ", c_player, elem.target, damage);
                        } else if players[elem.target].health <= 0 {
                            println!("Player {} attack player {}'s lifeless corpse for {} damage. ", c_player, elem.target, damage);
                        } else {
                            players[elem.target].health -= damage;
                            println!("Player {} attacks player {} for {} damage. HP Left: {}", c_player, elem.target, damage, players[elem.target].health);
                            if players[elem.target].health <= 0 {
                                println!("Player {} dies.", elem.target);
                            }
                        }
                    }
                    // Default is to fail
                    _ => {
                        println!("Failed to process unknown action {:?}", elem.turn_task);
                    }
                }
            }
        }
    }
}
