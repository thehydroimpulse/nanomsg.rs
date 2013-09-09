
// useful examples of code from bjz (thanks!):
/*
 https://github.com/bjz/glfw-rs#example-code
 https://github.com/bjz/glfw-rs/blob/master/src/glfw/lib.rs#L645
 https://github.com/bjz/glfw-rs/blob/master/src/glfw/lib.rs#L1069
*/

// if you want to be sure you are running on the main thread,
// do this:
#[start]
#[fixed_stack_segment]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    // Run on the main thread
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

// TODO: figure out how to make a safe interface that
//       wraps all these unsafe calls.

