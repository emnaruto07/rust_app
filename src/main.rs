fn main() {
    let mut naruto: String = String::from("hello sarthak");
    let mut sarthak: &mut String = &mut naruto;
    //sarthak = Ox4456
    //sarthak = Test


    let sarthak: &str = "Test"; // variable shadowing;

    println!("{sarthak}");
    // println!("{sarthak}");
    println!("{naruto}");
}

// fn main() {
//     let mut number = 5;
//     let mut number2 = 9;

//     let mut my_ref = &mut number;

//     *my_ref += 5;

//     println!("number is {}", *my_ref);

//     my_ref = &mut number2;

//     *my_ref += 3;

//     println!("number2 is {}", *my_ref);

//     // let mut number = 5;
//     // let mut my_mut_ref: &mut u8 = &mut number; // read and write

//     // let mut my_ref: &u8 = &number; 
    






//     // let number: u8  = 4;

//     // let my_ref: &u8 = &number; // readonly

//     // let calc = *my_ref + 3;
//     // // *my_ref = 3; // invalid

//     // let mut mut_number = 3;

//     // let my_immut_ref = &mut_number; // readonly
//     // // *my_mut_ref = 5; // change

//     // *my_mut_ref = 3434; // invalid
//     // my_mut_ref = &mut_number; // valid

//     // // *my_immut_ref = 6;
// }

// // fn main() {
// //     let my_var: usize = 1;
// //     let my_int: isize = -1;
// //     let floats: f32 = 1.9;
// //     let double: f64 = 23.8;
// //     let new_var = 8u8; // only with numbers
// //     let with_underscore = 88_000; // == 88000
// //     let character: char = 'k'; // single utf-8 encoded character
// //     let string: &str = "adfsadfasfd"; // string slice
// //     let boolean: bool = true; //  or false
// //     println!("Hello, world!");
// //     println!("my var is {}", my_var);
// //     println!("my var is {my_var}");

// //     // let add5 = my_var + 5; // +, -, /, *, |, &, << everything

// //     // let mut mutable_var = 3; // mutable, changeable

// //     // mutable_var = 5;

// //     // let temperature: f32 = calculate_temp();

// //     // // using temperature

// //     // temperature = temperature + 5

// //     // let if_var = if temperature == 1.2 {

// //     // } else {
        
// //     // }

// //     let cond_var = if my_var == 1 {
// //         println!("eight");
// //         8
// //     } else {
// //         println("ten");
// //         // 10
// //         "sadfasfd"
// //     }

    


// // }

// // //