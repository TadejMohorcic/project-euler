pub fn solver() {
    let mut result = 0;

    for i in 1..1000000 {
        if is_palindrome(i) {
            if base2_palindrome(i) {
                result += i;
            }
        }
    }

    println!("Problem 0036 - Double-base Palindromes: {}", result)
}

fn base2_palindrome(number: u32) -> bool {
    let mut number_copy = number;
    let mut base2_digits = Vec::new();

    while number_copy > 0 {
        let d = number_copy % 2;
        number_copy /= 2;
        base2_digits.push(d);
    }

    let mut digits_iter = base2_digits.into_iter();
    while let (Some(front), Some(back)) = (digits_iter.next(), digits_iter.next_back()) {
        if front != back {
            return false;
        }
    }

    true
}

fn is_palindrome(number: u32) -> bool {
    let mut number_copy = number;

    let mut reversed_digits = Vec::new();

    while number_copy > 0 {
        let d = number_copy % 10;
        number_copy /= 10;
        reversed_digits.push(d);
    }

    let reversed_number = reversed_digits.iter().fold(0, |acc, x| 10 * acc + x);

    number == reversed_number
}
