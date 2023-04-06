fn main() {
    let my_var: usize = 1;
    let my_int: isize = -1;
    let floats: f32 = 1.9;
    let double: f64 = 23.8;
    let new_var = 8u8; // only with numbers
    let with_underscore = 88_000; // == 88000
    let character: char = 'k'; // single utf-8 encoded character
    let string: &str = "adfsadfasfd"; // string slice
    let boolean: bool = true; //  or false
    println!("Hello, world!");
    println!("my var is {}", my_var);
    println!("my var is {my_var}");

    // let add5 = my_var + 5; // +, -, /, *, |, &, << everything

    // let mut mutable_var = 3; // mutable, changeable

    // mutable_var = 5;

    // let temperature: f32 = calculate_temp();

    // // using temperature

    // temperature = temperature + 5

    // let if_var = if temperature == 1.2 {

    // } else {
        
    // }

    let cond_var = if my_var == 1 {
        println!("eight");
        8
    } else {
        println("ten");
        // 10
        "sadfasfd"
    }

    


}

//