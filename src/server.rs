pub struct Server {
    addr: String,
}

impl Server {
    fn new(addr:String) -> Self {
        Self { addr }
    }
    fn run(self){
        println!("Listening on {}",self.addr);
    }
}