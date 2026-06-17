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
}
