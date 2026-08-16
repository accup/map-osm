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

### データベースの構造

利用側は最初に `metadata` テーブルを読み、出典表示・ライセンスと `kind` 列の値の意味を確認すること。`attribution` の値は、そのまま帰属の表示となる文字列である。座標の分解能は OSM と同一の 1e-7 度であり、形状の簡略化の許容誤差と合わせて、原典からの偏差は 0.1 メートル以内である。

| テーブル            | 列                                                                     | 内容                                                                                                                                                                                 |
| ------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `metadata`          | `key`・`value`                                                         | 付帯情報。出典表示（`attribution` と `attribution_url`）・ライセンス（`license` と `license_url`）・取得元（`source`）と、`kind` 列の値の意味（`line_kind.<値>`・`point_kind.<値>`） |
| `line_group`        | `id`・`kind`・`reference`・`name`                                      | 付番（`reference`）と名称を持つ路線。構成する折れ線は `line_group_member` で参照する                                                                                                 |
| `line`              | `id`・`kind`・`coordinates`                                            | 路線を構成する折れ線。`id` は OSM のウェイの識別子。`coordinates` は（緯度, 経度）の順の座標列を 1e-7 度の固定小数点へ量子化し、直前の点との差分を zigzag varint として並べた BLOB   |
| `line_group_member` | `group_id`・`line_id`                                                  | 路線と折れ線の多対多の対応                                                                                                                                                           |
| `line_index`        | `id`・`min_latitude`・`max_latitude`・`min_longitude`・`max_longitude` | 折れ線の外接矩形の R-tree 索引。`id` は `line.id` を参照する                                                                                                                         |
| `point`             | `id`・`kind`・`reference`・`name`・`latitude`・`longitude`             | 駅・インターチェンジの地点。`id` は OSM のノードの識別子。座標は 1e-7 度単位の整数                                                                                                   |

`kind` の値の意味は、路線（`line_group`・`line`）が 1: 鉄道、2: 高速道路、3: 国道、4: 都道府県道、地点（`point`）が 1: 駅、2: インターチェンジである。

## ライセンス

- ソースコードは [MIT License](./LICENSE) の下で提供する。
- `data/japan-routes.sqlite` は OpenStreetMap のデータ（© OpenStreetMap contributors、[出典](https://www.openstreetmap.org/copyright)）を複製・加工した派生データベースであり、[Open Database License (ODbL) 1.0](https://opendatacommons.org/licenses/odbl/1-0/) の下で提供する。詳細は [data/LICENSE](./data/LICENSE) を参照。アプリケーションなどへ組み込んで再配布する場合は、帰属表示と ODbL の明示を行うこと。
