fn call_me(){
    println!("hello");
}

fn call_me_1(num:i32)->() {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me();
    call_me_1(3);
    call_me_2(5);let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

fn sale_price(price: i32) -> i32{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}



fn call_me_2(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

