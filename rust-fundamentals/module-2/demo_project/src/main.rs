fn main() {
    let mut message = String::from("Weight in kilos: ");
    message.clear(); 
    let weight  = 190.0;
    let kilos = weight / 2.2;
    println!("{}{}", message, kilos);
    let mut height = 185;
    println!("Height in cm: {}", height);
    height = 73;
    println!("Height in inches: {}", height);
}
