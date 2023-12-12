use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const TABLE_SIZE: usize = 128; // ハッシュテーブルのバケット数

struct HashTable<K, V> {
    buckets: Vec<Option<(K, V)>>, // キーと値のペアを格納するバケット
}

impl<K, V> HashTable<K, V>
where
    K: Hash + Eq, // K は Hash と Eq トレイトを実装する必要がある
{
    // 新しいハッシュテーブルを生成する
    fn new() -> Self {
        // TABLE_SIZE の長さを持つ Vec を None で初期化
        let buckets = (0..TABLE_SIZE).map(|_| None).collect(); // 各バケットを None で初期化
        HashTable { buckets }
    }

    // キーからハッシュ値を計算する
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher); // キーをハッシュ化
        (hasher.finish() as usize) % TABLE_SIZE // ハッシュ値をテーブルサイズで割った余りを返す
    }

    // ハッシュテーブルにキーと値のペアを挿入する
    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key); // キーに基づいてインデックスを計算

        // 線形探索で空きスペースまたはキーが一致する場所を探す
        for i in 0..TABLE_SIZE {
            let new_index = (index + i) % TABLE_SIZE;
            let bucket = &mut self.buckets[new_index];

            if let Some((ref existing_key, _)) = bucket {
                if existing_key == &key {
                    *bucket = Some((key, value)); // 既存のキーを更新
                    return;
                }
            } else {
                *bucket = Some((key, value)); // 新しいキーを挿入
                return;
            }
        }

        panic!("ハッシュテーブルが満杯です");
    }

    // キーに基づいて値を取得する
    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key); // キーに基づいてインデックスを計算

        for i in 0..TABLE_SIZE {
            let new_index = (index + i) % TABLE_SIZE;
            if let Some((ref existing_key, ref value)) = self.buckets[new_index] {
                if existing_key == key {
                    return Some(value); // キーが一致する値を返す
                }
            }
        }

        None // キーが見つからない場合は None を返す
    }
}

fn main() {
    let mut hash_table = HashTable::new();

    // ハッシュテーブルにキーと値のペアを挿入
    hash_table.insert("key1", "value1");
    hash_table.insert("key2", "value2");
    hash_table.insert("key3", "value3");

    // キーに基づいて値を取得して表示
    println!("key1: {:?}", hash_table.get(&"key1"));
    println!("key2: {:?}", hash_table.get(&"key2"));
    println!("key3: {:?}", hash_table.get(&"key3"));
    println!("key4: {:?}", hash_table.get(&"key4")); // 存在しないキー
}
