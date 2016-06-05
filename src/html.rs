struct Parser {
    pos: usize,
    input: String,
}
//
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
}
