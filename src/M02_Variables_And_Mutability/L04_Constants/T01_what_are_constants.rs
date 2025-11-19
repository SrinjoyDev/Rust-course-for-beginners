

//constant is a name assigned to a value.
//a constant's value cannot change throughout the program
//we cant mutate constants

// a constants value must be known at compile time.
//therefore a constant varaiable value should be know when the rust compiler
//actually builds your program
//we can globally declare a constant variable and use it inside any scope
//or any function

//a good convention is to declare a constant variable we use all caps

const TAX_RATE : i32 = 100; //a constant's type should be manually decalred while decalring the constant variable

fn main() {
    let income = 200;
    println!("My income is {income} and the tax rate is {TAX_RATE}");
}