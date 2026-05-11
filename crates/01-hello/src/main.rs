// TODO: "Hello, world!" を出力してください
// ヒント: println! マクロを使います

// TODO: コメントの書き方を試してください
// - 行コメント (//)
// コメント
// - ブロックコメント (/* */)
/*
コメント
*/
// - ドキュメンテーションコメント (///)
/// コメント


fn main() {
    // ここにコードを書いてください
    // 出力
    println!("Hello World!");

    // 改行なし出力
    print!("改行なし");
    println!("続いて記述される");

    // フォーマット文字出力
    let name = "Name";
    println!("Hello {}!", name);

    // 複数の引数
    let x = 5;
    let y = 10;
    println!("x = {}, y = {}", x, y);

    // 位置引数
    println!("x: {0}, y: {1}, x: {0}", x, y);

    // 名前付き引数
    // ここでのnameと以前定義したnameは別物
    println!("{name}は{age}歳です", name = "太郎", age = 25);
    println!("{name}です");

    // デバッグ出力
    let arr = [1, 2, 3];
    println!("配列: {:?}", arr);

    // Prettyデバッグ出力
    let person = ("山田", 30);
    println!("人物: {:#?}", person);

    // 2,8,16進数
    println!("2進数: {:b}", 10);
    println!("8進数: {:o}", 10);
    println!("16進数: {:x}", 10);
    println!("16進数(大文字): {:X}", 10);

    // 幅寄せ出力
    println!("{:>5}", 42);
    println!("{:<5}", 42);
    println!("{:^5}", 42);
    println!("{:05}", 42);

    // エラー出力
    eprintln!("エラー: 何か問題が発生しました。")
}
