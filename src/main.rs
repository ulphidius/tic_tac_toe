mod ai_train;

use clap::{ App, Arg, SubCommand, ArgMatches, AppSettings };
// use tic_tac_toe_q_learning::Environment;
use crate::ai_train::train_ai_agent_vs_agent;

fn main() {
    let matches = App::new("tic_tac_toe")
        .version("0.1.0")
        .about("tic tac toe game with reforcement learning AI")
        .author("LAURENT Louis")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("play")
            .about("Game launcher command")
            .arg(Arg::with_name("console")
                .short("c")
                .visible_alias("cmd")
                .long("console")
                .required_unless("graphic")
                .conflicts_with("graphic")
                .help("Launch game inside the console")
            )
            .arg(Arg::with_name("graphic")
                .short("g")
                .visible_alias("gui")
                .long("graphic")
                .required_unless("console")
                .conflicts_with("console")
                .help("Launch game in graphic mode")
            )
            .arg(Arg::with_name("ai_path")
                .short("p")
                .long("ai-path")
                .value_name("AI_PATH")
                .required(true)
                .takes_value(true)
                .help("Path to where AI files are store")
            )
        )
        .subcommand(SubCommand::with_name("train")
            .about("Train AI with reforcement learning. The result if this command is the creation of a file with the trained AI")
            .arg(Arg::with_name("again_agent")
                .visible_alias("aa")
                .long("again-agent")
                .required_unless("again_random")
                .conflicts_with("again_random")
                .help("Train AI again another AI. The experience of both are use for training")
            )
            .arg(Arg::with_name("again_random")
                .visible_alias("ar")
                .long("again-random")
                .required_unless("again_agent")
                .conflicts_with("again_agent")
                .help("Train AI again random moves")
            )
            .arg(Arg::with_name("filename")
                .short("f")
                .long("filename")
                .required(true)
                .takes_value(true)
                .value_name("FILENAME")
                .help("Filename of training output file")
            )   
            .arg(Arg::with_name("GAMES_TO_PLAY")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Number of game to play")
            )
        )
        .get_matches();

    arguments_handler(matches).unwrap();
}

fn arguments_handler(arguments: ArgMatches) -> Result<(), &'static str> {
    return match arguments.subcommand() {
        ("play", Some(args)) => launcher_arguments_handler(args),
        ("train", Some(args)) => training_arguments_handler(args),
        _ => Ok(())
    };
}

fn launcher_arguments_handler(arguments: &ArgMatches) -> Result<(), &'static str> {
    if ! arguments.is_present("console") && ! arguments.is_present("graphic") {
        panic!("The 2 options console and graphic are empty ! That shouldn't be allow");
    }

    if ! arguments.is_present("ai_path") {
        panic!("The path to AI must be define");
    }
    
    if arguments.is_present("console") {
        println!("Start console mode");
    }

    if arguments.is_present("graphic") {
        println!("Start graphic mode");
    }

    unimplemented!();
}

fn training_arguments_handler(arguments: &ArgMatches) -> Result<(), &'static str> {
    if ! arguments.is_present("again_agent") && ! arguments.is_present("again_random") {
            panic!("The 2 options again-agent and again-random are empty ! That shouldn't be allow");
    }

    if arguments.is_present("again_agent") && arguments.is_present("again_random") {
        panic!("Only one of these arguments (again-agent, again-random) must be declare");
    }

    if ! arguments.is_present("filename") {
        panic!("No filename defined !");
    }

    if ! arguments.is_present("GAMES_TO_PLAY") {
        panic!("The number of games to do for training isn't defined !");
    }

    let filename = match arguments.value_of("filename") {
        Some(value) => value,
        None => panic!("Filename isn't defined")
    };

    let number_of_games: u16 = match arguments.value_of("GAMES_TO_PLAY") {
        Some(value) => value.parse::<u16>().unwrap(),
        None => panic!("GAME_TO_PLAY isn't defined")
    };

    return train_ai_agent_vs_agent(number_of_games, filename);
}

fn console_game_main() {
    // let environment = Environment::new();
    unimplemented!();
}
