// zig build -Dtarget=x86_64-windows

const std = @import("std");
const Build = std.Build;

pub fn build(b: *Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "app",
        .target = target,
        .optimize = optimize,
    });

    exe.addCSourceFiles(.{
        .files = &.{
            "main.cpp",
        },
        .root = b.path("."),
        .flags = &.{
            "-std=c++23",
        },
    });

    exe.linkLibCpp();
    exe.addIncludePath(b.path("../../include"));

    // if (target.result.os.tag == std.Target.Os.Tag.windows) {
    //     exe.addLibraryPath(b.path("../../windows/lib"));
    // } else if (target.result.os.tag == std.Target.Os.Tag.linux) {
    //     exe.addLibraryPath(b.path("../../linux/lib"));
    // }

    exe.addLibraryPath(b.path("../../linux/lib"));
    // exe.addLibraryPath(b.path("../../windows/lib"));
    exe.linkSystemLibrary("excel_to_csv");
    b.installArtifact(exe);
}
