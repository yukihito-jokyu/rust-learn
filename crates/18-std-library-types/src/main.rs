// 18 - 標準ライブラリの型（Standard Library Types）
// このファイルはTODOコメント付きの雛形です。
// guide.mdを参考にして、各TODOの部分を自分で実装してみてください。

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Box の使用 ===");

    let boxed_int = Box::new(5);
    println!("Boxの中身: {}", boxed_int);

    let unboxed = *boxed_int;
    println!("取り出した値: {}", unboxed);

    // TODO: Box::newでヒープに値を配置してください

    // TODO: 再帰的なデータ構造をBoxを使って定義してください
    // 例: 連結リスト

    // TODO: トレイトオブジェクトとしてBoxを使ってください

    println!("\n=== String の操作 ===");

    // TODO: Stringの作成、結合、検索を試してください

    // TODO: 文字列のイテレーション（chars, bytes）を試してください

    println!("\n=== Vec の操作 ===");

    // TODO: Vecの作成、要素の追加、アクセスを試してください

    // TODO: map, filter, collectを使ったVecの操作を試してください

    println!("\n=== HashMap / HashSet の使用 ===");

    // TODO: HashMapを作成し、insert, get, entryを試してください

    // TODO: HashSetを作成し、集合演算（積集合、和集合）を試してください

    println!("\n=== Rc / Arc の確認 ===");

    // TODO: Rc::cloneで参照カウントを確認してください

    // TODO: Arcを使ってスレッド間でデータを共有してください
}
