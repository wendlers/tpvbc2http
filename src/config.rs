pub(crate) struct Server {
    bind_addr: String,
    tpv_bcast_file_loc: String,
    static_content_loc: String,
}

impl Server {
    pub(crate) fn new() -> Server {
        Server {
            bind_addr: "0.0.0.0:8080".to_string(),
            tpv_bcast_file_loc: "/home/stefan/shared/".to_string(),
            // tpv_bcast_file_loc: "./".to_string(),
            static_content_loc: "/home/stefan/devel/tpvbc2http/http".to_string(),
            // static_content_loc: "C:/Users/stefan/Documents/TPVirtual/Broadcast/http/".to_string(),
        }
    }

    pub(crate) fn get_bind_addr(&self) -> &str {
        &self.bind_addr
    }

    pub(crate) fn set_bind_addr(&mut self, addr: String) {
        self.bind_addr = addr;
    }

    pub(crate) fn get_tpv_bcast_file_loc(&self) -> &str {
        &self.tpv_bcast_file_loc
    }

    pub(crate) fn set_tpv_bcast_file_loc(&mut self, loc: String) {
        self.tpv_bcast_file_loc = loc;
    }

    pub(crate) fn get_static_content_loc(&self) -> &str {
        &self.static_content_loc
    }

    pub(crate) fn set_static_content_loc(&mut self, loc: String) {
        self.static_content_loc = loc;
    }
}