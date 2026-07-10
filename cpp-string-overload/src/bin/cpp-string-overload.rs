use cpp::cpp;

cpp! {{
    #include <iostream>
}}

fn main() {
    println!("This does nothing in Rust yet");

    unsafe {
        cpp!([] -> () as "void" {
            std::cout << "This does nothing in C++ yet" << std::endl;
        });
    }
}
