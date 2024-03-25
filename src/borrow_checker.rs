fn consume_string(to_be_consumed: String) {
    println!("string consumed = {}", to_be_consumed);
}

fn consume_int(to_be_consumed: i32) {
    println!("int consumed = {}", to_be_consumed);
}

fn borrow_string(shared: &String) {
    println!("string borrowed = {}", shared);
}

fn borrow_int(shared: &i32) {
    println!("int borrowed = {}", shared);
}

fn main() {
    { // Consuming with a variable
        let some_string = String::from("some_value");
        let string_consumer = some_string;

        // println!("some_string = {}", some_string);
        println!("consumer = {}", string_consumer);
    }

    { // Consuming with a function
        let some_string = String::from("some_value");
        consume_string(some_string);
        // println!("some_string = {}", some_string);
    }

    { // Copying instead of consuming
        let some_int = 10;

        let int_consumer = some_int;
        consume_int(some_int);

        println!("some_int = {}", some_int);
        println!("consumer = {}", int_consumer);
    }

    { // Borrowing string
        let shared_string = String::from("shared_value");
        let string_borrowed = &shared_string;
        borrow_string(&shared_string);

        println!("shared_string = {}", shared_string);
        println!("string_borrowed = {}", string_borrowed);
    }

    { // Borrowing int
        let shared_int = 123;
        let string_borrowed = &shared_int;
        borrow_int(&shared_int);

        println!("shared_int = {}", shared_int);
        println!("string_borrowed = {}", string_borrowed);
    }

    { // Borrowing Rules:
      // 1. At any given time, you can have either one mutable reference or any number of immutable references. 
      // 2. References must always be valid.

        let mut shared = 10;

        let borrowed1 = &mut shared;
        // let borrowed2 = &mut shared;

        // borrow_int(&shared);
        borrow_int(borrowed1);
        // borrow_int(borrowed2);
    }
}
