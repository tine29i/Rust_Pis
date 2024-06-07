pub fn scytale_cipher(message: String, i: u32) -> String {
    let chars: Vec<char> = message.chars().collect();
    let mut cipher = String::new();

    for j in 0..i {
        let mut k = j;
        while (k as usize) < chars.len() {
            cipher.push(chars[k as usize]);
            if ((k + i) % i == 0) && ((k as usize) + 1 < chars.len()) {
                cipher.push(' ');
            }
            k += i;
        }
    }

    cipher
}