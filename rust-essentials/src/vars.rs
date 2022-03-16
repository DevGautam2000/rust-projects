pub fn run() {
    //let x = 5; //--- gives error as the variableis immutable by default
    let mut x =5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //assign multiple vars
    let (a,b) = (1,2);
    print!("{} x {} is {}",a,b,a*b);
}
