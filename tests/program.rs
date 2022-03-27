use num_bigint::BigInt;

#[test]
fn tests() {
    let big_int: BigInt = "1".parse().unwrap();

    let mut c = big_int.clone();
    c = "3".parse().unwrap();

    println!("{}", big_int);
}
