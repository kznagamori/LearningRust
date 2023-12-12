// クイックソートを実行する関数
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 配列の長さが2未満の場合、ソートは必要ない
    if len < 2 {
        return;
    }
    // 配列をピボットで分割し、ピボットのインデックスを取得
    let pivot_index = partition(arr);
    // ピボットより左側のサブ配列を再帰的にソート
    quick_sort(&mut arr[0..pivot_index]);
    // ピボットより右側のサブ配列を再帰的にソート
    quick_sort(&mut arr[pivot_index + 1..len]);
}

// 配列をピボットで分割する関数
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    // ピボットとして配列の最後の要素を選択
    let pivot_index = len - 1;
    let mut i = 0;

    // 配列を走査し、ピボットより小さい要素を左側に移動
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // ピボットを適切な位置に移動
    arr.swap(i, pivot_index);
    i // ピボットの新しいインデックスを返す
}

fn main() {
    let mut arr = [3, 6, 8, 10, 1, 2, 1];
    println!("Before sorting: {:?}", arr);
    quick_sort(&mut arr); // 配列にクイックソートを適用
    println!("After sorting: {:?}", arr);
}
