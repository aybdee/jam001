use std::env;
use std::fs;

use parser::HTMLParser;

mod dom;
mod network;
mod parser;
fn main() {
    let mut browser_client = network::BrowserClient::new();
    let response = browser_client.get("http://motherfuckingwebsite.com/");
    match response {
        Ok(body) => {
            // println!("{}", body);
            let dom_tree = HTMLParser::new(body).parse();
            println!("{:#?}", dom_tree)
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
