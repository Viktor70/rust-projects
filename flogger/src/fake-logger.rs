use argh::FromArgs;
use lib::FakeLogger;

#[derive(FromArgs)]
/// find deleted files.
#[argh(help_triggers("-h", "--help"))]
struct Args {
    /// path and file name for logs
    #[argh(positional, default = "String::from(\"fake.log\")")]
    path: String,

    /// interval for each line in ms
    #[argh(option, short = 'p', default = "100")]
    interval: u16,
}

fn main() {
    let args: Args = argh::from_env();
    println!("path is     {0:?}", args.path);
    println!("interval is {0} ms", args.interval);
    let logger = FakeLogger::new(&args.path).expect("Failed to create logger");
    logger.start(args.interval);
}
