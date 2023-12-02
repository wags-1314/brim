pub const Lexer = @import("lexer.zig");

test {
    @import("std").testing.refAllDecls(@This());
}
