use std::io;

fn basic_types() {
    let mut age: i32 = 19;
    age = 20;
    println!("My age is {}", age);

    let name = String::from("Pavel");
    println!("My name is {}",name);

    let is_true: bool = false;
    let _num = if is_true { 1 } else { 0 };
}

fn loops() {
    // inifinity loop
    loop {
        println!("infinity!");
        break;
    }

    for i in 0..101 {
        print!("{i} ");
    }
}

fn matchs() {
    // match (switch case)
    let mut number = 10;
    match number {
        n if n % 2 == 0 => { println!("even: {}", n); },
        10..=50 => { print!("между 10 и 50"); },
        _ => { println!("odd"); }
    }

    number = match number {
        2 => 1,
        10 => 11,
        55..=1111 => number % 55,
        _ => 0
    }; 
    println!("{}", number);

    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => print!("Здарова, {}, заебал", name), // _ – это колчество прочитанных байт
        Err(e) => println!("Error – {}", e),
    }
}

fn arrays() {
    let mut array = [1,2,3,4,5];

    array[4] = 2;
    println!("{:?}", array);

    array = [7; 5]; // пять семерок
    println!("{:?}", array);

    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("{:?}", squared_numbers);

    for i in squared_numbers.iter() {
        print!("{} ", i);
    }
}

fn constants() {
    const NUM: i8 = 3; // Constant `num` should have UPPER_SNAKE_CASE name, e.g. `NUM`
    println!("{}", NUM);
}

fn tuples() {
    let tuple = (12, 5.0, String::from("kekw"), false, 'a');
    let (int, float, str, boolean, char) = tuple;
    println!("int: {}, float: {}, str: {}, boolean: {}, char: {}", int, float, str, boolean, char);
    println!("tuple[0]: {}", tuple.0);
}

fn get_user_str() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("");
    str.pop();
    str
}

fn main() {
    print!("{}", get_user_str());
}
