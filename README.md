# ポモドーロタイマー

Rust で実装されたシンプルなポモドーロタイマーアプリケーションです。[iced](https://github.com/hecrj/iced)フレームワークを使用した GUI アプリケーションとして実装されています。

## 機能

- 25 分のポモドーロタイマー
- スタート/ストップ機能
- リセット機能
- ミリ秒単位の精確なタイマー表示
- PixelMplus フォントを使用したスタイリッシュな UI

## 技術スタック

- [Rust](https://www.rust-lang.org/) 2018 edition
- [iced](https://github.com/hecrj/iced) - GUI フレームワーク
- [async-std](https://async.rs/) - 非同期ランタイム

## 必要要件

- Rust 2018 以降
- Cargo (Rust のパッケージマネージャー)

## セットアップ

1. リポジトリのクローン

```bash
git clone https://github.com/yourusername/pomodoro-timer.git
cd pomodoro-timer
```

2. ビルドと実行

```bash
cargo run
```

## 使用方法

1. アプリケーションを起動すると、25 分のタイマーが表示されます
2. 「Start」ボタンをクリックしてタイマーを開始
3. タイマー実行中は「Stop」ボタンでタイマーを一時停止可能
4. 「Reset」ボタンで 25 分に戻すことができます
5. タイマーが 0 になると自動的に停止します

## プロジェクト構造

```
pomodoro-timer/
├── src/
│   └── main.rs      # メインアプリケーションコード
├── rsc/
│   └── PixelMplus12-Regular.ttf  # フォントファイル
└── Cargo.toml       # Rust依存関係定義
```

## 依存関係

- iced = "0.1.1" (async-std 機能付き)
- iced_native = "0.2.2"
- iced_futures = "0.1.2"
- async-std = "1.6.1"

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。

## 作者

[makutak](mailto:dev.takuma.kouno@gmail.com)
