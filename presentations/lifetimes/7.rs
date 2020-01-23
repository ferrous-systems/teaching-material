use std::str::Split;

struct Tokenizer<'input> {
    input: Split<'input, char>,
}

impl<'input> Tokenizer<'input> {
    fn next_token(&mut self) -> Option<&'input str> {
        self.input.next()
    }
}

struct Parser<'tokenizer, 'input: 'tokenizer> {
    tokenizer: &'tokenizer mut Tokenizer<'input>,
}

impl<'tokenizer, 'input: 'tokenizer> Parser<'tokenizer, 'input> {
    fn next_item(&mut self) -> Option<&'input str> {
        self.tokenizer.next_token()
    }
}

fn main() {
    let mut tok = Tokenizer { input: "( foo bar )".split(' ') };
    let mut parser = Parser { tokenizer: &mut tok };

    println!("{:?}", parser.next_item());
    let content = parser.next_item();
    let content2 = parser.next_item();
    println!("{:?}", parser.next_item());
    drop(parser);

    println!("{:?}", content);
    println!("{:?}", content2);

}