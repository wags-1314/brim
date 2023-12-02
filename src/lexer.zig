//- Imports --------------------------------------------------------------------
const std = @import("std");
const ascii = std.ascii;

//- Aliases --------------------------------------------------------------------
const Allocator = std.mem.Allocator;
const stderr = std.io.getStdErr().writer();

//- Definitions ----------------------------------------------------------------
pub const TokenType = enum {
    left_bracket,
    right_bracket,
    symbol,
    integer,
    err,
};

pub const Token = struct {
    is: TokenType,
    lexeme: []const u8,

    const Self = @This();

    pub fn print(self: Self) void {
        std.debug.print("Token[", .{});
        switch (self.is) {
            .left_bracket => std.debug.print("(]\n", .{}),
            .right_bracket => std.debug.print(")]\n", .{}),
            .symbol => std.debug.print("sym, \"{s}\"]\n", .{self.lexeme}),
            .integer => std.debug.print("int, \"{s}\"]\n", .{self.lexeme}),
            .err => std.debug.print("err]\n", .{}),
        }
    }
};

fn isDelimiter(char: u8) bool {
    return ascii.isWhitespace(char) or char == 0 or char == '(' or char == ')';
}

pub const Scanner = struct {
    const Self = @This();
    source: []const u8,
    start: usize = 0,
    end: usize = 0,

    pub fn init(source: []const u8) Scanner {
        return Scanner{ .source = source };
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
        return Token{ .is = is, .lexeme = self.source[self.start..self.end] };
    }

    pub fn skipWhitespace(self: *Self) void {
        while (!self.isAtEnd()) {
            var ch = self.peek();
            if (ascii.isWhitespace(ch)) {
                _ = self.advance();
            } else {
                return;
            }
        }
    }

    pub fn next(self: *Self) ?Token {
        self.skipWhitespace();
        self.start = self.end;

        if (self.isAtEnd()) {
            return null;
        }

        var char = self.advance();

        if (ascii.isDigit(char) or (char == '-' and ascii.isDigit(self.peek()))) {
            return self.scanNumber();
        }

        switch (char) {
            '(' => return self.makeToken(TokenType.left_bracket),
            ')' => return self.makeToken(TokenType.right_bracket),
            else => return self.scanSymbol(),
        }

        return self.makeToken(TokenType.err);
    }

    pub fn scanNumber(self: *Self) Token {
        while (!self.isAtEnd() and ascii.isDigit(self.peek())) {
            _ = self.advance();
        }

        return self.makeToken(TokenType.integer);
    }

    pub fn scanSymbol(self: *Self) Token {
        while (!self.isAtEnd() and !isDelimiter(self.peek())) {
            _ = self.advance();
        }

        return self.makeToken(TokenType.symbol);
    }
};

test "scan number" {
    var scanner = Scanner.init("0123456789");
    try std.testing.expectEqual(scanner.next().?.is, TokenType.integer);
}

test "scan ()" {
    var scanner = Scanner.init("()");
    try std.testing.expectEqual(scanner.next().?.is, TokenType.left_bracket);
    try std.testing.expectEqual(scanner.next().?.is, TokenType.right_bracket);
}

test "scan symbol" {
    var scanner = Scanner.init("gfjhfjf$@#@$");
    try std.testing.expectEqual(scanner.next().?.is, TokenType.symbol);
}

test "scan all" {
    var scanner = Scanner.init("1234 asdasd ()");
    try std.testing.expectEqual(scanner.next().?.is, TokenType.integer);
    try std.testing.expectEqual(scanner.next().?.is, TokenType.symbol);
    try std.testing.expectEqual(scanner.next().?.is, TokenType.left_bracket);
    try std.testing.expectEqual(scanner.next().?.is, TokenType.right_bracket);
}
