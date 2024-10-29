use std::io;

    //Data Types
    //- Scalar
        //- Unsigned Int: u8-128
        //- Signed Int: i8-128 (Two's complement)
        //- arch: isize/usize
    //- Number literals
        //- Separator '_'
        //- Hex 0x...
        //- Octal 0o...
        //- Binary 0b...
        //- byte (u8) b'...'
    //- Real
        //- f32, f64 (default)
    //- chars
        //- 4 bytes
        //- single quotes
    //- Compounds
        //- Tuple 
        //- Array

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
