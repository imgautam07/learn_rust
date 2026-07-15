fn main() {
    let x = 5;
    println!("The value x is: {}", x);
    // x = 6 : We can't assign to a immutable variable
    //
    let mut y = 10;
    println!("The value of old y is: {}", y);
    y = 15; // Value changed if it's mutable.
    println!("The value of new y is: {}", y);

    const /*mut*/ Z: u32 = 200000; // const can't be mutable and always type annotated.
    println!("The value of Z is: {}", Z);



    let x = 5;
    println!("The value of x is: {}", x);
    let x = "five"; // x is shadowed here, so this is valid.
    println!("The value of x is: {}", x);




    // Scaler data types ( single values )
    // Integer(signed & unsigned (8, 16, 32, 64, 128)), Floating Point, Boolean, Character
    let x = 5; //Decimal
    let x = 0xff; //Hexadecimal
    let x = 0o77; //Octal
    let x = 0b1111_0000; //Binary
    let x = b'A'; //Byte


    let y = 3.14;

    let z = true;

    let c = 'a';
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of c is: {}", c);



    // Compound data types ( collections )
    // Arrays, Vectors, Strings, Tuples, Structs, Enums
    let arr = [1, 2, 3, 4, 5]; // Arrays (Static size)
    let vec = vec![1, 2, 3, 4, 5]; // Vectors (Dynamic size)
    let str = "Hello, World!"; // Strings
    let tuple = (1, 2, 3); // Tuples

    println!("The value of arr is: {:?}", arr);
    println!("The value of vec is: {:?}", vec);
    println!("The value of str is: {}", str);
    println!("The value of tuple is: {:?}", tuple);

    let (x, y, z) = tuple;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    let x = tuple.1;
    println!("The value of x is: {}", x);

    let x = arr[3];
    println!("The value of x is: {}", x);




    let res = new_func(5, 3);
    println!("The result is {}", res);


    if res > 10 {
        println!("The result is above 10");
    }
    else if res > 20 {
        println!("The result is above 20");
    }
    else {
        println!("The result is something else.");
    }


    let condition = res > 10;
    let num = if condition { 1 } else { 0 };
    println!("The value of num is: {}", num);




    // Loops


    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("The value of counter is: {}", counter);
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is: {}", result);


    let mut number = 100;
    while number != 0 {
        println!("The value of number is: {}", number);
        number /= 2;
    }


    let a = [10,20,30,40,50];
    for element in a {
        println!("The value of element is: {}", element);
    }

    for number in 1..10 {
        if number % 2 == 0 {
            continue;
        }
        println!("The value of number is: {}", number);
    }

    // Single line comment 
    /*
     * Multi-line comment
     * 
     * yo
     * yo
     * yo
     */
}



fn new_func(x: i32, y: i32) -> i32 {
    println!("This is something new. {}", x + y); // Statement ( just run, no returns )
    let result = x + y; // Expression ( returns a value )
    println!("The result is: {}", result);

    // return result; // Either return this way or
    // result // Or return this way Or
    x + y  // This way
}
