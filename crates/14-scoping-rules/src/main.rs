// TODO: 所有権のムーブを確認してください
// - String を別の変数に代入して、元の変数が使えなくなることを確認
// ヒント: let s1 = String::from("hello"); let s2 = s1;
// ヒント: // println!("{}", s1); // エラーになることを確認

// TODO: Copy トレイトを持つ型の動作を確認してください
// - i32 を代入して、両方の変数が使えることを確認
// ヒント: let x = 42; let y = x; println!("x: {}, y: {}", x, y);

// TODO: 不変借用（&T）を試してください
// - 関数に &String を渡して、呼び出し側でも使えることを確認
// ヒント: fn calculate_length(s: &String) -> usize { s.len() }

// TODO: 可変借用（&mut T）を試してください
// - 関数に &mut String を渡して文字列を変更
// ヒント: fn append(s: &mut String) { s.push_str(" world"); }

// TODO: 借用のルールを確認してください
// - 不変借用が複数同時に存在できることを確認
// - 可変借用は1つだけしか存在できないことを確認
// - 不変借用と可変借用は同時に存在できないことを確認
// ヒント: let r1 = &s; let r2 = &s; // OK
// ヒント: let r3 = &mut s; // r1, r2 が使われなくなってからならOK

// TODO: ライフタイム注釈を使った関数を定義してください
// - 2つの文字列スライスを受け取り、長い方を返す関数
// ヒント: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }

// TODO: ライフタイムを持つ構造体を定義してください
// ヒント: struct Excerpt<'a> { content: &'a str }

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping resource: {}", self.name);
    }
}

fn take_ownership(s: String) {
    println!("taken: {}", s);
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

// 'a はライフタイムパラメータ
// 戻り値の参照は x と y の短い方のライフタイムを持つ
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // ここにコードを書いてください
    let r1 = Resource { name: String::from("A") };
    {
        let r2 = Resource { name: String::from("B") };
        println!("Inside inner scope");
    } // r2 がドロップされる
    println!("Outside inner scope");

    let s = String::from("hello");
    take_ownership(s);

    let s1 = String::from("long string");

    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("longest: {}", result);
        // s2 がスコープを抜けた後に result を使うとエラー
    }
    // println!("result: {}", result); // エラー！ s2 はもう存在しない
}
