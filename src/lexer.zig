//- Imports --------------------------------------------------------------------
const std = @import("std");

//- Aliases --------------------------------------------------------------------
const Allocator = std.mem.Allocator;
const stderr = std.io.getStdErr().writer();

//- Definitions ----------------------------------------------------------------
pub const TokenType = enum {
    left_bracket,
    right_bracket,
    symbol,
    integer,
    eof,
};

pub const Token = struct {
    is: TokenType,
    lexeme: []u8,
};

pub const Lexer = struct {
    const Self = @This();
    source: []u8,
    start: usize = 0,
    end: usize = 0,

    pub fn init(source: []u8) Lexer {
        return Lexer{ .source = source };
    }

    pub fn isAtEnd(self: *Self) bool {
        return self.end == self.source.len;
    }

    pub fn advance(self: *Self) u8 {
        self.end += 1;
        return self.source[self.end - 1];
    }

    pub fn peek(self: *Self) u8 {
        return self.source[self.end];
    }

    pub fn makeToken(self: *Self, is: TokenType) Token {
        return Token.init(is, self.source[self.start..self.end]);
    }
};
