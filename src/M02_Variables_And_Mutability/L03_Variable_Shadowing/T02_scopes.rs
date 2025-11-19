
// a scope is a boundary or region where a name is valid.
//bloc : is the area between the opening curly braces and the closing curly braces

fn main() {
    let price = 100;
    //variable price scope is inside this main method only.
    //this variable name is valid only in the main function scopre
    //therefore cant be used outside it.
    //outside the scope any values delclared within it , are cleared by rust.

    {
        //we created a block
        //nested scope
        //we can define some names here . 
        //that cant be accesed outside this scope.
        //names outside this scopre cant be accesed here in the inner scopre
        //therefore
        println!("{price}$ is the price of a mobile phone");

        let cookie_Price = 10;
        //it is not accessable outside this inner scope

        let price = 120; //is this variable overshadowing?
        //answer is no because , this is another scope
    }
}

//u cant use this here , it is outside the scope
// println!(price);