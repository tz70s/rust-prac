// Write some testing

#[derive(Debug)]
pub struct Server<'a> {
    name: &'a str,
    ip_addr: &'a str,
    port: u32,
}

impl<'a> Server<'a> {
    /// Create a new Server
    fn new(name: &'a str, ip_addr: &'a str, port: u32) -> Server<'a> {        
        Server {
            name: name,
            ip_addr: ip_addr,
            port: port,
        }
    }
    /// Dump Server informations
    fn dump_server_info(&self) {
        println!("{:#?}", self);
    }

    fn valid_ipv4(&self) {
        let classes = self.ip_addr.split(".");
        // collect in vector
        let classes: Vec<&str> = classes.collect();
        if classes.len() < 4 {
            panic!("Invalid ip address, it should be four classes in ipv4");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // test if the port 80
    #[test]
    fn port_assign() {
        let sv = Server::new("Server1", "127.0.0.1", 8080);
        assert_eq!(8080, sv.port);
    }

    #[test]
    fn valid_ip() {
        let sv = Server::new("Server1", "127.0.0.1", 8080);
        sv.valid_ipv4();
    }

    #[test]
    #[should_panic(expect = "The invalid input of ip address should be paniked")]
    fn invalid_ip() {
        let sv = Server::new("Server1", "127.0.0", 8080);
        sv.valid_ipv4();
    }
}
