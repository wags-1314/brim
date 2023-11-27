//- Imports --------------------------------------------------------------------
const std = @import("std");

//- Definitions & Aliases ------------------------------------------------------
const Allocator = std.mem.Allocator;
const stderr = std.io.getStdErr().writer();

pub fn readFile(path: []const u8, allocator: std.mem.Allocator) ![]const u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();

    const file_size = (try file.stat()).size;
    return try file.reader().readAllAlloc(allocator, file_size);
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
        const file_name = args[1];
        const source = readFile(file_name, allocator) catch |err| {
            std.debug.panic(
                "Error reading from {s}: {any}",
                .{ file_name, err },
            );
            std.process.exit(1);
        };
        defer allocator.free(source);

        std.debug.print("{s}", .{source});
    }
}
