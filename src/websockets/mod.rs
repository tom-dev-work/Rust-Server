use tungstenite::{connect, Message as tungMessage};
use url::Url;

#[path = "../config/mod.rs"]
mod config;
use config::Config;

pub fn send_message(msg: String) {
    let configuration: Config = config::load_config();
    let hostname = format!(
        "wss://{}:{}",
        configuration.ip, configuration.websocket_port
    );
    match Url::parse(&hostname) {
        Ok(url) => {
            match connect(url) {
                Ok(mut socket) => {
                    match socket.0.write_message(tungMessage::Text(msg).into()) {
                        Ok(_) => (),
                        Err(_) => println!("Server not alive..")
                    }
                },
                Err(_) => ()
            }
           
        }
        Err(_) => ()
    }
   
    
}

/*pub fn listen_for_messages() {
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}*/
