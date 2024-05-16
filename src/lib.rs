use clap::{Parser, Subcommand};
use redis::{Commands as RedisCommmands, RedisError};
use std::{io::Write, process, thread, time};

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
    Ping,

    /// Get memory used by a set of keys
    MemoryUsage {
        /// Key pattern to use for listing keys
        #[arg(default_value_t = String::from("*"))]
        key_pattern: String,

        /// Show progress as keys are being scanned
        #[arg(long, default_value_t = false)]
        show_progress: bool,

        /// Show progress and pause after SCAN_BATCH_SIZE keys have been processed
        #[arg(short = 'b', long, default_value_t = 1000)]
        scan_batch_size: u32,
    },
}

impl Args {
    pub fn run(&self) -> Result<(), RedisError> {
        match &self.cmd {
            Commands::MemoryUsage {
                key_pattern,
                show_progress,
                scan_batch_size,
            } => {
                println!("run for {}", key_pattern);

                let client = redis::Client::open(self.url.clone())?;
                let mut con = client.get_connection()?;
                let mut count_con = client.get_connection()?;

                let mut scanned = 0;
                let mut total_size = 0;
                for key in con.scan_match::<&str, String>(&key_pattern)? {
                    let size = redis::cmd("MEMORY")
                        .arg("USAGE")
                        .arg(&key)
                        .query::<u32>(&mut count_con)
                        .inspect_err(|err| {
                            println!("error getting memory usage for key {}: {}", key, err);
                        })
                        .unwrap_or(0);

                    total_size = total_size + size;

                    scanned = scanned + 1;
                    let batch_reached = scanned % scan_batch_size == 0;

                    if *show_progress && batch_reached {
                        print!("\rscanned keys: {} ({} bytes)", scanned, total_size);
                        let _ = std::io::stdout().flush();
                    }

                    if batch_reached {
                        thread::sleep(time::Duration::from_millis(500));
                    }
                }
                println!("\nfinished scanning {} keys", scanned); // Start a new line after the scanned keys
                println!("total size: {} bytes", total_size);

                Ok(())
            }

            Commands::Ping => {
                println!("pinging {}", self.url.clone());

                let client = redis::Client::open(self.url.clone())?;
                let mut con = client.get_connection()?;

                let result = redis::cmd("PING")
                    .query::<String>(&mut con)
                    .inspect_err(|err| {
                        println!("got error pinging redis at {}: {}", self.url.clone(), err);
                        process::exit(1);
                    })?;

                println!("ping answer: {}", result);

                Ok(())
            }
        }
    }
}
