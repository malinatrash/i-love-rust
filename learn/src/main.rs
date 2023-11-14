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

#[derive(Debug)]
struct Person {
    name: String,
    lastname: String,
    age: u8,
    balance: f64
}

#[derive(Debug)]
struct Tuple(i32, String, f32); // это структурный кортеж

struct Triangle {
    cat1: f64,
    cat2: f64,
}

impl Triangle {
    fn find_hyp(&self) -> f64 {
        (self.cat1 * self.cat1 + self.cat2 * self.cat2).sqrt()
    }

    fn find_area(&self) -> f64 {
        0.5 * self.cat1 * self.cat2
    }

    fn create_isc(cat: f64) -> Triangle {
        Triangle { cat1: cat, cat2: cat }
    }
}

fn structs() { 
    let person1 = Person {
        name: "p1 name".to_string(),
        lastname: "p1 lastname".to_string(),
        age: 19,
        balance: 4441.1
    };
    print!("{:#?}", person1);


    let tr1 = Triangle {
        cat1: 3.0,
        cat2: 4.0
    };
    println!("hyp is: {}", tr1.find_hyp());
    println!("area is: {}", tr1.find_area());
    
    let isc_tr = Triangle::create_isc(3.0);
    println!("isc hyp is: {}", isc_tr.find_hyp());
    println!("isc area is: {}", isc_tr.find_area());

}

#[derive(Debug)]
struct ListNode {
    value: i32,
    next_node: Option<Box<ListNode>>,
}

impl ListNode {
    fn print_node(&self) {
        print!("{} ", self.value);
        if let Some(next) = &self.next_node {
            next.print_node();
        }
    }
}

fn main() {
    let head: ListNode = ListNode {
        value: 124,
        next_node: Some(Box::new(ListNode {
            value: 141,
            next_node: None,
        })),
    };

    head.print_node();
}