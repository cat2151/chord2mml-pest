# chord2mml-pest

Rustのpestクレートを利用した、コード進行パーサー。

コード進行（例: "C-F-G-C"）を度数の配列（例: "1,4,5,1"）にパースします。

## 機能

- **Rustライブラリクレート**: 他のRustプロジェクトから利用可能
- **WASMライブラリ**: JavaScriptから利用可能（`pkg/`ディレクトリ）
- **CLIデモ**: コマンドラインから実行
- **ブラウザデモ**: テキストエリアで入力、コンソールに出力

## インストール

### Rustライブラリとして使用

`Cargo.toml`に追加:

```toml
[dependencies]
chord2mml_pest = { git = "https://github.com/cat2151/chord2mml-pest" }
```

### WASMとして使用

```bash
wasm-pack build --target web --out-dir pkg
```

## 使い方

### Rustライブラリ

```rust
use chord2mml_pest::parse_chord_progression;

fn main() {
    let result = parse_chord_progression("C-F-G-C").unwrap();
    println!("{:?}", result); // [1, 4, 5, 1]
}
```

### CLI

```bash
cargo run --bin chord2mml-cli C-F-G-C
# 出力: 1,4,5,1
```

### ブラウザ (WASM)

デモページ: `demo/index.html`

ローカルで実行:
```bash
# 簡易サーバーを起動（例: Python）
cd demo
python3 -m http.server 8000
# ブラウザで http://localhost:8000 を開く
```

### JavaScript/TypeScript

```javascript
import init, { parse_chords_wasm } from './pkg/chord2mml_pest.js';

await init();
const result = parse_chords_wasm("C-F-G-C");
console.log(result); // "1,4,5,1"
```

## 開発

### ビルド

```bash
# ライブラリとCLIをビルド
cargo build

# WASMをビルド
wasm-pack build --target web --out-dir pkg
```

### テスト

```bash
cargo test
```

## ライセンス

MITライセンス
