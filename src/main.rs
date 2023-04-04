fn main() {
    loop {
        println!("Enter a 32-bit floating point value in hexadecimal format: ");
        let mut f = String::new();
        std::io::stdin().read_line(&mut f).expect("Invalid string");

        if f.len() < 8 {
            println!("Input must contain 8 hexadecimal characters");
        }
        else if let Some(f) = u32::from_str_radix(&f[..8], 16).ok() {
            let f_bits = format!("{:032b}", f);
            let decodef = |a: usize, b: usize| u32::from_str_radix(&f_bits[a..b], 2).ok().unwrap();
            let s = decodef(0, 1);
            let e = decodef(1, 9);
            let m = decodef(9, 32);
            println!("value:      {:.6e}", f32::from_bits(f));
            println!("raw binary: {f:032b}");
            println!("sign:       {:b} ({})", s, if s==0 {"positive"} else {"negative"});
            println!("exponent:   {:08b} ({})", e, e as i32 - 127);
            println!("mantissa:   1.{:023b}", m);
            break;
        } else {
            println!("Invalid hexadecimal string.");
        }
    }
}
