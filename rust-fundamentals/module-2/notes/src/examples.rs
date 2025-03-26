pub fn demo() {
    let mut message = String::from("Weight in kilos: ");
    message.clear();
    let weight = 190.0;
    let kilos = weight / 2.2;
    println!("{}{}", message, kilos);
    let mut height = 185;
    println!("Height in cm: {}", height);
    height = 73;
    println!("Height in inches: {}", height);
}

pub fn conditionals() {
    let maybe_number = Some(42);

    if let Some(numbers) = maybe_number {
        println!("{}", numbers);
    } else {
        println!();
    }
}

pub fn loops() {
    let mut x = 1;

    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }
}
