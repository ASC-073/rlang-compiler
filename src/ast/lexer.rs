// To Learn from here:
// TextSpan struct has impl?, kind, span
// Are structs more like tuples in Rust? They can be used like that?
// #derive[Debug]

// usize: unsigned integer of size 32 bit or 64.
// Self - type of the thing for which u writing impl, self is the actual "self" from OOP
// Literal: Constant expression so evaluated at compile time, reduces runtime complexity.
// Option: Either some or none
// Peekable: iterator with peek() which returns Optional reference to next element. Here: string, iterated through chars

// Token types: number, identifier etc.
#[derive(Debug)]
pub enum TokenKind{
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Whitespace,
    Eof,
    Bad
}

// TextSpan: has some start and end, and represents span of the original text
// Spans of parts of code
#[derive(Debug)]
pub struct TextSpan{
    start:usize,
    end:usize,
    // Duplicate text here? BC we also have input text. More convenient this way tho
    literal:String,
    
}
impl TextSpan{
    pub fn new(start:usize, end:usize, literal:String) -> Self{
        Self {start,end,literal}
    }
    pub fn length(&self) -> usize{
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token{
    kind:TokenKind,
    span:TextSpan
}

impl Token{
    pub fn new(kind: TokenKind, span: TextSpan) -> Self{
        Self {kind, span}       
    }
}

pub struct Lexer <'a>{
    // Like an iterator of String for each character? (Peekable)
    // Instead took mutable reference 
    input: &'a str,
    current_pos: usize
}

impl <'a> Lexer <'a>{
    // Constructor
    pub fn new(input: &'a str) -> Self{
        Self {input, current_pos: 0}
    }
    
    pub fn next_token(&mut self) -> Option<Token>{
        // End of file obtained.
        if self.current_pos == self.input.len(){
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind :: Eof, 
                TextSpan :: new(0, 0, '\0'.to_string())
            ));
        }
        let c = self.current_char();
        // Explain map, and why we did it
        return c.map(|c|{
            // IF we get number: 3, then if it is immediately followed by another numerical character then they are part of same number
            // Else nah
            let start = self.current_pos;
            // Bad characters not relevant to our lexer, like #
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c){
                let number:i64 = self.consume_number();
                kind = TokenKind::Number(number);
            }
            else if Self::is_whitespace(&c){
                self.consume();
                kind = TokenKind::Whitespace;
            }
            else{
                kind = self.consume_punctuation();
            }
            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }
    
    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad
        }
    }
    
    fn is_number_start(c:&char) -> bool{
        c.is_digit(10) // radix 10 because decimal system assumed
    }
    
    fn is_whitespace(c:&char) -> bool{
        c.is_whitespace()
    }
    
    // Explain unwrap
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }
    
    // Just moves pointer to next character
    // Returns option because could reach end of file which would return None
    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len(){
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        c
    }
    
    fn consume_number(&mut self) -> i64 {
        let mut number = 0;
        while let Some(c) = self.current_char(){
            // More efficient than putting in a buffer then passing
            if c.is_digit(10){
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            }
            else{
                break;
            }
        }
        number
    }
}