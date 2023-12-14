# マクロ

Rustにはいくつかの組み込みマクロがあり、それぞれ異なる用途に使用されます。以下は、よく使われるRustのマクロの一覧とその簡単な説明です：

**1. println!:**
- 標準出力に文字列を出力します。文字列は改行で終わります。
- 例: `println!("Hello, world!");`

**2. print!:**
- `println!`と似ていますが、改行を付けずに標準出力に文字列を出力します。
- 例: `print!("Hello, "); print!("world!");`

**3. format!:**
- 文字列をフォーマットしますが、出力せずに文字列を返します。
- 例: `let s = format!("{} {}", "hello", "world");`

**4. panic!:**
- プログラムの実行を中止し、エラーメッセージを表示します。
- 例: `panic!("This is an error message");`

**5. assert!:**
- 指定された条件が `false` の場合にパニックを引き起こします。テストやデバッグ時に使用されます。
- 例: `assert!(1 == 1)`;

**6. assert_eq! と assert_ne!:**
- `assert_eq!` は二つの引数が等しいかチェックし、`assert_ne!` は二つの引数が等しくないかチェックします。
- 例: `assert_eq!(2 + 2, 4);`、`assert_ne!(2 + 2, 5);`

**7. vec!:**
- ベクタを作成します。ベクタは、同じ型の値の動的配列です。
- 例: `let v = vec![1, 2, 3];`

**8. macro_rules!:**
- 独自のマクロを定義するために使用します。
- 例: `macro_rules! say_hello { () => { println!("Hello!"); }; }`

**9. cfg!:**
- コンパイル時の条件付きチェックに使用されます。特定の構成オプションが有効かどうかを確認します。
- 例: `if cfg!(target_os = "windows") { println!("Running on Windows!"); }`

**10. include!:**
- 指定されたファイルの内容を現在のファイルに含めます。
- 例: `include!("foo.rs");`

**11. concat!:**
- 与えられた文字列リテラルを連結します。
- 例: `let s = concat!("foo", "bar", "baz");`

**12. env! と option_env!:**
- `env!`はコンパイル時に環境変数の値を取得し、`option_env!` は環境変数が設定されていない場合に `None` を返します。
- 例: `let path = env!("PATH");`、`let key = option_env!("SECRET_KEY");`

これらのマクロはRustの標準ライブラリに組み込まれており、さまざまな目的で使用されます。プログラムのパフォーマンス、エラーハンドリング、デバッグ、データ構造の作成、条件付きコンパイルなど、多岐にわたる用途に対応しています。