// 4.3 スライス型
// &str / &[T] は「一部への参照」。first_word 例・配列スライスを動かして確かめる。
// NG例はコメントアウト＋エラー注記で残す。

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("{:p}", &s);
    println!("{:p}", s.as_ptr());

    let slice = &s[0..2];
    let slice = &s[..2];

    s.clear();

    println!("the first word is: {}", word);

    // 引数としての文字列スライス
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> usize /* &str */ {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
            // return &s[0..1];
        }
    }

    s.len()
    // &s[..]
}
