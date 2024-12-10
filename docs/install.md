# インストール

## systemd環境でのインストール

#### 前提条件
- [ ] Rust v1.76.0以降が入っていること

#### やり方
```bash
git clone https://github.com/1673beta/notectl.git
cd notectl
cargo install --path src/
```

## docker環境での使用

### ローカルでイメージをビルドする場合
#### 前提条件
- [ ] Docker Composeがインストールされていること
- [ ] CPUが強靭であること(参考: M2のMacbook Airでは250秒から320秒程度かかります)

#### ビルド
```bash
git clone https://github.com/1673beta/notectl.git

cd notectl
docker compose build
```

### イメージをpullする場合
```bash
docker pull ここにURL
```

#### compose.ymlの編集
**Misskey側の**compose.ymlを開いて、下記項目を追加します。
```yml
  notectl:
    image: notectl-notectl:latest
    networks:
      - internal_network
      - external_network
    volumes:
      - ./files:/files
      - ./.config:/.config:ro
```

#### 実行
```bash
sudo docker compose run --rm notectl <subcommand>
```
サブコマンドはhelpを実行するか、[feature](./feature.md)を見てください。