use std::env;
use std::fs;

use parser::HTMLParser;

mod dom;
mod network;
mod parser;
fn main() {
    // let contents =
    //     fs::read_to_string("./file.html").expect("Should have been able to read the file");
    //
    // let mut parser = parser::HTMLParser::new(contents);
    // let tree = parser.parse();
    // println!("{:#?}", tree);

    let mut browser_client = network::BrowserClient::new();
    let response = browser_client.get("https://info.cern.ch/hypertext/WWW/TheProject.html");
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
