# CONTRIBUTING GUIDE for notectl [WIP]
notectlのコントリビューションガイドです。
まだ作成中です。

## 開発環境
notectlの開発には以下のいずれかの方法を使用することができます。
- [devcontainer](#devcontainer)
- [ローカル](#ローカル環境で開発する)

### devcontainer
devcontainerはVSCodeの拡張機能`Dev Containers(ms-azuretools.vscode-docker)`あるいはdevcontainer-cliとDocker Composeが必要です。  
デフォルトのコンテナ構成では、notectlのdevcontainerと、misskey、postgresql、redisの構成になっています。必要に応じてmeilisearchなどを有効にして下さい。  

### ローカル環境で開発する
以下の環境が必要です。
- Rust 1.81.0以降
- sea-orm-cli

また、MisskeyをローカルやDockerで動かす必要がある場合もあります。その際には次の手順を参考にローカルにMisskeyを立ててください。  
参考: [Misskeyサーバーの作成](https://misskey-hub.net/ja/docs/for-admin/install/guides/)

## Issue
イシューを作成する前に、以下の点を確認して下さい:
- 重複を避けるため、Issueを作成する前に似たようなIssueがあるか確認してください
- 質問やトラブルシューティングのためにIssueを利用しないでください
    - Issueはあくまで機能のリクエスト、提案、バグ追跡にのみ使用されるべきです
    - わからないことやトラブルシューティングは、[GitHub Discussions](https://github.com/1673beta/notectl/discussions)に作成するか、[リポジトリ所有者](https://c.koliosky.com/@esurio1673)に質問しにきてください。

> [!WARNING]
> 解決されようとしているIssueは、解決策がマージされるまで閉じないでください。

