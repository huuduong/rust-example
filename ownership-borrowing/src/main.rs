fn main() {
    println!("Hello, world!");
    // su dung closure
    let a = 12;
    let x = |y: &str| -> String{
        println!("hello function a");
        return  y.to_string();
    };
    let res = x("DuongNH");
    println!("Res: {}", res);

    // for , iter, info_iter, iter_mut
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    for value in vec1 {
        println!("Value : {}", value);
    }

    let vec2 = vec![10, 20, 30, 40, 50];
    vec2.iter().enumerate().for_each(|x|{
        println!("Index 0: {}, Index 1: {}", x.0, x.1);
    });

    let vec3 = vec![100, 200, 300, 400];
    let res:Vec<i32> = vec3.iter().map(|x| x * 2).collect();
    println!("res :{:?}", res);
}


fn function_a(){

}