trait Programmer {
    fn write_code(&self);
    fn rest(&self);
}

struct CppProgrammer {}

impl Programmer for CppProgrammer {
    fn write_code(&self) {
        println!("template<typename... Writing Some verbose code...");
    }

    fn rest(&self) {
        println!("Drinking tones of coffe...");
    }
}

struct RustProgrammer {}

impl Programmer for RustProgrammer {
    fn write_code(&self) {
        println!("Writing a trait... All done!");
    }

    fn rest(&self) {
        println!("Just chilling!")
    }
}

fn live_a_day_of(programmer: &dyn Programmer) {
    println!("# Some programmer's day:");
    programmer.write_code();
    programmer.rest();
    programmer.write_code();
    println!("Guess, who's the programmer?");
    println!();
}

fn main() {
    let cpp_programmer = CppProgrammer{};
    let rust_programmer = RustProgrammer{};

    live_a_day_of(&cpp_programmer);
    live_a_day_of(&rust_programmer);

    // let programmers: Vec<&dyn Programmer> = vec![&cpp_programmer, &rust_programmer];

    // for programmer in programmers {
    //     live_a_day_of(programmer);
    // }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_programmers() {
        let cpp_programmer = CppProgrammer{};
        let rust_programmer = RustProgrammer{};
        
        let programmers: Vec<&dyn Programmer> = vec![&cpp_programmer, &rust_programmer];
        assert_eq!(programmers.len(), 2);
    }
}