fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = {
        let y = 6;
        y + 1
    };
    println!("The value of x is: {x}");

    let five = five();
    println!("The value of five is: {five}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
