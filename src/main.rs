fn main() {
    typage();
    mutabilite();
    ownership();
    returnFromIf();
    matchSwitch();
    patternBinding();
    matchPatternBinding();
}

fn matchPatternBinding(){
    let a = ("div", 50, 3);

    let (op, res) = match a { 
        (op, x, y) if op == "div" => ("Division", x/y),
        _ => ("erreur", 0)
    };
    
    println!("Opération : {}, Résultat : {}", op, res);
}

fn patternBinding(){
    let (a, b) = (13, 14);
    
    println!("{a} - {b}");
    
    let (c, _, d) = (15, 16, 17);
    
    println!("{c} - {d}");
}

fn matchSwitch(){
    let a: i32 = 8;
    
    match a {
        n if n < 0 => println!("erreur {n}"),
        _ if a == 6 => println!("6"),
        2 | 4 | 8 => println!("2 4 ou 8 !"),
        0 => println!("zéro"),
        1..=17 => println!("mineur"),
        _=> println!("majeur")
    }
}

fn returnFromIf(){
    let a = 13;
    let b = 15;

    let result: String = if a < b { "true".to_owned() } else { "false".to_owned() } ;

    println!("{result}");
}

fn ownership(){
    let a = "Hello".to_owned();
    let mut b = a.clone();
    
    b.push_str(" World");
    
    println!("{a} - {b}");
}

fn mutabilite(){
    let mut a: u16 = 56;
    println!("{a}");

    a = 16;

    println!("{a}");
}

fn typage(){
    let test_var = 50; //Déclaration var typées automatiquement
    let test_var_typee: u8 = 52;
    let test_var_typee_diff = 52u8;
    let bigvar:i64  = 15_000_000_000;
    let hex = 0xff;
    let bin = 0b10011;


    println!("Print de la variable : {test_var}");
    println!("Print de la variable : {test_var_typee}");
    println!("Print de la variable : {test_var_typee_diff}");
    println!("Print de la variable : {bigvar}");
    println!("Print de la variable : {hex}");
    println!("Print de la variable : {bin}");
}