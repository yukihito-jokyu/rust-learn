// TODO: 2つの整数を足算する関数 `add` を定義してください
// ヒント: fn add(a: i32, b: i32) -> i32 { ... }

// TODO: 引数の絶対値を返す関数 `absolute` を定義してください
// ヒント: if文とreturnを使います

// TODO: タプルを返す関数 `swap` を定義してください
// ヒント: fn swap(a: i32, b: i32) -> (i32, i32) { ... }

// TODO: クロージャを作成してください
// - 環境の変数をキャプチャするクロージャ
// - 複数行のクロージャ
// ヒント: |引数| 式 の構文を使います

// TODO: 高階関数を使って vec![1, 2, 3, 4, 5] を操作してください
// - map で各要素を2倍
// - filter で偶数だけ抽出
// - fold で合計を計算
// ヒント: .iter().map(|x| ...).collect()

// TODO: 発散する関数 `die` を定義してください
// ヒント: fn die(message: &str) -> ! { panic!(...) }

// クロージャを引数に取る関数
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 関数の定義
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // ここにコードを書いてください
    let result = add(3, 4);
    println!("3 + 4 = {}", result);

    // クロージャ
    let add = |a: i32, b: i32| a + b;
    println!("add: {}", add(3, 4));

    // 複数行のクロージャ
    let compute = |a: i32| {
        let doubled = a * 2;
        let squared = doubled * doubled;
        squared
    };
    println!("compute: {}", compute(3));

    let mut color = String::from("red");
    // 3. ムーブでキャプチャ（T）: 所有権を取得
    let consume_color = || {
        let _c = color; // 所有権をムーブ
        println!("consumed");
    };

    // 呼び出し
    let double = |x| x * 2;
    println!("apply: {}", apply(double, 5));

    // 高階関数
    let numbers = vec![1, 2, 3, 4, 5];
    // 関数ポインタを渡すことも可能
    fn is_odd(n: &i32) -> bool {
        n % 2 != 0
    }
    let odds: Vec<&i32> = numbers.iter().filter(|x| is_odd(x)).collect();
    println!("odds: {:?}", odds);
}
