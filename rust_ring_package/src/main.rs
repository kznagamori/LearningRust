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
