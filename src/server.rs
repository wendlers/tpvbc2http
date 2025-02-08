extern crate simple_server;

use unicode_bom::Bom;
use std::{fs, io, sync::{Arc, Mutex}, thread};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
use simple_server::{Method, Server, StatusCode};

pub struct Cache {
    focus: Arc<Mutex<String>>,
    nearest: Arc<Mutex<String>>,
    entries: Arc<Mutex<String>>,
    event: Arc<Mutex<String>>,
    groups: Arc<Mutex<String>>,
    results_indv: Arc<Mutex<String>>,
    results_team: Arc<Mutex<String>>, 
}

impl Cache {
    pub fn new() -> Cache {
        Cache { 
            focus: Arc::new(Mutex::new(String::from("[]"))),
            nearest: Arc::new(Mutex::new(String::from("[]"))),
            entries: Arc::new(Mutex::new(String::from("[]"))),
            event: Arc::new(Mutex::new(String::from("[]"))),
            groups: Arc::new(Mutex::new(String::from("[]"))),
            results_indv: Arc::new(Mutex::new(String::from("[]"))),
            results_team: Arc::new(Mutex::new(String::from("[]"))),         
        }
    }
}

pub struct Instance {
    cache: Cache,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            cache: Cache::new(),
        }
    }

    fn read_from_fs(fname: &str) -> io::Result<String> {
        match fs::read_to_string(fname) {
            Ok(conten) => {
                // if there is a leading BOM, remove it ...
                let bom = Bom::from(conten.as_bytes());
                Ok(conten[bom.len()..].to_string())
            },
            Err(err) => Err(err),
        }
    }

    fn start_cache(&self, path: String) {
        let focus_cache_clone = self.cache.focus.clone();
        let nearest_cache_clone = self.cache.nearest.clone();
        let entries_cache_clone = self.cache.entries.clone();
        let event_cache_clone = self.cache.event.clone();
        let groups_cache_clone = self.cache.groups.clone();
        let results_indv_cache_clone = self.cache.results_indv.clone();
        let results_team_cache_clone = self.cache.results_team.clone();

        thread::spawn(move || {
            log::info!("Cache started on {}", path);

            let (tx, rx) = mpsc::channel::<Result<Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
            watcher.watch(Path::new(&path), RecursiveMode::NonRecursive).unwrap();

            for res in rx {
                match res {
                    Ok(event) => {
                        if event.kind.is_access() {
                            log::debug!("event: {:?}", event);
                            for p in event.paths {
                                match Instance::read_from_fs(p.to_str().unwrap()) {
                                    Ok(content) => {
                                        if p.ends_with("focus.json") {
                                            let mut focus_cache_locked = focus_cache_clone.lock().unwrap();
                                            *focus_cache_locked = content;
                                            log::info!("Updated cache for focus data");
                                        } else if p.ends_with("nearest.json") {
                                            let mut nearest_cache_locked = nearest_cache_clone.lock().unwrap();
                                            *nearest_cache_locked = content;
                                            log::info!("Updated cache for nearest data");
                                        } else if p.ends_with("entries.json") {
                                            let mut entries_cache_locked = entries_cache_clone.lock().unwrap();
                                            *entries_cache_locked = content;
                                            log::info!("Updated cache for entries data");
                                        } else if p.ends_with("event.json") {
                                            let mut event_cache_locked = event_cache_clone.lock().unwrap();
                                            *event_cache_locked = content;
                                            log::info!("Updated cache for event data");
                                        } else if p.ends_with("groups.json") {
                                            let mut groups_cache_locked = groups_cache_clone.lock().unwrap();
                                            *groups_cache_locked = content;
                                            log::info!("Updated cache for groups data");
                                        } else if p.ends_with("resultsIndv.json") {
                                            let mut results_indv_cache_locked = results_indv_cache_clone.lock().unwrap();
                                            *results_indv_cache_locked = content;
                                            log::info!("Updated cache for results_indv data");
                                        } else if p.ends_with("resultsTeam.json") {
                                            let mut results_team_cache_locked = results_team_cache_clone.lock().unwrap();
                                            *results_team_cache_locked = content;
                                            log::info!("Updated cache for results_team data");
                                        }
                                    },
                                    Err(err) => log::warn!("{:?}", err),
                                }
                            }
                        }
                    },
                    Err(e) => log::warn!("watch error: {:?}", e),
                }
            }
        });
    }

    pub fn start(&self, host: &str, port: &str, path: String) {
        self.start_cache(path.clone());
        
        let focus_clone = self.cache.focus.clone();
        let nearest_clone = self.cache.nearest.clone();
        let entries_clone = self.cache.entries.clone();
        let event_clone = self.cache.event.clone();
        let groups_clone = self.cache.groups.clone();
        let results_indv_clone = self.cache.results_indv.clone();
        let results_team_clone = self.cache.results_team.clone();

        let server = Server::new(move |request, mut response| {
            log::info!("Received: {} {}", request.method(), request.uri());
    
            if request.method() == &Method::GET && request.uri().path().starts_with("/bcast/") {
                let uri= &request.uri().path()[7..];

                if uri == "focus" {
                    let focus_locked = focus_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(focus_locked.as_bytes().to_vec())?)
                } else if uri == "nearest" {
                    let nearest_locked = nearest_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(nearest_locked.as_bytes().to_vec())?)
                } else if uri == "entries" {
                    let entries_locked = entries_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(entries_locked.as_bytes().to_vec())?)
                } else if uri == "event" {
                    let event_locked = event_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(event_locked.as_bytes().to_vec())?)
                } else if uri == "groups" {
                    let groups_locked = groups_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(groups_locked.as_bytes().to_vec())?)
                } else if uri == "resultsIndv" {
                    let results_indv_locked = results_indv_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(results_indv_locked.as_bytes().to_vec())?)
                }  else if uri == "resultsTeam" {
                    let results_team_locked = results_team_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(results_team_locked.as_bytes().to_vec())?)
                } else {
                    response.status(StatusCode::NOT_FOUND);
                    Ok(response.body("<h1>404</h1><p>File not found!<p>".as_bytes().to_vec())?)                       
                }
            } else {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("<h1>404</h1><p>Page not found!<p>".as_bytes().to_vec())?)                
            }
        });   
        server.listen(host, port);
    }
}