// TODO: if / else の動作を確認してください
// ヒント: if condition { ... } else if ... { ... } else { ... }

// TODO: loop と break / continue を試してください
// ヒント: loop { ... break; }
// ヒント: loopは値を返せる: let x = loop { break 42; };

// TODO: while ループを試してください
// ヒント: while condition { ... }

// TODO: for イテレータを試してください
// ヒント: for item in collection { ... }
// ヒント: for i in 0..10 { ... }
// ヒント: .iter(), .enumerate(), .rev()

// TODO: match パターンマッチングを試してください
// ヒント: match value { pattern => expr, _ => expr, }
// ヒント: ガード: pattern if condition => expr

// TODO: if let を使ってみてください
// ヒント: if let Some(x) = option { ... } else { ... }

fn main() {
    // ここにコードを書いてください
    // loop
    // loopは無限ループを作る
    // breakで脱出、continueで次のイテレーションへ進む
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            println!("countが5になったので終了");
            break;
        }
    }
    println!("count = {}", count);

    // loopは値を返せる
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);

    // ネストされたloopとラベル
    'outer: for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 {
                println!("外側のループを終了 ({}, {})", x, y);
                break 'outer;
            }
        }
    }

    // continueで次のイテレーションへ
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("奇数: {}", i);
    }

    // while
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("n = {}", n);

    // fot とイテレータ
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("要素: {}", element);
    }

    // イテレータメソッド
    let sum: i32 = arr.iter().sum();
    println!("合計: {}", sum);
    let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    println!("2倍: {:?}", doubled);
    let evens: Vec<&i32> = arr.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // タプルのマッチング
    let pair = (2, -2);
    match pair {
        (0, y) => println!("xは0、yは{}", y),
        (x, 0) => println!("xは{}、yは0", x),
        (x, y) if x == -y => println!("{}と{}は反転", x, y), // ガード
        _ => println!("マッチなし"),
    }

    // Enumのマッチング
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("赤"),
        Color::Green => println!("緑"),
        Color::Blue => println!("青"),
        Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    }

    // 参照のマッチング（refとref mut）
    let mut value = 10;
    match value {
        ref r => println!("参照: {}", r), // r は &i32
    }
    match value {
        ref mut m => {
            *m += 1;
            println!("可変参照: {}", m);
        },
    }

    // if let
    let option = Some(7);

    if let Some(i) = option {
        println!("値: {}", i)
    }

    // else付き
    let option: Option<i32> = None;
    if let Some(i) = option {
        println!("値: {}", i);
    } else {
        println!("値はありません");
    }
}
