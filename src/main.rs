fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 + val2;

    println!("answer = {} \n", ans);


    println!("-------------------------------");


    let mut vector = vec![2,4,6,8,10]; 

    println!("Current values in vector: {:?}", vector);

    vector.remove(4);
    vector.push(12);


    println!("After transforming: {:?}", vector);


    println!("-------------------------------");

    let mut my_string:String = "Hello".to_string();

    my_string = concat_string(my_string);

    println!("Result: {}", my_string);

    control_flow(1);

}


// Create a function called "concat_string". Create a string variable and assign the value "Hello" to it. 
// The function is going to take one argument that is of type string and is going to return a String. 
// Inside this function, concatenate the string " World". Print out the results in main() to confirm your results.

fn concat_string(char: String ) -> String{

    let world = char + " World";

    world
}


// Create a function called control_flow. This is going to take one argument that is an integer. Based on this integer, print out the following: "The value is one", "The value is greater than 50", "The value is less than 25", or "The value is greater than 25 but less than 50".

fn control_flow(value: i32){
    if value == 1 {
        println!("The value is one");
    }else if value > 50 {
        println!("The value is greater than 50");
    }else if value < 25 {
        println!("The value es less than 25");
    }else {
        println!("the value is greater than 25 but less than 50");
    }

}

use std::process::Command;

// fn main() {
//     let output = Command::new("cd")
//         .arg("..")
//         .output()
//         .expect("falló al ejecutar mkdir");

//     if output.status.success() {
//         println!("Directorio creado exitosamente");
//     } else {
//         let error_message = String::from_utf8_lossy(&output.stderr);
//         println!("Error al crear el directorio: {}", error_message);
//     }
// }



