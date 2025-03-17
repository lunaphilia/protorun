# 5. 所有権システム

## 5.1 所有権

```
// 所有権の移動
fn take(v: Vec<Int>): Int = {
  // vの所有権がここに移動
  v.length
}

let vec = [1, 2, 3]
take(vec)
// ここでvecは使用不可
```

## 5.2 借用

```
// 不変借用
fn peek(v: &Vec<Int>): Int = {
  // vの参照のみを使用
  v.length
}

// 可変借用
fn modify(v: &mut Vec<Int>): Unit = {
  // vの可変参照を使用
  v.push(42)
}

let mut vec = [1, 2, 3]
peek(&vec)    // 不変借用
modify(&mut vec)  // 可変借用
```

## 5.3 ライフタイム

```
// ライフタイム注釈
fn longest<'a>(x: &'a str, y: &'a str): &'a str = {
  if x.length > y.length { x } else { y }
}

// 構造体でのライフタイム
type Ref<'a, T> = {
  value: &'a T
}
```

## 5.4 リソース管理

```
// リソース型を使用したリソース管理
fn processFile(path: String): Result<String, IOError> = {
  // ファイルを開く（スコープ終了時に自動的に閉じられる）
  let file = File.open(path)?
  
  // ファイルの内容を読み取る
  let content = file.read()?
  
  // 内容を処理
  let processed = processContent(content)
  
  Result.Ok(processed)
} // ここでfileのreleaseメソッド（close）が自動的に呼び出される
