# braillix

A Rust library providing a simulated dot-matrix display created with
braille unicode characters (`U+2800-28FF`).

In its current state, the library provides a `Display` struct that provides
low-level functionality for setting and clearing individual braille "dots",
as well as a `Canvas` struct that builds on `Display` to expose simple
rastering functions for drawing lines, triangles, rectangles, etc.

Currently under development.
