//mutable means -> capable of change

//we can simply overwrite a variable.
fn main() {
    /*
    let gym_reps : i32 = 10;
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    */

    //this above code will throw error , it will throw the variable gym_reps is not mutable
    //to make it mutable we can do something like this:

    //we make a variable mutable , when we want to change the course of the value of the variable
    //during the programs's execution.
    let mut gym_reps = 10; //specifying mut here , to tell rust this variable is mutable
    println!("i plan to do {gym_reps} reps in gym today!!");

    gym_reps = 15; //now we can update the variable value here.
    //but see we still cant change the type of the variable.
    println!("i plan to do {gym_reps} reps in gym today!!");

    //the above example was for primitives.
    //for primitive variables rust mutability is no different.

    //what happens under the hood for primitive variables is:
    //when we deinf let mut gym_repos = 10;
    //it gets stored in main stack only not in heap.

    //so it is like>
    /*

        Stack Frame (main)
        +------------------------+
        | gym_reps: i32 = 10     |
        +------------------------+

    */

    //when we do:
    //gym_reps = 15;
    //ur question could be what happens to the value 15 , nothing happens it gets replaced by the new value.
    //now
    /*
        
        Stack Frame (main)
        +------------------------+
        | gym_reps: i32 = 10     |
        +------------------------+
        
    */

    //always remember primitives lives on the stack memory of a program always

    //for String things will be different that is where rust shines here.
    //in other languages like java for example.
    /*
     String s = "hello!",
     s = "World";
    */

    //in java or other languages what happens is>
    //before reassignment>

    /*

        STACK                              HEAP
        +-----------+                 +--------------------+
        | s -------+ | -------------> | "Hello" object     |
        +-----------+                 +--------------------+

    */

    //after reassignment

    /*
    
        STACK                              HEAP
        +-----------+                 +--------------------+
        | s -------+ | -------------> | "World" object     |
        +-----------+                 +--------------------+

        "Hello" object (still on heap)
        Marked as unreachable → GC will remove it later

    */

    //whereas how rust handles this is:
    let mut s = String::from("Hello");

    //a rust string is stored as like this>

    /*
    
            Stack                    Heap
        ----------------------------------------------------
        s.ptr  ---------------->  [ H e l l o ]   <-- data
        s.len  = 5
        s.cap  = 5-12 (capacity)

    
    */

    //reassignment

    s = String::from("World");

    /*

        STACK                                HEAP
        +--------------------+              +--------------------+
        | s.ptr  ----------+ | ----------> | "World"            |
        | s.len  = 5        |              +--------------------+
        | s.cap  = ...      |
        +--------------------+
    
    */

    //old memory freed , immediately after reassignment , rust has no garbage collectors.
    //whereas in java it would take some time for the GC to remove the old reference.
    //thats why Rust is so memory efficient and lightning fast.


}