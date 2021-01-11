use tic_tac_toe_q_learning::{ train_agent_again_agent };
use std::{fs::File, path::Path};
use std::io::prelude::*;

const MAX_GAMES: u16 = 1000;

pub fn train_ai_agent_vs_agent(number_of_games: u16, filename: &str) -> Result<(), &'static str> {
    if number_of_games > MAX_GAMES {
        return Err("Cannot train AI with more than 1000 game");
    }

    if Path::exists(Path::new(filename)) {
        return Err("The file already exist");
    }

    let (trained_ai, _) = train_agent_again_agent(number_of_games / 2);
    
    let serialized_q_table = serde_json::to_string(&trained_ai.q_table).unwrap();
    
    let mut trained_ai_file = File::create(filename).unwrap();
    trained_ai_file.write_all(serialized_q_table.as_bytes()).unwrap();

    return Ok(());
}
