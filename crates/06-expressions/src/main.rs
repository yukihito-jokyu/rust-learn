// TODO: ブロック式の値を確認してください
// ヒント: let x = { let a = 1; let b = 2; a + b }; // x = 3
// ヒント: セミコロンあり/なしの違いを確認

// TODO: if式を変数に代入してください
// ヒント: let result = if condition { "真" } else { "偽" };
// ヒント: 両方のブランチの型を一致させる必要があります

fn main() {
    // ここにコードを書いてください
    // 文(値を返さない)
    let x = 5;

    // 式(値を返す)
    let y = 5 + 3;

    // ブロック{}も式
    let x = {
        let a = 1;
        let b = 2;
        a + b // 最後はセミコロンなし
    };
    println!("x = {}", x);

    // 初期化時に複雑な計算を行う
    let result = {
        let base = 100;
        let tax = 10;
        let total = base + tax;
        format!("合計: {}円（税込み）", total)
    };
    println!("{}", result);

    // if式
    // rustではifは文ではなく、式
    let number = 5;
    if number > 3 {
        println!("{} は3より大きい", number);
    }

    // if-else式
    let x = 10;
    if x % 2 == 0 {
        println!("{} は偶数", x);
    } else {
        println!("{} は奇数", x);
    }

    // if式に変数を代入
    let condition = true;
    let result = if condition { "条件は真です" } else { "条件は偽です" };
    println!("結果: {}", result);
}
