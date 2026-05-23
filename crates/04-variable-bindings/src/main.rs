// TODO: 不変変数と可変変数の違いを確認してください
// ヒント: let x = 5; → 再代入不可
// ヒント: let mut y = 5; y = 6; → 再代入可能

// TODO: シャドウイングを試してください
// ヒント: let x = 5; let x = x + 1; let x = "文字列"; // 型も変わる

// TODO: スコープを確認してください
// ヒント: ブロック {} の内側と外側で変数の有効範囲を確認

// TODO: 凍結（freezing）を確認してください
// ヒント: let mut x = 10; { let x = x; /* ここでは不変 */ }

fn main() {
    // ここにコードを書いてください
    // 不変変数
    let x = 5;

    // 可変関数
    let mut y = 5;
    println!("y = {}", y);
    y = 6;
    println!("y = {}", y);

    // スコープ
    let outer = 1;
    {
        // 内側のブロック開始
        let inner = 2;
        println!("outer = {}, inner = {}", outer, inner);
        // 両方アクセス可能
    }

    // println!("inner = {}", inner); // エラー!アクセス不可
    println!("outer = {}", outer);

    // シャドウィング
    let x = 5;
    println!("x = {}", x);

    // 同じ名前で新しい変数を定義
    let x = x + 1;
    println!("x = {}", x);

    // 型を変更して定義できる
    let x = "文字列";
    println!("x = {}", x);

    // シャドウイングのユースケース
    // 文字列を数値に変換して同じ名前で使う
    let input = "42";
    let input: i32 = input.parse().unwrap();
    println!("数値: {}", input);

    // 凍結
    let mut x = 10;
    {
        // 可変変数xを不変変数でシャドウイング
        let x = x;

        println!("凍結された x = {}", x);
    }
    // 外側のスコープでは x は再び可変
    x = 20;
    println!("可変な x = {}", x);

    // mut: 同じ型で値を繰り返し変更したい場合に使う
    let mut count = 0;
    for _ in 0..5 {
        count += 1;
    }
    println!("count = {}", count);
}
