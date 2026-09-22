fn main() {
    let letters = ['d', 'e', 'f'];
    let mut x = 10;
    let z = false;
    let a = 4;
    let b=5;
    let c = 6;
    println!("The value of x is: {}", x);
       x = 5;
      println!("The value of x is: {}, however that is {}.", x, z);
    println!("The average of a, b and c is: {}", (a + b + c) / 3);
    println!("The first letter in my array is: {}", letters[0]);

    if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is not equal to 5");
    }
    fn say_hello(){
            println!("Hello, world!");
    }
    say_hello();
    for number in 0..5{
        println!("This program will be over in {}", number);
    }
}
