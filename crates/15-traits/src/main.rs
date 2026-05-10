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

fn main() {
    println!("=== トレイトの基本 ===");

    // TODO: DogとCatを作成し、talk()を呼び出してください

    println!("\n=== deriveの確認 ===");

    // TODO: deriveした構造体のインスタンスを作成し、
    // デバッグ出力と比較を試してください

    println!("\n=== 演算子オーバーロード ===");

    // TODO: Vector2Dの加算とスカラー乗算を試してください

    println!("\n=== Dropトレイト ===");

    // TODO: Dropを実装した構造体のスコープ抜け時の動作を確認してください

    println!("\n=== Iteratorトレイト ===");

    // TODO: カスタムイテレータを使って値を取得してください
}
