pub fn exec() {
    let maybe_number = Some(42);

    if let Some(numbers) = maybe_number {
        println!("{}", numbers);
    } else {
        println!();
    }
}
