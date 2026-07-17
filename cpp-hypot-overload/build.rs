fn main() {
    cpp_build::Config::new()
        .flag("-std=c++17")
        .build("src/bin/cpp-hypot-overload.rs");
}
