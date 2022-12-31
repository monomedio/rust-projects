fn main() {
    // SHADOWING
    // second variables shadows first one until it is out of scope or shadowed itself
    let x = 5;
    let x = 6;
    {
        let x = x * x;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");


    // RUST SCALAR TYPES, one single value


    // INTEGERS
    let x: i32 = 3_000; // default integer type is i32, and underscore _ used to visually separate numbers
    // rust compiler does not check for integer overflow when building with the --release flag

    // FLOATING POINT NUMBERS
    // either f32 or f64
    // default type is f64, double precision floating point numbers
    let x: f64 = 64.1234;

    // NUMERICAL OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division (integer division)
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // BOOLEAN
    // 1 byte in size
    let t = true;
    let f: bool = false; // explicit type annotation

    // CHARACTER
    // 4 bytes in size, supports a wide range of characters
    // Unicode Scalar Value
    // denoted by single quotes ''
    let character: char = 'S';
    let character = 'A'; // shadows the previous variable


    // RUST COMPOUND TYPES, group multiple values into one type


    // TUPLES
    // fixed length, once declared cannot shrink or grow in size
    let tup: (i32, f64, char) = (5, 6.912, 'L');

    // accessing tuple elements
    //1. using dot notation to access element directly (zero-indexed)
    let first = tup.0;
    let second = tup.1;

    // 2. use pattern matching to destructure a tuple
    let (x, y, z) = tup;
    println!("Tuple's first element is {x}, second element is {y}, third is {z}.");
    // The tuple without any values has a special name, unit. This value and its 
    // corresponding type are both written () and represent an empty value or an 
    // empty return type. Expressions implicitly return the unit value if they 
    // don’t return any other value.

    // ARRAYS
    // every element must share the same type
    // fixed length
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [6, 7, 8, 9, 10]; // with type annotation

    // initializing array to have x amount of the same value
    let arr3 = ['L'; 5];

    let arr3_0: char = arr3[4];

    // accessing array elements
    println!("Fifth element of arr3 is {arr3_0}");

    // Rust at runtime will check if a given index is out-of-bounds, unlike 
    // many low-level languages

}
