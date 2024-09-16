use std::collections::HashMap;

use crate::dom::{AttrMap, ElementData, Node};
pub struct HTMLParser {
    pub html: String,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum HTMLToken {
    Text(String),
    OpenTag(TagMeta),
    SelfClose(TagMeta),
    CloseTag(String),
}

#[derive(Debug, PartialEq)]
pub struct TagMeta {
    name: String,
    attributes: AttrMap,
}

impl HTMLParser {
    pub fn new(html: String) -> Self {
        Self { html, pos: 0 }
    }

    fn next(&mut self) -> Option<char> {
        let next = self.html.chars().nth(self.pos);
        match next {
            Some(c) => {
                self.pos += 1;
                Some(c)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<char> {
        self.html.chars().nth(self.pos)
    }

    fn starts_with(&self, s: char) -> bool {
        let char = self.peek();
        match char {
            Some(c) => c == s,
            None => false,
        }
    }

    fn peek_at_n(&self, n: usize) -> Option<char> {
        self.html.chars().nth(self.pos + n)
    }

    fn peek_to_n(&self, n: usize) -> Option<String> {
        match self.peek_at_n(n) {
            Some(_) => {
                let mut result = String::new();
                for _ in 0..n {
                    result.push(self.peek().unwrap())
                }
                Some(result)
            }
            None => None,
        }
    }

    fn next_n(&mut self, n: usize) -> Option<String> {
        match self.peek_at_n(n - 1) {
            Some(_) => {
                let mut result = String::new();
                for _ in 0..n {
                    result.push(self.next().unwrap())
                }

                Some(result)
            }
            None => None,
        }
    }

    fn next_while_inc(&mut self, cond: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while self.peek().is_some() && cond(self.peek().unwrap()) {
            result.push(self.next().unwrap());
        }
        result.push(self.next().unwrap());
        result
    }

    fn next_while(&mut self, cond: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while self.peek().is_some() && cond(self.peek().unwrap()) {
            result.push(self.next().unwrap());
        }
        result
    }

    fn peek_while(&mut self, cond: impl Fn(char) -> bool) -> Option<String> {
        let mut result = String::new();
        let characters = self.html.chars().skip(self.pos);
        let mut char_count = 0;

        for char in characters {
            char_count += 1;
            if cond(char) {
                result.push(char);
            } else {
                break;
            }
        }

        //if result length is character count then condition was never met return None
        if result.len() == char_count {
            None
        } else {
            Some(result)
        }
    }

    fn next_text(&mut self) -> Option<HTMLToken> {
        let text = self.next_while(|c| !['<', '>'].contains(&c));
        if text.is_empty() {
            None
        } else {
            Some(HTMLToken::Text(text))
        }
    }

    fn next_tag(&mut self) -> Option<HTMLToken> {
        let closing_match = self.peek_while(|c| !matches!(c, '>'));
        match closing_match {
            Some(t) => {
                if t.starts_with("</") {
                    let tag = self.next_n(t.len() + 1).unwrap();
                    Some(HTMLToken::CloseTag(
                        tag.chars().filter(|c| c.is_alphanumeric()).collect(),
                    ))
                } else if t.starts_with("<!") {
                    //check if it's a comment or "directive"
                    if t.starts_with("<!--") {
                        //comment so skip completely
                        self.next_while_inc(|c| c != '>');
                        None
                    } else {
                        //directive
                        let name = "directive";
                        let mut attributes: HashMap<String, String> = HashMap::new();
                        //skip all the non alphanumeric characters
                        self.next_while(|c| !c.is_alphanumeric());
                        let text = self.next_while(|c| c != '>');
                        self.next(); //skip the >
                        attributes.insert("text".to_string(), text);
                        Some(HTMLToken::SelfClose(TagMeta {
                            name: name.to_string(),
                            attributes,
                        }))
                    }
                } else {
                    //remove the <
                    self.next();
                    //check for attributes
                    let name = self.next_while(|c| c.is_alphanumeric());
                    let mut attributes = HashMap::new();
                    while self.peek().unwrap() != '>' {
                        //remove leading whitespace
                        self.next_while(|c| c.is_whitespace());

                        //get attribute name
                        let attr_name = self.next_while(|c| c.is_alphanumeric());

                        let seperator = self.next_while(|c| !c.is_alphanumeric());
                        //check if attribute is quoted
                        if seperator.contains('"') {
                            let attr_value = self.next_while_inc(|c| c != '"');
                            attributes.insert(attr_name, attr_value);
                        } else {
                            let attr_value = self.next_while(|c| c.is_alphanumeric());
                            attributes.insert(attr_name, attr_value);
                        }

                        //skip all non alphanumeric characters
                        self.next_while(|c| !c.is_alphanumeric() && c != '>');
                    }
                    self.next(); //skip the >
                    Some(HTMLToken::OpenTag(TagMeta { name, attributes }))
                }
            }
            None => None,
        }
    }

    fn next_token(&mut self) -> Option<HTMLToken> {
        let next_char = self.peek();
        match next_char {
            Some(c) => {
                if c == '<' {
                    self.next_tag()
                } else {
                    match self.next_text() {
                        Some(HTMLToken::Text(text)) => {
                            if text.trim() == "" {
                                None
                            } else {
                                Some(HTMLToken::Text(text))
                            }
                        }
                        _ => None,
                    }
                }
            }
            None => None,
        }
    }

    fn create_dom_tree(&self, tokens: Vec<HTMLToken>) -> Vec<Node> {
        let mut stack: Vec<Node> = vec![];
        let mut traversed: Vec<Node> = vec![];
        for token in tokens {
            // println!("token - {:#?}", token);
            // println!("stack - {:#?}", stack);
            match token {
                HTMLToken::OpenTag(tag) => {
                    let element = Node::element(ElementData {
                        tag_name: tag.name,
                        attributes: tag.attributes,
                        children: vec![],
                    });

                    stack.push(element)
                }

                HTMLToken::CloseTag(t) => {
                    let node = stack.pop().unwrap();
                    let parent = stack.last_mut();
                    match parent {
                        Some(parent) => {
                            if let Node::Element(parent_element) = parent {
                                parent_element.children.push(node);
                            }
                        }

                        None => {
                            traversed.push(node);
                        }
                    }
                }

                HTMLToken::SelfClose(tag) => {
                    if !tag.name.is_empty() {
                        let element = Node::element(ElementData {
                            tag_name: tag.name,
                            attributes: tag.attributes,
                            children: vec![],
                        });

                        let parent = stack.last_mut();
                        match parent {
                            Some(parent) => {
                                if let Node::Element(parent_element) = parent {
                                    parent_element.children.push(element);
                                }
                            }

                            None => {
                                traversed.push(element);
                            }
                        }
                    }
                }

                HTMLToken::Text(text) => {
                    let text_node = Node::text(text);
                    let parent = stack.last_mut();
                    match parent {
                        Some(parent) => {
                            if let Node::Element(parent_element) = parent {
                                parent_element.children.push(text_node);
                            }
                        }

                        None => {
                            traversed.push(text_node);
                        }
                    }
                }
            }
        }
        traversed
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut tokens = vec![];
        while self.peek().is_some() {
            let token = self.next_token();
            // println!("token - {:#?}", token);
            if let Some(n) = token {
                tokens.push(n);
            }
        }

        //clean nodes
        //convert  unclosed tags to self close
        let mut token_stack: Vec<(usize, &str)> = vec![];
        let mut unclosed_tokens: Vec<usize> = vec![];
        let mut invalid_tokens: Vec<usize> = vec![];
        for (index, token) in tokens.iter().enumerate() {
            // println!("token - {:#?}", token);
            match token {
                HTMLToken::OpenTag(tag) => {
                    if tag.name.is_empty() {
                        invalid_tokens.push(index);
                    } else {
                        token_stack.push((index, &tag.name));
                    }
                }
                HTMLToken::CloseTag(tag) => {
                    if !token_stack.is_empty() {
                        if token_stack.last().unwrap().1 == tag {
                            token_stack.pop();
                        } else {
                            let mut peek_index = token_stack.len() - 1;

                            while token_stack.get(peek_index).unwrap().1 != tag {
                                if peek_index == 0 {
                                    break;
                                } else {
                                    let (index, item) = token_stack.get(peek_index).unwrap();
                                    peek_index -= 1;
                                }
                            }
                            if token_stack.get(peek_index).unwrap().1 == tag {
                                // //remove all the items up to peek_index
                                let current_unclosed: Vec<usize> = token_stack
                                    .split_off(peek_index + 1)
                                    .iter()
                                    .map(|(index, _)| *index)
                                    .collect();
                                unclosed_tokens.extend(current_unclosed);

                                //pop tag from stack
                                token_stack.pop();
                            } else {
                                //no matching open tag found, add to invalid tokens
                                invalid_tokens.push(index);
                            }
                        }
                    } else {
                        //stack is empty add to invalid tokens
                        invalid_tokens.push(index);
                    }
                }
                _ => {}
            }
        }

        //convert unclosed tags
        for index in unclosed_tokens.iter() {
            let item = tokens.get_mut(*index).unwrap();
            if let HTMLToken::OpenTag(tag) = item {
                *item = HTMLToken::SelfClose(TagMeta {
                    name: tag.name.clone(),
                    attributes: tag.attributes.clone(),
                });
            }
        }

        //remove invalid tokens
        let valid_tokens: Vec<HTMLToken> = tokens
            .into_iter()
            .enumerate()
            .filter(|(index, _)| !invalid_tokens.contains(index))
            .map(|(_, token)| token)
            .collect();

        self.create_dom_tree(valid_tokens)
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_tag() {
        let html = "<a class=\"id\" href=\"https://abundance.com\"></a>";
        let mut parser = HTMLParser::new(html.to_string());
        let tag = parser.next_token();
    }

    #[test]
    fn create_tree() {
        let html = "<HEADER>
  <TITLE>The World Wide Web project</TITLE>
  <NEXTID N=\"55\">
</HEADER>
            </yam>
";
        let mut parser = HTMLParser::new(html.to_string());
        let tree = parser.parse();

        println!("tree - {:?}", tree);

        // let tree = parser.create_dom_tree(nodes);
    }
}
