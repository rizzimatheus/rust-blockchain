use std::{io, process};
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("\nInput a miner address: ");
    io::stdout().flush().expect("Failed to flush!");
    io::stdin().read_line(&mut miner_addr).expect("Failed to read user input!");

    println!("\nDifficulty:");
    io::stdout().flush().expect("Failed to flush!");
    io::stdin().read_line(&mut difficulty).expect("Failed to read user input!");
    let diff = difficulty.trim().parse::<u32>().expect("Need an integer");

    println!("\nGenerating genesis block...");
    let mut chain = blockchain::Chain::new(diff, miner_addr.trim().to_string());

    loop {
        println!("\nMenu:");
        println!("[1] New transaction");
        println!("[2] Mine block");
        println!("[3] Change difficulty");
        println!("[4] Change reward");
        println!("[0] Exit");
        io::stdout().flush().expect("Failed to flush!");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read user input!");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("\nExiting...");
                process::exit(0);
            },
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("\nEnter sender address:");
                io::stdout().flush().expect("Failed to flush!");
                io::stdin().read_line(&mut sender).expect("Failed to read user input!");
                println!("\nEnter receiver address:");
                io::stdout().flush().expect("Failed to flush!");
                io::stdin().read_line(&mut receiver).expect("Failed to read user input!");
                println!("\nEnter amount:");
                io::stdout().flush().expect("Failed to flush!");
                io::stdin().read_line(&mut amount).expect("Failed to read user input!");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("\nTransaction added!"),
                    false => println!("\nTransaction failed!"),
                }

            },
            2 => {
                println!("\nGenerating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("\nBlock generated successfully!"),
                    false => println!("\nBlock generation failed!"),
                }
            },
            3 => {
                let mut new_diff = String::new();

                println!("\nEnter new difficulty:");
                io::stdout().flush().expect("Failed to flush!");
                io::stdin().read_line(&mut new_diff).expect("Failed to read user input!");

                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("\nUpdated difficulty!"),
                    false => println!("\nFailed update!"),
                }
            },
            4 => {
                let mut new_reward = String::new();

                println!("\nEnter new difficulty:");
                io::stdout().flush().expect("Failed to flush!");
                io::stdin().read_line(&mut new_reward).expect("Failed to read user input!");

                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("\nUpdated reward!"),
                    false => println!("\nFailed update!"),
                }
            },
            _ => println!("\nInvalid choice, try again!"),
        }
    }
}
