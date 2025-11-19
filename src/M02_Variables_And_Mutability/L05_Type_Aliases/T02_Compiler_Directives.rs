// a compiler directive is an annotation that tells the compiler how to parse
//the source code.

#[allow(unused_variables)]
fn add(a : i32 , b : i32 ) -> i32 {
    let sum = a + b;
    return sum;
}

fn main() {
    let _unusedVar = 1; //using underscore is also telling rust that this is an unused code , and i am fine with it.

    //another alternative is to:
    #[allow(unused_variables)] //telling rust this is unused variable using compiler directive , and telling them to be fine with it.
    let anotherUnusedVar = 2;

    //note we can also use compiler directives to functions too.
    //or the entire program file too.
    //to do that we do the same thing but with an extra "!"
    //eg : #![allow(unused_variables)] //this tells rust to be fine with unused methods and variables for the entire program file
}