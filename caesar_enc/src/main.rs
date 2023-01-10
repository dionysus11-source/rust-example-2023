fn main() {
    println!("Hello, world!");
    let enc = encrypt("I LOVE RUST", 3);
    let dec = encrypt(&enc, -3);
    println!("{} -> {}",dec, enc);
    let enc_c = encrypt_closure("I LOVE RUST", 3);
    let dec_c= encrypt_closure(&enc_c, -3);
    println!("{} -> {}",dec_c, enc_c);
}

fn encrypt_closure(text: &str, shift:i16) -> String{
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c<='Z';
    let conv = |c| (((c-a+shift+26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c){conv(c as i16)} else{c};
    text.chars().map(|c| enc1(c)).collect()
}

fn encrypt(text: &str, shift:i16) -> String{
    let mut result = String::new();
    let code_a  = 'A' as i16;
    let code_z = 'Z' as i16;

    for ch in text.chars(){
        let mut code = ch as i16;
        if code_a <= code && code <= code_z{
            code = (code + shift - code_a + 26) % 26 + code_a;
        }
        result.push((code as u8)as char);

    }
    return result;
}