
//lets say we want to specify our distance from home.
//lets say it is 100m
//we can save the value as 100.
//it is i32. which is fine.
//for better specification and type clearly we can decalre the type more nicely.
//as metres lets say.

type Metres = i32; //we declare a type that is i32 .

type Centimetres = i32;

fn main() {
    //and we refer the type metres which is itself an i32 , it is the same only , but it is for better 
    //readable maintainable and type safe code.
    let distance_from_home: Metres = 100;
    let height : Centimetres = 192;

    println!(" distance from home is {distance_from_home} and my height is {height}");
}