fn main() {
    let mut x = 5;

    { // Scope for y
        let y = &mut x;
        *y = 10;
    }
    { // Scope for z
        let z = &mut x;
        *z = 20; 
    }

    println!("x = {}", x); // Prints x = 20
} 