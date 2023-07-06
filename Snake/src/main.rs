mod plate;

fn main() {
    let mut plt: Vec<Vec<i32>>;
    plt = plate::read_map();
    println!("{:?}", plt);
}
