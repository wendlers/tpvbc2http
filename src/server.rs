use log::{info, warn};

use std::net::TcpListener;
use std::fs;
use std::{path::Path, sync::mpsc};
use std::time::Duration;
use std::sync::Mutex;

use notify::{Event, RecursiveMode, Result, Watcher};

use tinyhttp::prelude::*;

use crate::config;

static BCAST_FILE_LOC: Mutex<String> = Mutex::new(String::new());

fn response_from_file(fname: &str, wait_for_change: u64) -> Response {

    let mut ok_status = "HTTP/1.1 200 OK\r\n".to_string();

    if wait_for_change > 0 {
        info!("file '{}' requested when changed (waiting {} sec.)", fname, wait_for_change);

        let (tx, rx) = mpsc::channel::<Result<Event>>();
        let mut watcher = notify::recommended_watcher(tx).unwrap();
        let changed;

        // fixme: error handling
        watcher.watch(Path::new(fname), RecursiveMode::NonRecursive).unwrap();

        match rx.recv_timeout(Duration::from_secs(wait_for_change)) {
            Ok(_) => changed = true,
            Err(_) => changed = false,
        }

        if changed {
            // thread::sleep(Duration::from_millis(500));
            match rx.recv_timeout(Duration::from_secs(wait_for_change)) {
                Ok(_) => info!("file '{}' changed", fname),
                Err(_) => warn!("file '{}' changed but no second event", fname),
            }
            
        } else {
            warn!("file '{}' did not change within {} sec.", fname, wait_for_change);
            ok_status = "HTTP/1.1 202 Accepted\r\n".to_string();
        }    
    } else {
        info!("file '{}' requested", fname);
    }
    
    match fs::read_to_string(fname) {
        Ok(content) => Response::new()
                                .status_line(ok_status)
                                .mime("text/json")
                                .body(content[3..].as_bytes().to_vec()),
        Err(err) => Response::new()
                                .status_line("HTTP/1.1 404 Not Found\r\n")
                                .mime("text/plain")
                                .body(err.to_string().as_bytes().to_vec()),
    }
}

fn parse_timeout(wildcard: &String) -> u64 {
    match wildcard.parse() {
        Ok(timeout) => timeout,
        Err(_) => 0,
    }
}

fn full_path(parital: &str) -> String {
    let p = BCAST_FILE_LOC.lock().unwrap();
    let mut fp = p.clone();
    fp.push_str(parital);
    fp.to_string()
}

#[get("/bcast/focus")]
fn get_focus() -> Response {
    response_from_file(&full_path("focus.json"), 0)
}

#[get("/bcast/focus/blocking/:")]
fn get_focus_blocking(req: Request) -> Response {
    response_from_file(&full_path("focus.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/nearest")]
fn get_nearest() -> Response {
    response_from_file(&full_path("nearest.json"), 0)
}

#[get("/bcast/nearest/blocking/:")]
fn get_nearest_blocking(req: Request) -> Response {
    response_from_file(&full_path("nearest.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/event")]
fn get_event() -> Response {
    response_from_file(&full_path("event.json"), 0)
}

#[get("/bcast/event/blocking/:")]
fn get_event_blocking(req: Request) -> Response {
    response_from_file(&full_path("event.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/entries")]
fn get_entries() -> Response {
    response_from_file(&full_path("entries.json"), 0)
}

#[get("/bcast/entries/blocking/:")]
fn get_entries_blocking(req: Request) -> Response {
    response_from_file(&full_path("entries.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/groups")]
fn get_groups() -> Response {
    response_from_file(&full_path("groups.json"), 0)
}

#[get("/bcast/groups/blocking/:")]
fn get_groups_blocking(req: Request) -> Response {
    response_from_file(&full_path("groups.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/resultsindv")]
fn get_resultsindv() -> Response {
    response_from_file(&full_path("resultsIndv.json"), 0)
}

#[get("/bcast/resultsindv/blocking/:")]
fn get_resultsindv_blocking(req: Request) -> Response {
    response_from_file(&full_path("resultsIndv.json"), parse_timeout(req.get_wildcard().unwrap()))
}

#[get("/bcast/resultsteam")]
fn get_resultsteam() -> Response {
    response_from_file(&full_path("resultsTeam.json"), 0)
}

#[get("/bcast/resultsteam/blocking/:")]
fn get_resultsteam_blocking(req: Request) -> Response {
    response_from_file(&full_path("resultsTeam.json"), parse_timeout(req.get_wildcard().unwrap()))
}

pub(crate) fn start(cfg: config::Server) {
    {
        let mut s = BCAST_FILE_LOC.lock().unwrap();
        *s = cfg.get_tpv_bcast_file_loc().to_string();
    }
    let bind_addr = cfg.get_bind_addr();
    let socket = TcpListener::bind(bind_addr).unwrap();
    let routes = Routes::new(vec![
        get_focus(),
        get_focus_blocking(), 
        get_nearest(),
        get_nearest_blocking(),
        get_event(),
        get_event_blocking(),
        get_groups(),
        get_groups_blocking(),
        get_resultsindv(),
        get_resultsindv_blocking(),
        get_resultsteam(),
        get_resultsteam_blocking(),
    ]);
    let config = Config::new()
        .routes(routes)
        .mount_point(cfg.get_static_content_loc());
    let http = HttpListener::new(socket, config);

    info!("Starting http server");

    http.start();
}
