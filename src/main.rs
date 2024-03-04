use clap::Parser;

#[derive(Parser)]
struct TestStcuc {
    test_string: String,
}

fn main() {
    let args = TestStcuc::parse();

    if args.test_string == "testString" {
        println!("this works");
    }
    println!("Hello, world! {}", args.test_string);
}
