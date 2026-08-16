# map-osm

OpenStreetMap のデータを複製・加工して、日本の鉄道網・道路網の番号（路線記号、駅番号、路線番号、付番など）、名称、路線の形状を整理したデータを作成するプロジェクト。

## 構成

ワークスペースは、単一の明確な目的を持つ汎用クレートと、それらを繋ぎ合わせるアプリケーション本体のクレート（`crates/map`）で構成する。クレートの一覧は [Cargo.toml](./Cargo.toml) の `members` を、各クレートの目的は各クレートの Cargo.toml の `description` を参照。

## 環境

- [rustup](https://rustup.rs/)
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/)
- [GNU Make](https://www.gnu.org/software/make/)

## 開発

| コマンド      | 内容                                         |
| ------------- | -------------------------------------------- |
| `make format` | ソースコードを整形する                       |
| `make lint`   | 静的検査を行う                               |
| `make test`   | ワークスペースのテストを実行する             |
| `make check`  | ワークスペースの型検査を行う                 |
| `make build`  | リリースビルドを行う                         |
| `make ready`  | format・check・test・lint をまとめて実行する |

## データの構築

Geofabrik が配布する日本全域の OpenStreetMap データ（PBF 形式）を `data/japan-latest.osm.pbf` へダウンロードし（既に存在する場合は省略）、鉄道・高速道路・国道・都道府県道の路線と駅・インターチェンジの地点を抽出して、SQLite データベース `data/japan-routes.sqlite` を構築する。

```sh
cargo run --release
```

データベースは路線の種別・付番・名称と簡略化した形状、地点の種別・付番・名称と座標、および R-tree による空間索引を持つ。

## ライセンス

ソースコードは [MIT License](./LICENSE) の下で提供する。OpenStreetMap のデータは © OpenStreetMap contributors であり、複製・加工したデータは [Open Database License (ODbL)](https://www.openstreetmap.org/copyright) に従う。
