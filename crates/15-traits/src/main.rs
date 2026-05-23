// 15 - トレイト（Traits）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

// TODO: Animalトレイトを定義してください
// - name() -> &'static str
// - noise() -> &'static str
// - talk()（デフォルト実装付き）

// TODO: Dog構造体を定義してください

// TODO: Cat構造体を定義してください

// TODO: DogにAnimalトレイトを実装してください

// TODO: CatにAnimalトレイトを実装してください（talkをオーバーライド）

// TODO: #[derive]を使った構造体を定義してください
// Debug, PartialEq, Cloneをderive

// TODO: 演算子オーバーロードを実装してください
// std::ops::Addを使ってVector2Dの+演算子をオーバーロード

// TODO: Dropトレイトを実装した構造体を定義してください

// TODO: Iteratorトレイトを実装したカスタムイテレータを定義してください
// 例: フィボナッチ数列やカウントダウン

// トレイトの定義
trait Animal {
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // デフォルト実装を提供することもできる
    fn talk(&self) {
        println!("{}は「{}」と言った！", self.name(), self.noise());
    }
}

struct Dog {
    name: &'static str,
}
struct Cat {
    name: &'static str,
}

// トレイトの実装
impl Animal for Dog {
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        "ワンワン！"
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        "ニャー！"
    }
    // デフォルトメソッドをオーバーライドすることも可能
    fn talk(&self) {
        println!("{}は甘えています！", self.name());
    }
}

// ジェネリック関数にトレイト境界を指定
fn print_animal<T: Animal>(animal: &T) {
    println!("{}の鳴き声: {}", animal.name(), animal.noise());
}

// where句を使ったより読みやすい書き方
fn some_function<T, U>(t: &T, u: &U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone,
{
    println!("T: {}, U: {:?}", t, u);
}

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

// + 演算子のオーバーロード
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D { x: self.x + other.x, y: self.y + other.y }
    }
}

// * 演算子のオーバーロード（スカラー倍）
impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D { x: self.x * scalar, y: self.y * scalar }
    }
}

fn main() {
    println!("=== トレイトの基本 ===");

    // TODO: DogとCatを作成し、talk()を呼び出してください
    let dog = Dog { name: "ポチ" };
    let cat = Cat { name: "ミケ" };

    dog.talk();
    cat.talk();

    println!("\n=== deriveの確認 ===");

    // TODO: deriveした構造体のインスタンスを作成し、
    // デバッグ出力と比較を試してください

    println!("\n=== 演算子オーバーロード ===");

    // TODO: Vector2Dの加算とスカラー乗算を試してください
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };

    let sum = v1 + v2;
    println!("和: {:?}", sum);

    let scaled = v1 * 2.0;
    println!("スカラー倍: {:?}", scaled);

    println!("\n=== Dropトレイト ===");

    // TODO: Dropを実装した構造体のスコープ抜け時の動作を確認してください

    println!("\n=== Iteratorトレイト ===");

    // TODO: カスタムイテレータを使って値を取得してください
}
