// 4.1 所有権とは？
// ここに学んだ「動く最小コード」を毎日追記していく。
// NG例（move後使用など）はコメントアウトし、すぐ下にコンパイラのエラー要旨を1行残す。
// 例:
//   let s2 = s1; println!("{s1}");
//   ^ error[E0382]: borrow of moved value: `s1`（s1 は s2 へ move 済み）

fn main() {
    println!("ch04 4.1 所有権とは？ — ここから書き始める");

    // これはリテラル
    let mut literal = "hello";

    // literal.push_str(" world"); エラー！

    literal = "world"; // OK
    // String 型は、コンパイル後（実行時）でもヒープ上のデータを変更できる。
    // 一方で文字列リテラルは、コンパイル時に読み取り専用領域に固定され、変更できない。
    // "hello" → 読み取り専用領域（コンパイル時に配置済み）
    // "world" → 読み取り専用領域（コンパイル時に配置済み）
    // s → どちらかを指すだけ

    println!("{}", literal);

    // これは文字列
    let mut s = String::from("hello");

    s.push_str(" world!");

    println!("{}", s);

    // https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html#%E6%89%80%E6%9C%89%E6%A8%A9%E3%81%A8%E9%96%A2%E6%95%B0
    // 所有権と関数

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
