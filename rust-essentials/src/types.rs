//integer - unsigned and signed - defualt:i32
//float - f32 and f64 - defualt:f64
//boolean - bool
//Character - char
//Arrays,Vectors
//Tuples

pub fn run(){

    let integer = 21;
    let float = 1.3;
    let boolean = false;
    let character = '1';
    let tuples = (1,3.2,[2],"anything");
    let arr1:[i32;2] = [2,3];
    let arr:Vec<i32> = vec![1,2,3];

    println!("{:?}",(integer,float,boolean,character,tuples,arr1,arr));
}