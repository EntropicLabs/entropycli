use clap::{Parser, Subcommand};
use ecvrf_rs::SecretKey;
use rand::Rng;


#[derive(Debug, Parser, Clone)]
pub struct KeyCommandOptions {
    #[clap(subcommand)]
    pub command: KeyCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum KeyCommand {
    #[clap(about = "Generate a key")]
    New{
        #[clap(short, long)]
        #[clap(default_value = "1")]
        /// The name of the key
        num: u64
    },
}

pub fn key_cmd(options: &KeyCommandOptions) {
    match options.command {
        KeyCommand::New{num} => new_key(num),
    }
}

fn new_key(num: u64) {
    let mut rng = rand::thread_rng();
    for _ in 0..num {
        let mut key = [0u8; 32];
        rng.fill(&mut key);
    
        let sk = SecretKey::new(&key);
        let pk = sk.extract_public_key_and_scalar().unwrap().0;
    
        println!("Public key: {}", pk);
        println!("Secret key: {}", sk);
    }
}