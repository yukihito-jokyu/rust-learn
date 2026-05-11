// TODO: 整数型の変数を宣言して演算を試してください
// ヒント: i8, i16, i32, i64, u8, u16, u32, u64, usize, isize

// TODO: 浮動小数点型の変数を宣言して演算を試してください
// ヒント: f32, f64（デフォルト）

// TODO: 真偽型（bool）の変数を宣言して論理演算を試してください
// ヒント: true, false, &&, ||, !

// TODO: 文字型（char）の変数を宣言してください
// ヒント: 'a', '🦀', '\u{1F600}'

// TODO: タプルを作成してアクセスしてみてください
// ヒント: let tuple = (1, 2.0, 'a');
// ヒント: tuple.0 でアクセス、let (x, y, z) = tuple; で分解

// TODO: 配列を作成してアクセスしてみてください
// ヒント: let arr = [1, 2, 3, 4, 5];
// ヒント: arr[0] でアクセス、arr.len() で長さ取得

fn main() {
    // ここにコードを書いてください
    // 符号あり
    let a: i8 = -128;
    let b: i16 = 32767;
    let c: i32 = 2_147_483_647;
    let d: i64 = 123_456_789;
    let e: i128 = 123_456_789_012_345;

    // 符号なし
    let f: u8 = 255;
    let g: u16 = 65535;
    let h: u32 = 4_294_967_295;
    let i: u64 = 123_456_789;
    let j: u128 = 123_456_789_012_345;

    // 型サフィックスによる指定
    let k = 42i32;
    let l = 42u8;

    // アーキテクチャ依存のサイズ
    let m: isize = -1;
    let n: usize = 1;

    println!("a = {}, f = {}, m = {}, n = {}", a, f, m, n);

    // 浮動小数点型
    let x: f32 = 3.5;
    let y: f64 = 2.728;

    // 演算
    let sum = x as f64 + y;
    let diff = y - x as f64;
    let product = x as f64 * y;
    let quotient = y / x as f64;
    let remainder = y % x as f64;

    println!("x = {}, y = {}", x, y);
    println!("和 = {}, 差 = {}", sum, diff);
    println!("積 = {}, 商 = {}", product, quotient);
    println!("余り = {}", remainder);

    // 真偽型
    let t: bool = true;
    let f: bool = false;

    // 演算
    println!("t && f = {}", t && f);
    println!("t || f = {}", t || f);
    println!("!t= {}", !t);

    // 文字型
    let c: char = 'z';
    let z: char = '\u{1F600}';
    println!("c = {}, z = {}", c, z);

    // タプル
    let tuple: (i32, f64, char) = (500, 6.4, 'a');

    // インデックスによるアクセス
    println!("一番目: {}", tuple.0);
    println!("二番目: {}", tuple.1);
    println!("三番目: {}", tuple.2);

    // 分解(デストラクト)
    let (x, y, z) = tuple;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // ネストされたタプル
    let nested = ((1, 2), (3, 4));
    println!("nested.0.1 = {}", nested.0.1);

    // タプルのデバッグ出力
    println!("tuple = {:?}", tuple);
    println!("nested = {:#?}", nested);

    // 配列
    // 配列の作成
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // 全要素を同じ値で初期化
    let zeros = [0; 10];

    // インデックスによるアクセス
    println!("arr[0] = {}", arr[0]);
    println!("arr[4] = {}", arr[4]);

    println!("zeros = {:?}", zeros);

    // 配列の長さ
    println!("len = {}", arr.len());

    // 配列はスタックに確保されるため、変更不可
    // ただし、mutなら要素自体は変更可能
    let mut mutable_arr = [1, 2, 3];
    mutable_arr[0] = 100;
    println!("mutable_arr = {:?}", mutable_arr);
}
