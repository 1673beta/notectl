# 実装されている機能

## はじめに
`webpush`および`id`以外のコマンドは全て`--config`あるいは`-c`フラグで`.config/default.yml`が存在するパスを指定してください。

## Search CLI
```bash
notectl search deploy --config <CONFIG_PATH>
```
Meilisearchに対してインデックスを生成します。既にMeilisearchのインデックスが存在する場合、後述の`drop`コマンドを使用してから実行してください。
  
```bash
notectl search drop --config <CONFIG_PATH>
```
Meilisearchのインデックスを削除します。

```bash
notectl search list
```
現在存在するMeilisearchのインデックスの一覧を取得します。

```bash
notectl search health
```
Meilisearchのヘルスチェックを行います。

## Webpush CLI
```bash
notectl webpush generate
```
プッシュ通知機能を有効にするのに必要な公開鍵と秘密鍵を生成します。

## Config CLI
```bash
notectl config show --config <CONFIG_PATH>
```
`.config/default.yml`の記述をjson形式で出力します。

## Remote CLI
```bash
notectl remote gone --config <CONFIG_PATH> --url <INSTANCE_URL>
```
`--url`フラグで指定したリモートサーバーを410 Goneで閉鎖したと指定します。INSTANCE_URLには`https://` の部分はつけないでください。

```bash
notectl remote suspend --config <CONFIG_PATH> --url <INSTANCE_URL>
```
`--url`フラグで指定したリモートサーバーに対して配信停止します。既に410を出して停止している、あるいは既に配信停止になっているサーバーについてはエラーが出力されます。

```bash
notectl remote unsuspend --config <CONFIG_PATH> --url <INSTANCE_URL>
```
`--url`フラグで指定したリモートサーバーに対して実施している配信停止を解除します。

## Id CLI
```bash
notectl id parse --format <ID_TYPE> --id <ID>
```
指定された形式のidから日時を取得します。
```bash
notectl id gen --format <ID_TYPE>
```
指定された形式で、現在日時からIDを生成します。

## Note CLI
```bash
notectl note delete --config <CONFIG_PATH> --days <DAYS> [--host: <HOST>]
```
指定した日数以前のノートを削除します。  
日数には1〜18446744073709551615までが指定できます。  
`--host`フラグを使用することで、特定のホストの指定した日数以前のノートを消すことも可能です。このフラグを使用しない場合、デフォルトではリモートの全ての投稿が対象となります。    
