use std::cmp;

fn main() {
    fn karatsuba(x: u64, y: u64) -> u64 {
        if x.to_string().len() == 1 || y.to_string().len() == 1 {
            return x * y;
        }
        let n_digits = cmp::max(x.to_string().len(), y.to_string().len());
        let m = (n_digits / 2) as u32;
        let a = x / 10u64.pow(m);
        let b = x % 10u64.pow(m);
        let c = y / 10u64.pow(m);
        let d = y % 10u64.pow(m);
        let ac = karatsuba(a, c);
        let bd = karatsuba(b, d);
        let ad_bc = karatsuba(a+b, c+d) - ac - bd;
        return ac * 10u64.pow(2*m) + ad_bc * 10u64.pow(m) + bd;
    }
    println!("{}", karatsuba(1234, 5678));
}