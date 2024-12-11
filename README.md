# JLI Project

https://jli.li のリポジトリです。

## 何故JLIという名前なんですか？

ドメインにした時jli.liという文字列が短く見えていいなと思ってドメインを取りました。

なので具体的な意味はありません

## 何故オープンソース？

Team ThunLightsにはRustが得意なプログラマーが居ないためもしコードに間違いなどあれば、Pull Requestをしてほしいからです。

またTeam ThunLightsは現在進行形でRustが得意なプログラマーを募集中です。

詳しくは[こちら](https://github.com/ThunLights#%E3%83%A1%E3%83%B3%E3%83%90%E3%83%BC%E5%8B%9F%E9%9B%86)をご覧ください

## セットアップ方法

### 1. .env.exampleをベースに.envを作成する。

### 2. マイグレーションコマンドを打つ

```console
sqlx migrate run
```

## 更新一覧

バージョン一覧を書いておきます。

### Version 1.0

リリース: 2024/10/06

色々苦戦しつつも何とかリリース

HTMLが一部完成してないので早いうちに改善したい
