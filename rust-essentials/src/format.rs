
    pub fn run(){

        //names arguments
        print!("{name} is {age} years old",name="Gautam",age="21");

        //positional arguments
        println!("{0} likes music and {0} likes instruments.","Gautam");

        //placeholder traits
        println!("Binary: {:b} , Hex: {:x}, Octal: {:o}, Decimal: {}",10,10,10,10);

        //placeholder debug trait
        println!("{:?}", (21,"Men in black",100))
    }