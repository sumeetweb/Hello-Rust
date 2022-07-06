// u8 - u128, usize, i8-i128, isize, str, String, char, bool, tuple, array, vector, hashmap 

fn add(x:i32, y:i32) -> i32 {
    let result = x + y;
    result
}

fn max(x:i32, y:i32) -> i32 {
    if x > y {x} else {y}
}

fn main() {
    println!("Hello World");
    let mut city = String::from("Bhubaneswar");
    city.push_str(" is a city in India.");
    println!("{}", city);
    println!("{}", add(10, 15));

    // For loop
    let mut n = 1;
    for supplement in 0..5 {
        println!("{}", supplement);
        n += supplement;
    }
    println!("Result is {}", n);

    // While Loop
    let mut sup = 0;
    let mut num = 1;
    while sup < 5 {
        num += sup;
        sup += 1;
    }
    println!("Result is {}", num);

    // Infinite Loop
    let mut supp = 0;
    let mut numm = 1;

    loop {
        numm += supp;
        supp += 1;
        if supp >= 5 {
            break;
        }
    }
    println!("Result is {}", numm);



}

