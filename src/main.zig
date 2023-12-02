//- Imports --------------------------------------------------------------------
const std = @import("std");

const Scanner = @import("lexer.zig").Scanner;

//- Definitions & Aliases ------------------------------------------------------
const Allocator = std.mem.Allocator;
const stderr = std.io.getStdErr().writer();

pub fn readFile(path: []const u8, allocator: std.mem.Allocator) ![]const u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    const file_size = (try file.stat()).size;
    return try file.reader().readAllAlloc(allocator, file_size);
}

pub fn runFromFile(path: []u8, allocator: Allocator) void {
    const source = readFile(path, allocator) catch |err| {
        std.debug.print(
            "Error reading from {s}: {any}",
            .{ path, err },
        );
        std.process.exit(1);
    };
    defer allocator.free(source);

    var scanner = Scanner.init(source);
    std.debug.print("Scanning...\n", .{});
    while (scanner.next()) |token| {
        token.print();
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len == 1) {
        std.debug.print("Start REPL!", .{});
    } else if (args.len == 2) {
        const path = args[1];
        runFromFile(path, allocator);
    }
}
