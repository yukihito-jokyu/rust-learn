// 21 - Unsafe Rust
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

fn main() {
    println!("=== unsafeブロックの使用 ===");

    // TODO: unsafeブロックを作成し、その中で操作を行ってください

    println!("\n=== rawポインタの操作 ===");

    // TODO: 参照から生ポインタを作成してください
    // let mut num = 42;
    // let ptr = &mut num as *mut i32;

    // TODO: unsafeブロック内で生ポインタをデリファレンスしてください

    // TODO: 配列のポインタを使って要素にアクセスしてください
    // ヒント: as_ptr()とptr.add()を使う

    println!("\n=== 可変static変数 ===");

    // TODO: static mut 変数を宣言してください（関数の外で）
    // static mut COUNTER: u32 = 0;

    // TODO: unsafeブロック内で可変static変数を読み書きしてください

    // TODO: 関数を作成して可変static変数を変更してください

    println!("\n=== unsafe関数 ===");

    // TODO: unsafe関数を定義し、呼び出してください
    // unsafe fn dangerous_function() { ... }

    // TODO: 安全なラッパー関数を作成してください

    println!("\n=== extern（FFI）===");

    // TODO: extern "C"ブロックでC標準ライブラリ関数を宣言してください
    // extern "C" {
    //     fn abs(input: i32) -> i32;
    // }

    // TODO: unsafeブロック内で外部関数を呼び出してください
}

// TODO: static mut変数をここに宣言してください
// static mut COUNTER: u32 = 0;

// TODO: unsafe関数をここに定義してください
// unsafe fn dangerous_function() {
//     println!("危険な操作！");
// }
