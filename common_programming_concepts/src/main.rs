/// Common programming concepts applied in Rust.
fn main() {
    println!("Common programming concepts applied in Rust");
    println!("");

    // declare variables
    let x = 5;
    println!("Variables: 'let x = 5;' results in x being {}", x);
    // x is immutable by default
    // the following line would causea compilation error as x cannot be changed
    // x = 6;
    println!("");

    // Type syntax
    // Variables must have a type, which can be explicit using the : syntax, or inferred.println!
    let defined_type: i32 = 100;
    println!(
        "Type specification syntax: defined_type : i32 has value {}",
        defined_type
    );
    println!("");

    // Constants
    // constants values are known at compile time and cannot be changed.println!
    // Rust convention is that they be in uppercase.
    // A type must be provided.
    const C: i8 = 100;
    println!("Constants: C is declared a const and is set to {}", C);
    // We cannot change C once declared.
    println!("");

    // Mutability
    let mut y = 10;
    println!(
        "Mutability: We can declare y as mutable and set it to {} .",
        y
    );
    y = 20;
    println!(
        "Mutability: We can then change the value of y to {} as it was delared mutable.",
        y
    );
    println!("");

    // Shadowing
    // while we cannot change the value of a immutable variable, we can shadow it.
    // shadowing is essentially reusing the variable name.println!
    let x = 6;
    println!(
        "Shadowing: x is now {} as we rebound it to an new value but kept its name.",
        x
    );
    println!("");

    println!("Data Types - Integers");
    println!("providing type information");
    println!("\tlet signed_8_bit: i8 = 100;");
    println!("\tlet signed_8_bit = 100i8;");
    println!("");

    println!("Integer Sizes");
    println!("Length\tSigned\tUnsigned");
    println!("8\ti8\tu8");
    println!("16\ti16\tu16");
    println!("32\ti32\tu32");
    println!("64\ti64\tu64");
    println!("128\ti128\tu128");
    println!("arch\tisize\tusize");
    println!("The 'arch' types (isize and usize) are architecture specific lengths.");
    println!("");

    // INTEGERS
    let some_8_bit = 100i8;

    let signed_8_bit: i8 = 100;
    let signed_16_bit: i16 = 100;
    let signed_32_bit: i32 = 100;
    let signed_64_bit: i64 = 100;
    let signed_128_bit: i128 = 100;
    let signed_isize_bit: isize = 100;

    let unsigned_8_bit: u8 = 100;
    let unsigned_16_bit: u16 = 100;
    let unsigned_32_bit: u32 = 100;
    let unsigned_64_bit: u64 = 100;
    let unsigned_128_bit: u128 = 100;
    let unsigned_isize_bit: usize = 100;

    // FLOATING POINT
    let floating_1 = 1.0; // f64 apparently is default.
    let floating_2: f32 = 3.14; //f32

    println!("Integer Literals: ");
    println!("The _ is ignored in number definitions and can be used for formatting.");
    println!("\tlet some_decimal = 100_434;");
    println!("");

    let some_decimal = 100_434;
    let some_hex = 0x44;
    let some_octal = 0o11;
    let some_binary = 0b_0000_1100_0011_1111;
    let some_byte = b'G';

    println!("Numeric operations");
    let addition = 1 + 2;
    let subtraction = 21 - 10;
    let multiplication = 21 * 10;
    let division = 21 / 10;
    let remainder = 21 % 10;
    println!("");

    println!("Boolean");
    let t = true;
    let f: bool = false;
    println!("");

    // Characters
    let c = 'x';
    let dollar = '$';

    // Compound Types
    let tup = (1, 2, 3);
    let tup2: (i8, u8, usize) = (1, 2, 3);
}
