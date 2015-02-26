find_package(rustc)
include(Rust)

project(hello-world)
add_executable(hello-world hello_world.cpp)

project(hello-world-rs)
rust_crate_auto(hello_world.rs ALL TARGET_NAME hello-world-rs)

project(hello-world-d D)
add_executable(hello-world-d hello_world.d)
