use librstab;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        librstab::add_one(num)
    );
}
