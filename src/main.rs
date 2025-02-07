use std::env;
use clap::Parser;
use log::info;

/// Serve TraingPeaks Virtual broadcast files (JSON) via HTTP.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port on which to run the HTTP server
    #[arg(short, long, default_value_t = 8080)]
    port: u32,

    /// TrainingPeaks Virtual Broadcast directory
    #[arg(short, long, default_value_t = String::new())]
    tpvbcdir: String,

    /// Static HTML directory
    #[arg(short, long, default_value_t = String::new())]
    statdir: String,
}

mod server;

fn main() {
    colog::init();

    let path = env::current_dir().unwrap();
    let args = Args::parse();
    let mut tpvbcdir = format!("{}/http/testing/", path.display());

    if args.tpvbcdir.len() > 0 {
        tpvbcdir = args.tpvbcdir;
    }

    info!("tpvbc2http\ncwd: {}\nport: {}\ntpvbcdir: {}", 
        path.display(), 
        args.port,
        tpvbcdir,
    );

    let s = server::Instance::new();
    s.start("0.0.0.0", &format!("{}", args.port), tpvbcdir);
}
