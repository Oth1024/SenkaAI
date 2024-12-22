use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Listener
{
    listener: TcpListener,
    construction_time: u128,
}

impl Listener
{
    pub fn build_listener(port: i32) -> Listener
    {
        let listener = TcpListener:: bind(format!("{}:{}", "127.0.0.1", port)).unwrap();
        return Listener
        {
            listener: listener,
            construction_time: SystemTime:: now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        };
    } 
}