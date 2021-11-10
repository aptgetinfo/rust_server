use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr:String) -> Self {
        Self { addr }
    }
    pub fn run(self){
        println!("Listening on {}",self.addr);
        let listner=TcpListener::bind(&self.addr).unwrap();
        loop {
            match listner.accept() {
                OK((stream,_)) =>{
                    println!("OK");
                }
                Err(e) => println("Failed to establish a connection {}",e);
            }
        }
    }
}