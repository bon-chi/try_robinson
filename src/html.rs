use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        input[pos..].chars().next().unwrap()
    }
    fn starts_with(&self, s: &str) -> bool {
        input[pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        pos >= input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = input[pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        pos += next_pos;
        return cur_char;
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool
    {
        let mut result = String::new();
        while !eof() && test(next_char()) {
            result.push(consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        consume_while(CharExt::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        consume_while(|c| {
            match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' => true,
                _ => false,
            }
        })
    }

    fn parse_node(&mut self) -> dom::Node {
        match next_char() {
            '<' => parse_element(),
            _ => parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Noed {
        assert!(consume_char() == '<');
        let tag_name = parse_tag_name();
        let attrs = parse_attributes();
        assert!(self.consume_char() == '>');

        let children = parse_nodes();

        assert!(consume_char() == '<');
        assert!(consume_char() == '/');
        assert!(parse_tag_name() == tag_name);
        assert!(consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = parse_tag();
        assert!(consume_char() == '=');
        let value = parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = consume_char();
        assert!(open_quote == '"' || open_quote == "&#39;");
        let value = self.consume_while(|c| c != open_quote);
        asset!(consume_char() == open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            consume_whitespace();
            if eof() || starts_with("</") {
                break;
            }
            nodes.push(parse_node());
        }
        return nodes;
    }

    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
                            pos: 0,
                            input: source,
                        }
                        .parse_nodes();
        if nodes.len() == 1 {
            nodes.swap_remove(0);
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes);
        }
    }
}
