fn main() {
    typage();
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