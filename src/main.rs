use rand::Rng;
use std::collections::HashMap;
use std::io::Error;

mod hello_module;
use crate::hello_module::hello_mod;
mod user;
use crate::user::Gender;
use crate::user::User;

mod shapes;
use crate::shapes::*;

fn main() {
    typage();
    mutabilite();
    ownership();
    returnFromIf();
    matchSwitch();
    patternBinding();
    matchPatternBinding();
    arrays();
    tuples();
    vectors();
    hashMaps();

    match get_random_number(-10, 10) {
        None => println!("None"),
        Some(x) => println!("Random number : {}", x),
    }

    let text: String = String::from("value");
    say(&text);
    say(&text);

    hello_mod::hello();

    structs();
    traits();
}

fn traits() {
    let r = Rect::new(32.0, 52.0);
    let c = Circle::new(41.6);

    calc_surface(r);
    calc_surface(c);
}

fn calc_surface(shape: impl Shape) {
    println!("{}", shape.surface());
}

fn structs() {
    let user = User::new(
        "Test".to_owned(),
        16,
        Gender::Other("Test".to_owned()),
        None,
    );
    println!("{:?}", user);
    println!("{:#?}", user);

    user.hello();

    //user.kill_me();

    println!("1/2 = {:?}", divide(1.0, 2.0).unwrap());
    //println!("1/0 = {:?}", divide(1.0, 0.0).unwrap());
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        return Err("Cannot divide by zero! dumbass".to_string());
    }

    Ok(a / b)
}

fn say(text: &String) {
    println!("{}", text);
}

fn get_random_number(min: i32, max: i32) -> Option<i32> {
    if (min > max || max < min) {
        return None;
    }

    let mut rng = rand::thread_rng();

    return Some(rng.gen_range(min..max));
}

fn hashMaps() {
    let mut a = HashMap::new();

    a.insert("test", 13u8);

    println!("{:?}", a);
}

fn vectors() {
    let mut vec = vec![1, 3, 4, 5];

    println!("{:?}", vec);
}

fn tuples() {
    let a = (1, 2, 3, "string");
    let b = (1, (2, (3, 4)));

    println!("{}", a.3);
    println!("{}", b.1 .1 .0);
}

fn arrays() {
    let mut a: [u8; 4] = [1, 2, 3, 4];
    println!("{:?}", a);

    a[2] = 16;
    println!("{:?}", a);

    let slice = &mut a[0..3];

    println!("Slice: {:?}", slice);
    slice[1] = 67;

    println!("Array : {:?}", a);
}

fn matchPatternBinding() {
    let a = ("div", 50, 3);

    let (op, res) = match a {
        (op, x, y) if op == "div" => ("Division", x / y),
        _ => ("erreur", 0),
    };

    println!("Opération : {}, Résultat : {}", op, res);
}

fn patternBinding() {
    let (a, b) = (13, 14);

    println!("{a} - {b}");

    let (c, _, d) = (15, 16, 17);

    println!("{c} - {d}");
}

fn matchSwitch() {
    let a: i32 = 8;

    match a {
        n if n < 0 => println!("erreur {n}"),
        _ if a == 6 => println!("6"),
        2 | 4 | 8 => println!("2 4 ou 8 !"),
        0 => println!("zéro"),
        1..=17 => println!("mineur"),
        _ => println!("majeur"),
    }
}

fn returnFromIf() {
    let a = 13;
    let b = 15;

    let result: String = if a < b {
        "true".to_owned()
    } else {
        "false".to_owned()
    };

    println!("{result}");
}

fn ownership() {
    let a = "Hello".to_owned();
    let mut b = a.clone();

    b.push_str(" World");

    println!("{a} - {b}");
}

fn mutabilite() {
    let mut a: u16 = 56;
    println!("{a}");

    a = 16;

    println!("{a}");
}

fn typage() {
    let test_var = 50; //Déclaration var typées automatiquement
    let test_var_typee: u8 = 52;
    let test_var_typee_diff = 52u8;
    let bigvar: i64 = 15_000_000_000;
    let hex = 0xff;
    let bin = 0b10011;

    println!("Print de la variable : {test_var}");
    println!("Print de la variable : {test_var_typee}");
    println!("Print de la variable : {test_var_typee_diff}");
    println!("Print de la variable : {bigvar}");
    println!("Print de la variable : {hex}");
    println!("Print de la variable : {bin}");
}
