fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // let x = 5;

    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");
    // Ok
    // let spaces = "   ";
    // let spaces = spaces.len();
    // Err
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces is: {spaces}");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundread = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}", five_hundread);
    println!("{}", six_point_four);
    println!("{}", one);

    let a = [1, 2, 3, 4, 5];

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 5];
}
