fn _fib_naive(n: u8) -> u128 {
    if n <= 1 {
        n as u128
    } else {
        _fib_naive(n - 1) + _fib_naive(n - 2)
    }
}

fn _fib_linear(n: u8) -> u128 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        (a, b) = (b, a + b);
    }

    a
}

type Vector = [u128; 2];
type Matrix = [Vector; 2];

const F: Matrix = [[1, 1], [1, 0]];
const X: Vector = [1, 0];

pub fn fib(n: u8) -> u128 {
    let f_pow_n = matrix_power(F, n);
    let [_, ans] = apply(f_pow_n, X);
    ans
}

const ID: Matrix = [[1, 0], [0, 1]];

fn matrix_power(m: Matrix, n: u8) -> Matrix {
    // powers[i] := m^(2^i)
    let mut powers = [Default::default(); 8];
    powers[0] = m;
    for i in 1..8 {
        powers[i] = square(powers[i - 1]);
    }

    let mut product = ID;
    for i in 0..8 {
        if (n & 1 << i) != 0 {
            product = matrix_mul(product, powers[i]);
        }
    }
    product
}

fn square(m: Matrix) -> Matrix {
    matrix_mul(m, m)
}

fn matrix_mul(m1: Matrix, m2: Matrix) -> Matrix {
    let [[a, b], [c, d]] = m1;
    let [[e, f], [g, h]] = m2;
    [
        [a * e + b * g, a * f + b * h],
        [c * e + d * g, c * f + d * h],
    ]
}

fn apply(m: Matrix, x: Vector) -> Vector {
    let [[a, b], [c, d]] = m;
    let [x, y] = x;
    [a * x + b * y, c * x + d * y]
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(0, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 1)]
    #[test_case(3, 2)]
    #[test_case(4, 3)]
    #[test_case(5, 5)]
    #[test_case(6, 8)]
    #[test_case(7, 13)]
    #[test_case(8, 21)]
    #[test_case(9, 34)]
    #[test_case(50, 12_586_269_025)]
    fn fib(n: u8, expected: u128) {
        let actual = super::fib(n);
        assert_eq!(expected, actual);
    }

    #[test_case(ID, 0, ID)]
    #[test_case(ID, 1, ID)]
    #[test_case(ID, 2, ID)]
    #[test_case(F, 2, [[2, 1], [1, 1]])]
    fn matrix_power(m: Matrix, pow: u8, expected: Matrix) {
        let actual = super::matrix_power(m, pow);
        assert_eq!(expected, actual);
    }

    #[test_case(ID, ID, ID)]
    #[test_case(F, ID, F)]
    #[test_case(ID, F, F)]
    #[test_case(F, F, [[2, 1], [1, 1]])]
    fn matrix_mul(m1: Matrix, m2: Matrix, expected: Matrix) {
        let actual = super::matrix_mul(m1, m2);
        assert_eq!(expected, actual);
    }
}
