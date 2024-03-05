# LearningRust
Rust学習用レポジトリ

## Rustとは
Rustは、高いパフォーマンスとメモリ安全性を提供するために設計されたシステムプログラミング言語です。Mozilla Researchによって開発され、2010年代初頭に公開されました。以下はRust言語の主要な特徴です：

1. メモリ安全性：Rustの最も注目すべき特徴の一つは、コンパイル時の所有権と借用の概念を通じてメモリ安全性を提供することです。これにより、ガベージコレクションを使用せずにメモリリークや競合状態を防ぐことができます。

2. 並行処理：Rustはデータ競合を防ぐための安全な並行処理メカニズムを持っています。これにより、マルチスレッドプログラムをより安全に書くことができます。

3. パフォーマンス：RustはC言語と同等のパフォーマンスを提供し、低レベルのシステムプログラミングに適しています。

4. ゼロコスト抽象化：Rustの抽象化は、ランタイムコストを発生させないように設計されています。これにより、抽象化を利用しつつも高いパフォーマンスを維持することができます。

5. 型安全性と型推論：Rustは静的型付け言語であり、型安全性を提供します。また、型推論により、プログラマがすべての変数に型を明示的に指定する必要がない場合があります。

6. エコシステムとツール：Cargoというビルドシステムとパッケージマネージャーを含む強力なツールチェーンを持っています。また、Crates.ioというパッケージレジストリを通じて、様々なライブラリ（クレートと呼ばれる）にアクセスできます。

7. クロスプラットフォーム対応：Rustは多くのプラットフォームで動作し、クロスプラットフォーム開発をサポートしています。

Rustは、システムレベルのプログラミング、組み込みシステム、ウェブアセンブリ、並行処理が必要なアプリケーション、およびパフォーマンスが重要な場面で広く使われています。安全性、速度、並行処理のサポートにより、近年非常に人気が高まっている言語の一つです。

### 推しポイント
- 実行速度が速い。
- メモリ安全性が高い。

### いまいち、ダメなポイント
- 構文がC言語体系とは異なるため、学習コストが高い。
- メモリアクセスが分かりづらい。
- クレート(パッケージ)が少ないかつ、破壊的変更があるので使いづらい。
- WindowsだとVisual Studioが必要。
- ChatGPTで質問しても、正解コードの取得が難しい。

## 基礎
- [プロジェクト作成からビルドまでの手順](./rust_start_project/README.md)
- [外部クレートを使用したプログラムを作成する手順](./rust_use_package/README.md)
- [複数ファイルを使用する](./rust_multi_file/README.md)
- [複数ファイルを機能ごとにクレートをを分けて使用する](./rust_multi_pack_file/README.md)
- [ファイル内グローバル変数と関数、ファイル外グローバル変数と関数](./rust_global_local_scope/README.md)
  - [ファイル内グローバル変数と関数、ファイル外グローバル変数と関数(アトミック)](./rust_atomic_global_local_scope/README.md)
- [クラスを使う](./rust_struct_methods/README.md)
- [パブリックなクラスのメンバ、メソッドを定義とプライベートなクラスのメンバ、メソッドを定義する](./rust_public_private/README.md)
- [クラスの継承を実現する](./rust_inherit_class/README.md)
- [クラスのインタフェースを使用したポリモーフィズムの実現](./rust_poly_class/README.md)
- [C#みたいなクラスのプロパティを定義、使用](./rust_class_property/README.md)
- [一般的なエラー処理](./rust_error_handling/README.md)
- [各型のstring formatの組み合わせ](./rust_string_format/README.md)
- [ジェネリック機能](./rust_generics_example/README.md)
- [ラムダ式](./rust_lambda_example/README.md)
- [C#みたいなLINQ機能](./rust_linq_example/README.md)

## アルゴリズムとデータ構造
- [リンクドリスト構造](./rust_linked_list/README.md)
- [リングバッファ構造](./rust_ring_buffer/README.md)
- [キュー構造](./rust_data_queue/README.md)
- [スタック構造](./rust_data_stack/README.md)
- [二分木構造](./rust_binary_tree/README.md)
- [平衡二分木構造](./rust_balanced_tree/README.md)
- [ハッシュテーブル](./rust_hash_table/README.md)
- [クイックソート](./rust_quick_sort/README.md)
- [再帰を使用しないシェルソート](./rust_non_recursive_shell_sort/README.md)

## 応用
- [リンクドリスト](./rust_list_package/README.md)
- [リングバッファ](./rust_ring_package/README.md)
- [キュー](./rust_queue_package/README.md)
- [スタック](./rust_stack_package/README.md)
- [平衡二分木](./rust_balanced_tree_package/README.md)
- [ハッシュテーブル](./rust_hash_table_package/README.md)
- [ソート](./rust_sort_package/README.md)
- [スレッド](./rust_threading_example/README.md)
- [Async/Await](./rust_async_await_example/README.md)
- [排他処理](./rust_mutex_example/README.md)
- [メッセージボックス](./rust_message_box_example/README.md)

## エコシステム
- [クレート（パッケージ）をインストールして使用する](./rust_install_package/README.md)

## 言語特性
- [メモリアクセス](./rust_memory/README.md)
  - [Box\<T>](./rust_memory/memory_box/README.md)
  - [Rc\<T>](./rust_memory/memory_rc/README.md)
  - [RefCell\<T>](./rust_memory/memory_ref_cell/README.md)
  - [Rc\<RefCell\<T>>](./rust_memory/memory_rc_ref_cell/README.md)
  - [Weak\<T>](./rust_memory/memory_weak/README.md)
  - [Arc\<T>](./rust_memory/memory_arc/README.md)
  - [Arc\<Mutex\<T>>](./rust_memory/memory_arc_mutex/README.md)
- [アトリビュート](./rust_attribute/README.md)
- [マクロ](./rust_macro/README.md)
- [Runtimeの静的リンク](./rust_static_link_runtime/README.md)

