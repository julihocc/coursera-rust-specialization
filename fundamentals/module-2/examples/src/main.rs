mod examples;

fn main() {

    let example = 3;
    
    if example == 0 {
        examples::demo();
    } 
    if example == 1 {
        examples::loops();
    }
    if example == 2 {
        examples::conditionals();
    }  
    if example == 3 {
        examples::conditionals_with_some();
    }
}
