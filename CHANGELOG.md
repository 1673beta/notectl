# Unreleased

## Release Date
unreleased

## Feat
- `user delete`で`username`を指定できるように
- `note delete`で`visibility`を指定できるように
- `note delete`で反応がないノートのみを削除できるように

## Fix

# 0.1.3

## Release Date
2025/02/06

## Fix
- バージョンの上げ忘れ

# 0.1.2

## Release Date
2025/02/05

## Dev
- editorconfigとrustfmtを改善

## Fix
- opensslを0.10.70に
- meilisearchが有効な場合に`note delete`を実行する際に、誤ってローカルの投稿が削除される問題を修正

# 0.1.1

## Release Date
2025/1/12

## Fix
- meilisearchが有効な場合に`note delete`を実行する際、ホスト指定が考慮されていなかった問題を修正


# 0.1.0

## Release Date
2025/1/12

## Features
- `user`コマンドを追加
- `note`コマンドを追加
- `config`コマンドを追加
- `id`コマンドを追加
- `search`コマンドを追加
- `remote`コマンドを追加
- `webpush`コマンドを追加
