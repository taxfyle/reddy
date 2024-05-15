use clap::Parser;
use reddy::Args;

fn main() -> redis::RedisResult<()> {
    let args = Args::parse();

    args.run()
}
