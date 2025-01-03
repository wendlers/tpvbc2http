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

mod config;
mod server;

fn main() {
    colog::init();

    let path = env::current_dir().unwrap();
    let args = Args::parse();
    let bind_addr = format!("0.0.0.0:{}", args.port);
    let mut tpvbcdir = format!("{}/http/testing/", path.display());

    if args.tpvbcdir.len() > 0 {
        tpvbcdir = args.tpvbcdir;
    }

    let mut statdir = format!("{}/http/static/", path.display());

    if args.statdir.len() > 0 {
        statdir = args.statdir;
    }

    info!("tpvbc2http\ncwd: {}\nbind_addr: {}\ntpvbcdir: {}\nstatdir: {}", 
        path.display(), 
        bind_addr,
        tpvbcdir,
        statdir
    );

    let mut cfg = config::Server::new();
    cfg.set_bind_addr(bind_addr);
    cfg.set_tpv_bcast_file_loc(tpvbcdir);
    cfg.set_static_content_loc(statdir);

    server::start(cfg);
}
