use std::iter::Peekable;
use std::str::Chars;
use std::cmp::min;

#[derive(Debug)]
pub enum Token {
    LParen(usize, usize, usize),
    RParen(usize, usize, usize),
    Symbol(usize, usize, usize),
    Integer(usize, usize, usize),
    Error(usize, usize, usize),
}

pub struct Lexer<'a> {
    muncher: Peekable<Chars<'a>>,
    start: usize,
    end: usize,
    line: usize,
    len: usize,
    has_errored: bool,
}

fn is_symbol(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '!' || ch == '@' || ch == '#' ||
	ch == '$' || ch == '%' || ch == '^' || ch == '&' || ch == '*' ||
	ch == '<' || ch == '>' || ch == '`' || ch == '~' || ch == '/' ||
	ch == '?' || ch == '-' || ch == '_' || ch == '+' || ch == '|' ||
	ch == ':' || ch == ';'
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
	Lexer {
	    muncher: source.chars().peekable(),
	    start: 0,
	    end: 0,
	    line: 0,
	    len: source.chars().count(),
	    has_errored: false,
	}
    }

    pub fn reset(&mut self, source: &'a str) {
	self.muncher = source.chars().peekable();
	self.start = 0;
	self.end = 0;
	self.line = 0;
	self.len = source.chars().count();
    }

    fn skip_whitespace(&mut self) {
	loop {
	    match self.muncher.peek() {
		Some(' ' | '\t') => {
		    let _ = self.advance();
		},
		Some('\n') => {
		    self.line += 1;
		    let _ = self.advance();
		},
		Some(_) | None => break,
	    }
	}
    }

    fn advance(&mut self) -> Option<char> {
	self.end = min(self.end + 1, self.len);
	self.muncher.next()
    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
	if self.has_errored {
	    return None;
	}
	self.skip_whitespace();
	self.start = self.end;

	let ch = self.advance();

	if let Some('(') = ch {
	    Some(Token::LParen(self.start,
			       self.end,
			       self.line))
	} else if let Some(')') = ch {
	    Some(Token::RParen(self.start,
			       self.end,
			       self.line))
	} else if let Some(_) = ch {
	    let ch = ch?;
	    if ch.is_ascii_digit() {
		loop {
		    match self.muncher.peek() {
			Some(ch) if ch.is_ascii_digit() => {
			    let _ = self.advance();
			},
			_ => { break; }
		    };
		}
		Some(Token::Integer(self.start,
				    self.end,
				    self.line))
	    } else if is_symbol(ch) {
		loop {
		    match self.muncher.peek() {
			Some(ch) if is_symbol(*ch) => {
			    let _ = self.advance();
			}
			_ => { break; }
		    };
		}
		Some(Token::Symbol(self.start,
				   self.end,
				   self.line))
	    } else {
		self.has_errored = true;
		Some(Token::Error(self.start,
				  self.start + 1,
				  self.line))
	    }
	} else {
	    None
	}
    }
}
