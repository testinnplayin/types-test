fn main() {
    let t_in_c = 0;
    let t_in_f = convert_c_to_f(t_in_c);

    println!("If temperature in C is {}, temperature in F is {}", t_in_c, t_in_f);

    let t_in_f = 32;
    let t_in_c = convert_f_to_c(t_in_f);

    println!("If a temperature in F is {}, temperature in C is {}", t_in_f, t_in_c);
}

fn convert_c_to_f(t: i32) -> i32 {
    (t * 9/5) + 32
}

fn convert_f_to_c(t: i32) -> i32 {
    (t - 32) * 5/9
}
