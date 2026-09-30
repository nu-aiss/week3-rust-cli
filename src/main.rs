use std::env;

#[derive(Debug)]
enum Network {
    Devnet,
}

#[derive(Debug)]
struct Transfer {
    from: String,
    to: String,
    amount: f64,
    network: Network,
}

impl Transfer {
    fn new(from: String, to: String, amount: f64) -> Result<Transfer, String> {
        if from.trim().is_empty() {
            return Err("Sender address cannot be empty.".to_string());
        }

        if to.trim().is_empty() {
            return Err("Receiver address cannot be empty.".to_string());
        }

        if amount <= 0.0 {
            return Err("Amount must be greater than 0.".to_string());
        }

        Ok(Transfer {
            from,
            to,
            amount,
            network: Network::Devnet,
        })
    }

    fn print_info(&self) {
        println!("Transfer created successfully!");
        println!("From: {}", self.from);
        println!("To: {}", self.to);
        println!("Amount: {} SOL", self.amount);

        match self.network {
            Network::Devnet => println!("Network: Devnet"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage:");
        println!("cargo run -- transfer <FROM> <TO> <AMOUNT>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "transfer" => {
            let from = args[2].clone();
            let to = args[3].clone();

            let amount: f64 = match args[4].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Error: amount must be a number.");
                    return;
                }
            };

            match Transfer::new(from, to, amount) {
                Ok(transfer) => transfer.print_info(),
                Err(error) => println!("Error: {}", error),
            }
        }

        _ => {
            println!("Unknown command.");
            println!("Available command: transfer");
        }
    }
}
