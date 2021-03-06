use ferris_says::say;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("æ¸¸æç»æð®");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    // çæä¸ä¸ª1-100éæºå¼
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("ç§å¯æ°å­å·²çæ, è¯·å¨1-100ä¸­çä¸ä¸ªæ°å­ï¼è¾å¥éæ³å­ç¬¦ä¼èªå¨ç»æ");
    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("ä½ ççæ°å­æ¯: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("ä½ çå¯¹äº");
                break;
            },
            Ordering::Greater => println!("ä½ ççæç¹å¤§ohï½ï¼åè¯è¯å§"),
            Ordering::Less => println!("ä½ ççæç¹å°ohï½ï¼åè¯è¯å§"),
        }
    }
}
