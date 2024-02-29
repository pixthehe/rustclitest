use clap::Parser;

#[derive(Parser)]
struct TestStcuc {
    TestString: String,
}

fn main() {
    let args = TestStcuc::parse();

    if args.TestString == "testString" {
        println!("this works");
    }
    println!("Hello, world! {}", args.TestString);
}
