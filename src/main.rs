use clap::Parser;
use whisper_growth::example_module;

mod whisper_core;
mod whisper_growth;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let test = example_module::setup_example();

    let another_test = test.map(|whisper_command| {
        return clap::Command::new(whisper_command.name.as_str()).args(
            whisper_command.arguments.map(|argument| {
                clap::Arg::new(argument.long.as_str())
                    .short(argument.short.chars().next().unwrap())
                    .long(argument.long.as_str())
                    .value_name(argument.value_name.as_str())
            }),
        );
    });

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
