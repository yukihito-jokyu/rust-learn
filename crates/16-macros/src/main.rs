// 16 - マクロ（Macros）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

// TODO: 基本的なマクロを定義してください
// 引数なしで"Hello, Macros!"と出力するマクロ

// TODO: 引数を取るマクロを定義してください
// 式を受け取って、その式と結果を表示するマクロ

// TODO: 指定子を使ったマクロを定義してください
// ident指定子を使って関数を動的に生成するマクロ

// TODO: オーバーロードマクロを定義してください
// 引数1つ: 値を表示
// 引数2つ: 値を比較

// TODO: 繰り返しマクロを定義してください
// 可変長引数を受け取ってVecを作成するマクロ

// TODO: DRYマクロを定義してください
// 複数の型に対して同じトレイト実装を生成するマクロ

macro_rules! test {
    // 引数なし
    ($left:expr) => {
        println!("値: {:?}", $left);
    };
    // 2つの引数を比較
    ($left:expr, $right:expr) => {
        println!("{:?} と {:?} を比較: {}", $left, $right, $left == $right);
    };
    // 条件付きテスト
    ($left:expr, $right:expr, $should_eq:expr) => {
        if $should_eq {
            test!($left, $right);
        } else {
            println!("比較をスキップ: {:?}, {:?}", $left, $right);
        }
    };
}

fn main() {
    println!("=== 基本的なマクロ ===");

    // TODO: 引数なしマクロを呼び出してください

    println!("\n=== 引数付きマクロ ===");

    // TODO: 式を渡してマクロを呼び出してください

    println!("\n=== 指定子の使用 ===");

    // TODO: ident指定子のマクロを使って関数を生成してください

    println!("\n=== オーバーロード ===");

    // TODO: 引数1つと引数2つのパターンを試してください

    test!(1); // 値: 1
    test!(1, 1); // 1 と 1 を比較: true
    test!(1, 2); // 1 と 2 を比較: false
    test!(1 + 1, 2, true); // 2 と 2 を比較: true
    test!(1 + 1, 2, false); // 比較をスキップ: 2, 2

    println!("\n=== 繰り返しマクロ ===");

    // TODO: 可変長引数マクロを使ってVecを作成してください

    println!("\n=== DRYマクロ ===");

    // TODO: DRYマクロを使って複数の型に実装を提供してください
}
