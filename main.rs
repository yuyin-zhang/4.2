fn main() {
    let x = u32::max_value() - 2;
    let y = 3;

    match x.checked_add(y) {
        Some(v) => {
            println!("{} + {} = {}", x, y, v);
        }
        None => {
            println!("None");
        }
    };
}