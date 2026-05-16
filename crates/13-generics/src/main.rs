// TODO: ジェネリック関数 `largest` を定義してください
// - スライスから最大値を見つける関数
// ヒント: fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }

// TODO: ジェネリック構造体 `Pair<T, U>` を定義してください
// - first: T, second: U の2つのフィールド
// ヒント: struct Pair<T, U> { first: T, second: U }

// TODO: トレイト境界を使って関数を定義してください
// - Display + PartialOrd を満たす型を比較して表示する関数
// ヒント: fn compare_and_print<T: Display + PartialOrd>(a: T, b: T) { ... }

// TODO: where 句を使って関数を定義してください
// ヒント: fn some_fn<T, U>(t: &T, u: &U) -> String where T: Display, U: Display { ... }

// TODO: PhantomData を使って型レベルで区別する構造体を作ってください
// ヒント: use std::marker::PhantomData;
// ヒント: struct Distance<Unit> { value: f64, _phantom: PhantomData<Unit> }

// T は型パラメータ
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // ここにコードを書いてください
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));
}
