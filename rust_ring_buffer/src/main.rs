struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    size: usize,
    start: usize,
    end: usize,
}

impl<T> RingBuffer<T> {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);

        // bufferをNoneで初期化します。
        for _ in 0..size {
            buffer.push(None);
        }

        RingBuffer {
            buffer,
            size,
            start: 0,
            end: 0,
        }
    }

    // バッファにデータを追加する関数。
    fn push(&mut self, item: T) {
        self.buffer[self.end] = Some(item);
        self.end = (self.end + 1) % self.size;

        if self.end == self.start {
            // バッファが満杯の場合、startを移動して古いデータを上書きします。
            self.start = (self.start + 1) % self.size;
        }
    }

    // バッファからデータを取り出す関数。
    fn pop(&mut self) -> Option<T> {
        if self.start == self.end {
            // バッファが空の場合はNoneを返します。
            None
        } else {
            let result = self.buffer[self.start].take();
            self.start = (self.start + 1) % self.size;
            result
        }
    }

    // 指定されたインデックスの要素を参照する関数。
    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            None
        } else {
            self.buffer[(self.start + index) % self.size].as_ref()
        }
    }
}

fn main() {
    let mut buffer = RingBuffer::new(3);

    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    // バッファの現在の状態を表示します。
    for i in 0..3 {
        println!("buffer[{}] = {:?}", i, buffer.get(i));
    }

    // データを取り出し、表示します。
    while let Some(value) = buffer.pop() {
        println!("pop: {}", value);
    }

    // バッファの現在の状態を表示します。
    for i in 0..3 {
        println!("buffer[{}] = {:?}", i, buffer.get(i));
    }
}
