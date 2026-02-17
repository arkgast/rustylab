use calculator::Calculator;

fn main() {
    let a = 400u32;
    let b = 200u32;

    let calculator = Calculator::new(a, b);

    println!("{}", calculator);
}
