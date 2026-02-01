# Bomberman TUI

ターミナル上で動作するボンバーマンライクなTUIゲーム

## 技術スタック

- **Rust**: 高速・メモリ安全
- **Ratatui**: TUIフレームワーク
- **Crossterm**: ターミナルバックエンド
- **Tokio**: 非同期ランタイム

## 操作方法

| キー | 動作 |
|------|------|
| `h` | 左移動 |
| `j` | 下移動 |
| `k` | 上移動 |
| `l` | 右移動 |
| `Space` | 爆弾設置 |
| `p` | ポーズ |
| `q` / `Esc` | 終了 |

## 開発環境

このプロジェクトはDev Containerを使用しています。VS Codeで開き、"Reopen in Container"を選択してください。

## ビルド＆実行

```bash
cargo run
```
