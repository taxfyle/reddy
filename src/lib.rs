use clap::{Parser, Subcommand};
use redis::{Commands as RedisCommmands, RedisError};
use std::process;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    /// URL to use to connect to Redis
    #[arg(short, long, default_value_t = String::from("redis://127.0.0.1/"))]
    pub url: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get memory used by a set of keys
    MemoryUsage {
        /// Key pattern to use for listing keys
        #[arg(default_value_t = String::from("*"))]
        key_pattern: String,
    },
}

impl Args {
    pub fn run(&self) -> Result<(), RedisError> {
        match &self.cmd {
            Commands::MemoryUsage { key_pattern } => {
                println!("run for {}", key_pattern);

                let client = redis::Client::open(self.url.clone())?;
                let mut con = client.get_connection()?;

                let size: Option<u32> = con
                    .keys::<_, Vec<String>>(key_pattern)?
                    .into_iter()
                    .map(|key| {
                        redis::cmd("MEMORY")
                            .arg("USAGE")
                            .arg(&key)
                            .query::<u32>(&mut con)
                            .inspect_err(|err| {
                                println!("got error getting memory usage for key {}: {}", key, err)
                            })
                            .unwrap_or(0)
                    })
                    .reduce(|acc, e| acc + e);

                match size {
                    None => {
                        println!("got no keys for pattern");
                        process::exit(1)
                    }

                    Some(bytes) => {
                        println!("got {} bytes", bytes);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
