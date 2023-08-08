use std::array;

fn main() {
    println!("Hello, world!");
    let x = 10;
    let y = 20.8;
    let z = 130u8;
    let array: [i32; 3] = [10, 10, 10]; // array co kich thuoc co dinh, cac phan tu trong array cung kieu du lieu, chi doc ma khog¥ng thay doi gia tri
    println!("array 1: {}", array[0]);
    //println!("array 4: {}", array[3]);
    let tuple1: (i32, i32) = (10,10);
    let tuple1: (i32, i32, f64, &str) = (10,10, 10.1, "hello world");
    println!("tuple1 1: {}", tuple1.3);
    //println!("tuple1 4: {}", tuple1.4);
    // -2^
    let trage: u32; // it dung hon
    type  a = u32;  // hay dung hon


    // name covension
    let a = 10; // warning should use snake_case
    //a = a + 10; -- wrong becase cannot change value
    let mut b = 10; //them mut to change value of variable
    b = b + 30;
    println!("b : {}", b);


    // contans
    const PI: f64 = 3.14;
    const NUMBER_CASE: u32 = 234;


    // string
    let mut s = String::new();
    println!("string is empty:{}", s.is_empty());
    s.push('c');
    println!("string is empty:{}", s);

    let mut m = String::from("Hello worlf¥d");
    println!("string is :{}", m);
    

    // reference string -- chi co quyen doc
    let s2 = "Helloe World";
    //let mut s2: &str = "Helloe World";
    // s2 = s2 + 'g'; --> sai vi khong co quyen ghi
    m = m + s2;
    println!("string m :{}", m);

    println!("result i :{}", &s2[0..5]);

    // convert &str --> string
    let conversion_string = "Duong".to_string();
    let conversio_str = &conversion_string;
    let cvt_str = conversion_string.as_str();
    let byte = conversion_string.as_bytes();
    

    // condiftion
    let state = true;
    if state {
        println!("Hello");
    } else {
        println!("Byte");
    }

    match state {
        true => println!("Hello"),
        false => println!("Byte")
    }

    // default match
    let number_ = 10;
    match number_ {
        10 => println!("value = 10"),
        _ => println!("value !10")
    }

    // tuple
    let tuple1 = (10, 10.2);
    match tuple1 {
        (10, 10.2) | (9, 9.0) => {
            match number_ {
                10 => println!("value = 10"),
                _ => println!("value !10")
            }
        },
        _ => todo!()
    }

    // for
    let vec = vec![1, 2, 3, 4, 5, 6];
    for value in vec.iter(){
        println!("vector value: {} ", value);
    }

    // for val in vec {
    //     println!("vector value: {} ", val);
    // }

    let max = vec.iter().max().unwrap();
    print!("Max value: {} ", max);


    function_x();
    function_y(String::from("value"));
    let f = function_z(String::from("return value"));
    println!("this is funtion z: {}", f);
}
 fn function_x(){
    println!("this is function x");
 }

 fn function_y(s:String){
    println!("this is functio y: {}" , s);
 }

 fn function_z(s:String)-> String {
    s
 }