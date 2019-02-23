fn main() {
    let t_in_c = 0;
    let t_in_f = convert_c_to_f(t_in_c);

    println!("If temperature in C is {}, temperature in F is {}", t_in_c, t_in_f);

    let t_in_f = 32;
    let t_in_c = convert_f_to_c(t_in_f);

    println!("If a temperature in F is {}, temperature in C is {}", t_in_f, t_in_c);
    let n:usize = 12;
    let f:usize = calculate_fibonnaci(n);

    println!("The fibonnaci of n = {} is {}", n, f);

    sing_twelve_days_o_xmas();
}

fn convert_c_to_f(t: i32) -> i32 {
    (t * 9/5) + 32
}

fn convert_f_to_c(t: i32) -> i32 {
    (t - 32) * 5/9
}

fn calculate_fibonnaci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut n_1:usize = 0;
        let mut n_2:usize = 1;
        let mut counter:usize = 1;
        let mut f:usize = 0;

        while counter < n {
            f = n_1 + n_2;
            n_1 = n_2;
            n_2 = f;
            counter = counter + 1;
        }
        f
    }
}

fn sing_twelve_days_o_xmas() {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "six",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let mut ind = 1;
    // const MAX:u8 = 11;

    for day in days.iter() {
        println!("On the {} day of Christmas my true love sent to me", day);
        for i in (0..ind).rev() {
            println!("{}", lyrics[i]);
        }
        ind = ind + 1;
    }

}