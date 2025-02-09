extern crate simple_server;

use unicode_bom::Bom;
use std::{fs, io, sync::{Arc, Mutex}, thread};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
use simple_server::{Method, Server, StatusCode};

pub struct CacheableJson {
    data: Arc<Mutex<String>>,
}

impl CacheableJson {
    pub fn new() -> CacheableJson {
        CacheableJson { 
            data: Arc::new(Mutex::new(String::from("[]"))), 
        }
    }
}

pub struct Cache {
    focus       : CacheableJson,
    nearest     : CacheableJson,
    entries     : CacheableJson,
    event       : CacheableJson,
    groups      : CacheableJson,
    results_indv: CacheableJson,
    results_team: CacheableJson, 
}

impl Cache {
    pub fn new() -> Cache {
        Cache { 
            focus       : CacheableJson::new(),
            nearest     : CacheableJson::new(),
            entries     : CacheableJson::new(),
            event       : CacheableJson::new(),
            groups      : CacheableJson::new(),
            results_indv: CacheableJson::new(),
            results_team: CacheableJson::new(),         
        }
    }

    pub fn focus_data(&self) -> Arc<Mutex<String>> {
        self.focus.data.clone()
    }

    pub fn nearest_data(&self) -> Arc<Mutex<String>> {
        self.nearest.data.clone()
    }

    pub fn entries_data(&self) -> Arc<Mutex<String>> {
        self.entries.data.clone()
    }

    pub fn event_data(&self) -> Arc<Mutex<String>> {
        self.event.data.clone()
    }

    pub fn groups_data(&self) -> Arc<Mutex<String>> {
        self.groups.data.clone()
    }

    pub fn results_indv_data(&self) -> Arc<Mutex<String>> {
        self.results_indv.data.clone()
    }

    pub fn results_team_data(&self) -> Arc<Mutex<String>> {
        self.results_team.data.clone()
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
        // access to cache data
        let focus = self.cache.focus_data();
        let nearest = self.cache.nearest_data();
        let entries = self.cache.entries_data();
        let event = self.cache.event_data();
        let groups = self.cache.groups_data();
        let results_indv = self.cache.results_indv_data();
        let results_team = self.cache.results_team_data();

        thread::spawn(move || {
            log::info!("Cache started on {}", path);

            let (tx, rx) = mpsc::channel::<Result<Event>>();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
            watcher.watch(Path::new(&path), RecursiveMode::NonRecursive).unwrap();

            for res in rx {
                match res {
                    Ok(e) => {
                        if e.kind.is_access() {
                            log::debug!("event: {:?}", e);
                            for p in e.paths {
                                match Instance::read_from_fs(p.to_str().unwrap()) {
                                    Ok(content) => {
                                        if p.ends_with("focus.json") {
                                            let mut focus_locked = focus.lock().unwrap();
                                            *focus_locked = content;
                                            // notify change listeners
                                            log::info!("Updated cache for focus data");
                                        } else if p.ends_with("nearest.json") {
                                            let mut nearest_locked = nearest.lock().unwrap();
                                            *nearest_locked = content;
                                            log::info!("Updated cache for nearest data");
                                        } else if p.ends_with("entries.json") {
                                            let mut entries_locked = entries.lock().unwrap();
                                            *entries_locked = content;
                                            log::info!("Updated cache for entries data");
                                        } else if p.ends_with("event.json") {
                                            let mut event_locked = event.lock().unwrap();
                                            *event_locked = content;
                                            log::info!("Updated cache for event data");
                                        } else if p.ends_with("groups.json") {
                                            let mut groups_locked = groups.lock().unwrap();
                                            *groups_locked = content;
                                            log::info!("Updated cache for groups data");
                                        } else if p.ends_with("resultsIndv.json") {
                                            let mut results_indv_locked = results_indv.lock().unwrap();
                                            *results_indv_locked = content;
                                            log::info!("Updated cache for results_indv data");
                                        } else if p.ends_with("resultsTeam.json") {
                                            let mut results_team_locked = results_team.lock().unwrap();
                                            *results_team_locked = content;
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
        
        // const CACHE_WAIT_TIMEOUT: u64 = 5000;

        // access to cache data
        let focus = self.cache.focus_data();
        let nearest = self.cache.nearest_data();
        let entries = self.cache.entries_data();
        let event = self.cache.event_data();
        let groups = self.cache.groups_data();
        let results_indv = self.cache.results_indv_data();
        let results_team = self.cache.results_team_data();

        // access to msg. bus to broadcast cache changes 
        // let focus_bus = self.cache.focus_bus();
        // let nearest_bus = self.cache.nearest_bus();
        // let entries_bus = self.cache.entries_bus();
        // let event_bus = self.cache.event_bus();
        // let groups_bus = self.cache.groups_bus();
        // let results_indv_bus = self.cache.results_indv_bus();
        // let results_team_bus = self.cache.results_team_bus();

        let server = Server::new( move |request, mut response| {
            log::info!("Received: {} {}", request.method(), request.uri());

            if request.method() == &Method::GET && request.uri().path().starts_with("/bcast/") {
                let uri= &request.uri().path()[7..];

                if uri == "focus" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = focus_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let focus_locked = focus.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(focus_locked.as_bytes().to_vec())?)
                } else if uri == "nearest" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = nearest_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let nearest_locked = nearest.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(nearest_locked.as_bytes().to_vec())?)
                } else if uri == "entries" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = entries_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let entries_locked = entries.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(entries_locked.as_bytes().to_vec())?)
                } else if uri == "event" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = event_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let event_locked = event.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(event_locked.as_bytes().to_vec())?)
                } else if uri == "groups" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = groups_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let groups_locked = groups.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(groups_locked.as_bytes().to_vec())?)
                } else if uri == "resultsIndv" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = results_indv_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let results_indv_locked = results_indv.lock().unwrap();
                    response.header("content-type", "text/json");
                    Ok(response.body(results_indv_locked.as_bytes().to_vec())?)
                }  else if uri == "resultsTeam" {
                    // wait for cache update data
                    // let mut rx: bus::BusReader<bool>;
                    // { 
                    //     let mut b = results_team_bus.lock().unwrap();
                    //     rx = b.add_rx();
                    // }
                    // let _ = rx.recv_timeout(Duration::from_millis(CACHE_WAIT_TIMEOUT));
                    let results_team_locked = results_team.lock().unwrap();
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