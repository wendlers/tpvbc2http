extern crate simple_server;

use std::{fs, io, sync::{Arc, Mutex}, thread};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
use simple_server::{Method, Server, StatusCode};

pub struct Instance {
    focus_cache: Arc<Mutex<String>>,
    nearest_cache: Arc<Mutex<String>>,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            focus_cache: Arc::new(Mutex::new(String::from("   []"))),
            nearest_cache: Arc::new(Mutex::new(String::from("   []"))),
        }
    }

    fn read_from_fs(fname: &str) -> io::Result<String> {
        fs::read_to_string(fname)
    }

    fn start_cache(&self, path: String) {
        let focus_cache_clone = self.focus_cache.clone();
        let nearest_cache_clone = self.nearest_cache.clone();

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
        
        let focus_cache_clone = self.focus_cache.clone();
        let nearest_cache_clone = self.nearest_cache.clone();

        let server = Server::new(move |request, mut response| {
            log::info!("Received: {} {}", request.method(), request.uri());
    
            if request.method() == &Method::GET && request.uri().path().starts_with("/bcast/") {
                let uri= &request.uri().path()[7..];

                if uri == "focus" {
                    let focus_cache_locked = focus_cache_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(focus_cache_locked.as_bytes().to_vec())?)
                } else if uri == "nearest" {
                    let nearest_cache_locked = nearest_cache_clone.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(nearest_cache_locked.as_bytes().to_vec())?)
                } else if uri == "entries" || uri == "event" || uri == "groups" || uri == "resultsIndv" || uri == "resultsTeam"{
                    response.header("content-type", "text/json");
                    Ok(response.body("   []".as_bytes().to_vec())?)
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