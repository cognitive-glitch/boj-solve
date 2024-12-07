/// Fast I/O macros for competitive programming
/// These macros provide efficient input/output operations optimized for competitive programming

/// Reads a single line from stdin
/// # Examples
/// ```
/// let line = read!(); // reads a line as String
/// let num = read!(i32); // reads and parses a single number
/// let (a, b) = read!(i32, i32); // reads two numbers into a tuple
/// let (x, y, z) = read!(i32, i32, i32); // reads three numbers into a tuple
/// ```
#[macro_export]
macro_rules! read {
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().to_string()
    }};
    ($t:ty) => {{
        let input = read!();
        input
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse::<$t>()
            .unwrap()
    }};
    ($t1:ty, $t2:ty) => {{
        let line = read!();
        let v: Vec<String> = line
            .split_ascii_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        (v[0].parse::<$t1>().unwrap(), v[1].parse::<$t2>().unwrap())
    }};
    ($t1:ty, $t2:ty, $t3:ty) => {{
        let line = read!();
        let v: Vec<String> = line
            .split_ascii_whitespace()
            .map(ToOwned::to_owned)
            .collect();
        (
            v[0].parse::<$t1>().unwrap(),
            v[1].parse::<$t2>().unwrap(),
            v[2].parse::<$t3>().unwrap(),
        )
    }};
}

/// Reads a vector of values from a single line
/// # Examples
/// ```
/// let vec = readv!(i32); // reads all numbers from line into Vec<i32>
/// let vec_n = readv!(i32, 5); // reads exactly 5 numbers from line
/// ```
#[macro_export]
macro_rules! readv {
    ($t:ty) => {{
        read!()
            .split_ascii_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<_>>()
    }};
    ($t:ty, $n:expr) => {{
        read!()
            .split_ascii_whitespace()
            .take($n)
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<_>>()
    }};
}

/// Reads a matrix (2D vector) of size r×c
/// # Examples
/// ```
/// let matrix = readm!(i32, 3, 4); // reads 3×4 matrix of i32
/// ```
#[macro_export]
macro_rules! readm {
    ($t:ty, $r:expr, $c:expr) => {{
        (0..$r).map(|_| readv!($t, $c)).collect::<Vec<_>>()
    }};
}

#[allow(unused)]
#[test]
fn test_io() {
    let x: i32 = read!(i32);
    let vec: Vec<i64> = readv!(i64);
    let n = read!(usize);
    let (n, m) = read!(usize, usize);
    let vec_n: Vec<i32> = readv!(i32, n);
    let (r, c) = (read!(usize), read!(usize));
    let matrix: Vec<Vec<i32>> = readm!(i32, r, c);
}

fn main() {
    let s = read!();
    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut ans = vec![0; 10];
    s.chars()
        .for_each(|c| ans[numbers.iter().position(|&x| x == c).unwrap()] += 1);

    let t: i32 = (ans[6] + ans[9] + 1) / 2 as i32;
    ans[6] = 0;
    ans[9] = 0;
    let max = std::cmp::max(ans.iter().max().unwrap(), &t);
    println!("{}", max);
}
