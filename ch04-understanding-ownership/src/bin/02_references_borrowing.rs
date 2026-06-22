// 4.2 参照と借用
// &T 不変参照 / &mut T 可変参照 / 借用規則 / ダングリング参照 を動かして確かめる。
// NG例はコメントアウト＋エラー注記で残す。

fn main() {
    // 参照と借用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可変参照
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
