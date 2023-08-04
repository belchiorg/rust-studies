use std::io::stdin;

fn add(value : &mut f64) {
    println!("{} + ?", *value);
    let mut second_val = String::new();
    stdin().read_line(&mut second_val).expect("Couldn't read line");
    
    match second_val.trim().parse::<f64>() {
        Ok(numeric) => {
            *value = *value + numeric;
        },
        Err(_) => println!("Invalid Number")
    }
}

fn subtract(value: &mut f64) {
    println!("{} - ?", value);
    let mut second_val = String::new();
    stdin().read_line(&mut second_val).expect("Couldn't read line");

    match second_val.trim().parse::<f64>() {
        Ok(val) => *value = *value - val,
        Err(_) => println!("Invalid number"),
    } ;
}

fn multiply(value: &mut f64) {
    println!("{} * ?", value);
    let mut second_val = String::new();
    stdin().read_line(&mut second_val).expect("Couldn't read line");

    match second_val.trim().parse::<f64>() {
        Ok(res) => *value = *value * res,
        Err(_) => println!("Invalid number"),
    };
}

fn divide(value: &mut f64) {
    println!("{} / ?", value);
    let mut second_val = String::new();
    stdin().read_line(&mut second_val).expect("Couldn't read line");

    match second_val.trim().parse::<f64>() {
        Ok(res) => *value = *value / res,
        Err(_) => println!("Invalid number"),
    };
}

fn main() {
    println!("Welcome to this amazing calculator that only supports 4 operations");

    println!("What is your first number?");

    let mut val = String::new();
    let mut operator = String::new();
    let mut val_numeric: f64; 

    loop {
        stdin().read_line(&mut val).expect("Failed to read line");
        match val.trim().parse::<f64>() {
            Ok(ok) => {
                val_numeric = ok;
                break;
            },
            Err(_) => println!("Invalid number"),
        };
    }



    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Current value: {}", val_numeric);

        println!("\nWhat operation do you want to perform? (\"+\",\"-\",\"*\",\"/\") (0 to cancel)");
        stdin().read_line(&mut operator).expect("Failed to read line");

        match operator.trim() {
            "+" => add(&mut val_numeric),
            "-" => subtract(&mut val_numeric),
            "*" => multiply(&mut val_numeric),
            "/" => divide(&mut val_numeric),
            "0" => break,
            _ => println!("Invalid operator")
        };

        operator = String::from("")
    }

}
