mod jack_value;
use jack_value::JackValue;

fn test_binop(a: JackValue, b: JackValue) {
    println!("- {} = {}", a, -a);
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} \\ {} = {}", a, b, JackValue::idiv(a, b));
    println!("{} % {} = {}", a, b, a % b);
}

fn main() {

    test_binop(JackValue::Integer(13), JackValue::Integer(5));
    test_binop(JackValue::Integer(42), JackValue::new_rational(13, 25));
    test_binop(JackValue::new_rational(3, 4), JackValue::Integer(7));
    test_binop(JackValue::new_rational(7, 12), JackValue::new_rational(4, 12));
    test_binop(JackValue::new_rational(1, 2), JackValue::new_rational(3, 2));
    test_binop(JackValue::new_rational(3, 2), JackValue::new_rational(1, 2));
    test_binop(JackValue::new_rational(3, 2), JackValue::Integer(2));
    test_binop(JackValue::new_rational(5, 2), JackValue::Integer(2));

    // 1/2 / 3/2 -> 3 r 0
    // 3/2 / 2 -> 1 r 1/2

    // println!("{}",  );
    // println!("{}", JackValue::new_rational(7, 12) - JackValue::new_rational(4, 12) );
    // println!("{}", JackValue::new_rational(7, 12) * JackValue::new_rational(4, 12) );
    // println!("{}", JackValue::new_rational(7, 12) / JackValue::new_rational(4, 12) );
    // println!("{}", &num3 - &num5 );
    // println!("{}", &num1 / &num2 );
    // println!("{}", &(&num2 + &num2) + &num4 );

}
