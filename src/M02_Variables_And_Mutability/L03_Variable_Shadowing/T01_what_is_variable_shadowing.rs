

fn main() {
    //variable shadowing means redeclaring a variable.
    //The original variable is replce by the new one.
    //the first variable declaration becomes invalidated/overshadowed.

    //common use case is we want to reassign the value of the variable
    //but we want to use the same name of the variable through out the process.


    let grams = "100.123"; //this is arriving as a string ,which is a number

    //we would want to convert this string which is a number actually to a decimal point number
    //which is a flating point number
    //even if it is mutable we cant do like this>
    /*
        let mut grams = "100.123";
        grams = 100.123; //this will throw type mismatch error.
        //we cant mutate types
    */

    //we can simply invalidate the prev declaration of the variable grams like this
    let grams = 100.123; //now grams is a flaoring point memory
    //the above created variable overshadowing we overshadow the prev declaraion

    let mut grams = 100 ; //again overshadowing the prev declaration
    grams = 101;
    println!("grams is {grams}");
}