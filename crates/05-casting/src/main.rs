// TODO: 数値型のキャスト（as）を試してください
// ヒント: let x: i32 = 42; let y: u8 = x as u8;
// ヒント: f64 -> i32（小数の切り捨て）も試してください

// TODO: リテラルの型注釈を試してください
// ヒント: let a = 42i32; let b = 3.14f32;
// ヒント: 10進数、16進数(0x)、8進数(0o)、2進数(0b)を試してください

// TODO: 型推論の動作を確認してください
// ヒント: let x = 5; // i32が推論される
// ヒント: let mut v = Vec::new(); v.push(42); // Vec<i32>が推論される

// TODO: typeエイリアスを使ってみてください
// ヒント: type Age = u32;
// ヒント: type IntPair = (i32, i32);

fn main() {
    // ここにコードを書いてください
    // 整数型間のキャスト
    let a: i32 = 42;
    let b: u8 = a as u8;
    println!("i32 -> u8: {} -> {}", a, b);

    // 丸めに注意
    let c: i32 = 300;
    let d: u8 = c as u8;
    println!("i32 -> u8: {} -> {}", c, d); // 300 -> 44 オーバーフロー

    // 符号付き -> 符号なし
    let e: i8 = -1;
    let f: u8 = e as u8;
    println!("i8 -> u8: {} -> {}", e, f);

    // 浮動小数点数 -> 整数(少数部分は切り捨て)
    let g: f64 = 3.99;
    let h: i32 = g as i32;
    println!("f64 -> i32: {} -> {}", g, h);

    // char -> u8
    let k: char = 'A';
    let l: u8 = k as u8;
    println!("char -> u8: {} -> {}", k, l);

    // u8 -> char
    let m: u8 = 65;
    let n: char = m as char;
    println!("u8 -> char: {} -> '{}'", m, n);

    // 型推論
    let mut v = Vec::new();
    v.push(42);
    println!("{:?}", v);
}
