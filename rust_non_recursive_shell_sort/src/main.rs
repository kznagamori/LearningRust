// シェルソートを実行する関数
fn shell_sort<T: Ord>(arr: &mut [T]) {
    // ギャップの初期値を配列の長さの半分に設定
    let mut gap = arr.len() / 2;

    // ギャップが0より大きい間、処理を繰り返す
    while gap > 0 {
        // ギャップから配列の終わりまで繰り返す
        for i in gap..arr.len() {
            let mut j = i;
            // ギャップを使って後ろから比較していく
            while j >= gap && arr[j - gap] > arr[j] {
                // より小さい要素を前に移動
                arr.swap(j - gap, j);
                j -= gap;
            }
        }
        // ギャップを半分に減らす
        gap /= 2;
    }
}

fn main() {
    let mut arr = [9, 8, 3, 7, 5, 6, 4, 1];
    println!("Before sorting: {:?}", arr);
    // 配列にシェルソートを適用
    shell_sort(&mut arr);
    println!("After sorting: {:?}", arr);
}
