## ストレージと抽出

### データベースを駆動するデータ構造

- READ の検索コストは O(n) であるので、データ数に応じて劣化していく。
- そのためにメタデータが必要。
  - ハッシュインデックス
    - メモリに乗り切れない場合はランダムアクセスでデータを参照しにいくのでパフォーマンス劣化が激しい
    - 範囲に対する効率が悪い
  - SSTable（Sorted String Table）
    - 既存の行を挿入や更新で上書きするのではなく、挿入または更新したデータの新しいバージョンをタイムスタンプ付きで新しい SSTable に書き込みます。
      - この際にデータが key によってソートされている状態にしたい。
      - データ追加の際には、memtable というインメモリのツリーデータに一度データを格納してからディスクに書き出す。
      - Log-Structured Merge-Tree という
    - 削除は tomb stone のフラグを立てる
    - 重複するデータがファイルに含まれることになるので、最新バージョンのデータを残して削除するコンパクションが必要になる
    - データが存在しない場合はメモリではなくディスクをみにいくので遅くなる
  - B ツリー
    - Log-structured なインデックスを可変サイズのインデックスをファイルに格納する。
    - tree なので、分岐係数とどれだけ格納するかが重要
    - 複数のファイルを更新する場合があり、更新中にクラッシュするとデータが破損する
      - この際は Write-Ahead ログ（WAL, redo ログ）で復元する

### トランザクション処理か、分析処理か？

- オンライントランザクション処理
  - 顧客がデータを参照し、購買をするといった時の処理。
  - 低レイテンシーであることが重要
- オンライン分析処理
  - 大量データに対して分析を行う
  - バルクインポート（ELT）あるいはイベントストリームを用いる。
  - 別途データウェアハウスを持っている場合が多い。
  - ファクトテーブルを中心に、データをまとめるスタースキーマを用いる。
    - 基本はいつ、どこで、誰が何をしたのか、なぜ、どのように、でデータをまとめていく