fn main() {
    let mut l = std::io::stdin().lines().take(2);
    let (f, s) = (l.next().unwrap(), l.next().unwrap());

    let n: u64 = f.unwrap().parse().unwrap();
    let sum: u64 = s
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .sum();

    let res = n * (n + 1) / 2 - sum;

    println!("{res}")
}
