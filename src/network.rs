use ureq::{Agent, AgentBuilder};

pub struct BrowserClient {
    agent: Agent,
    pub page_stack: Vec<String>,
    pub current_page_index: usize,
}

impl BrowserClient {
    pub fn new() -> Self {
        BrowserClient {
            agent: AgentBuilder::new().build(),
            page_stack: Vec::new(),
            current_page_index: 0,
        }
    }

    pub fn get(&mut self, url: &str) -> Result<String, Box<ureq::Error>> {
        let response = self.agent.get(url).call()?;
        let body = response.into_string().unwrap();
        self.page_stack.push(url.to_string());
        self.current_page_index = self.page_stack.len() - 1;
        Ok(body)
    }
}

// println!("tag - {:?}", tag);
// let mut peek_index = token_stack.len() - 1;
// while token_stack.get(peek_index).unwrap().1 != tag {
//     if peek_index == 0 {
//         break;
//     } else {
//         println!("peek_index - {:?}", peek_index);
//         unclosed.push(index);
//         println!("unclosed - {:?}", unclosed);
//         peek_index -= 1;
//     }
// }
// if peek_index != 0 {
//     //remove all the items up to peek_index
//     token_stack.truncate(peek_index)
// }
