# map

国土数値情報・国土地理院などの公的なデータを、道路や路線の名称・形状・グラフ化された接続のデータへ加工して配置するプロジェクト。

## 必要環境

- [rustup](https://rustup.rs/)（ツールチェーンは [rust-toolchain.toml](./rust-toolchain.toml) に従って自動で導入される）
- [Node.js](https://nodejs.org/) と [pnpm](https://pnpm.io/)（バージョンは [package.json](./package.json) の `devEngines` が管理する）
- [GNU Make](https://www.gnu.org/software/make/)

## 開発

アプリケーション本体（`crates/map`）を薄く保ち、計算は `crates/` 配下の汎用クレートへ切り出す。

| コマンド      | 内容                                         |
| ------------- | -------------------------------------------- |
| `make format` | ソースコードを整形する                       |
| `make lint`   | 静的検査を行う                               |
| `make test`   | ワークスペースのテストを実行する             |
| `make check`  | ワークスペースの型検査を行う                 |
| `make build`  | リリースビルドを行う                         |
| `make ready`  | format・check・test・lint をまとめて実行する |
