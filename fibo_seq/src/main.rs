fn main() {
    println!("Hello, world!");
    for n in 1..10 {
        println!("---{}",fib(n))
    }
}


//F(1)=1，F(2)=1, F(n)=F(n-1)+F(n-2)（n>=3，n∈N*）


fn fib(n:i32) -> i32{

    if n==1 {
        1
    }
    else if n == 2{
        1
    } else {
        fib(n-2) + fib(n-1)
    }

        

}