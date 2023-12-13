# リングバッファ

Rustでリングバッファ（ `circular buffer` ）を使用するために、多くの場合外部クレートを利用します。ここでは、一般的に使われる `ringbuf` クレートを例に、実行可能なサンプルプログラムを提供します。

まず、`Cargo.toml` ファイルに `ringbuf` クレートを依存関係として追加する必要があります。

## サンプルプログラム

**Cargo.toml**
まず、`Cargo.toml` ファイルに `ringbuf` クレートを依存関係として追加する必要があります。

```
[dependencies]
ringbuf = "0.2"
```

**main.rc**
```rust
use ringbuf::RingBuffer;
use std::convert::TryInto;

fn main() {
    let capacity = 5;
    let rb = RingBuffer::<i32>::new(capacity);
    let (mut prod, mut cons) = rb.split();

    // リングバッファに要素を追加
    for i in 0..capacity {
        // usize型の`i`をi32型に変換
        let value: i32 = i.try_into().unwrap();
        prod.push(value).unwrap();
    }

    // 要素がオーバーフローしないように注意
    //let value: i32 = capacity.try_into().unwrap();
    //prod.push(value).unwrap(); // これはエラーになる

    // リングバッファから要素を取り出す
    while let Some(value) = cons.pop() {
        println!("Popped: {}", value);
    }
}
```

このプログラムでは、指定した容量のリングバッファを作成し、そのバッファに整数を追加しています。リングバッファの容量を超えて要素を追加しようとするとエラーが発生するため、この点に注意が必要です。取り出し側では、popメソッドを使用してバッファから要素を取り出し、それを表示しています。

この例では基本的なリングバッファの使用方法を示していますが、`ringbuf` クレートはさらに高度な機能も提供しているため、詳細については公式ドキュメントを参照することをお勧めします。
