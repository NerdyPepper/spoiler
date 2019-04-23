// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By
// starting with 1 and 2, the first 10 terms will be:
// 
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find
// the sum of the even-valued terms

fn main() {
    let mut x1 = 1.;
    let mut x2 = 2.;
    let mut sum = 2.;

    loop {
        if x1 > 4e6 { break; }

        x1 += x2;
        x2 += x1;

        if x1 % 2. == 0. { sum += x1; }
        else if x2 % 2. == 0. { sum += x2; }
    }
    println!("{}", sum);
}
