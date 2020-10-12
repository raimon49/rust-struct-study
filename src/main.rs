// 8bitグレースケールピクセルの長方形
// 構造体を含む型の名前はCamelCaseを使う慣習
// デフォルトでプライベートのため、外部公開するにはキーワードpubを付ける
pub struct GrayscaleMap {
    // フィールド名/メソッド名はsnake_case
    // 型はパブリックでフィールドはプライベート（デフォルト）
    pixels: Vec<u8>,
    size: (usize, usize)
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum BroomIntent { FetchWater, DumpWater } // Broomができる2つの活動

// 値としてBroomを受け取り、所有権を得る。
fn chop(b: Broom) -> (Broom, Broom) {
    // `broom1` の大半を変数 `b` から作り、高さだけを半分にする。
    // StringはCopyではないので、 `broom1` は `b` の名前の所有権を得る。
    let mut broom1 = Broom { height: b.height / 2, .. b };

    // `broom2` の大半を `broom1` から作る。StringはCopyでないので `name` を明示的にclone()する。
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };

    // それぞれに別の名前を与える。
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    // 2つを返す。
    (broom1, broom2)
}

// タプル型構造体
// 型を定義すると自動的に生成用の関数が定義される（引数は通常のタプルと同じ）
struct Bounds(pub usize, pub usize);

// ユニット型構造体
// 1つの値しか持たない型
struct Onesuch;

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>
}

// implブロックはfnによる関数定義の集合体
// 構造体Queueに関連付けられた関数（associated function）と呼ばれる
// 逆にimplブロックの外で定義されたものは自由関数（free function）と呼ばれる
impl Queue {
    // メソッドの第1引数はself: &mut Queueだが型は自明なため省略可能
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    // selfを変更しないメソッド定義では共有参照（&mutでなく&）で宣言すればよい
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    // メソッドがselfの所有権を取得する（引数を参照でなくする）
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = new_map((width, height), vec![0; width * height]);

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    assert_ne!(hokey1.intent, BroomIntent::DumpWater);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);
    assert_ne!(hokey2.intent, BroomIntent::DumpWater);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let _o = Onesuch;

    let mut q = Queue { older: Vec::new(), younger: Vec::new() };
    q.push('0'); // pushメソッド定義により、暗黙的には(&mut q).push(...)として変更可能参照が借用される
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());
    q.push('σ');
    assert!(!q.is_empty());

    let (older, younger) = q.split(); // qの所有権はsplitメソッドに移動し、未定義状態となった
    assert_eq!(older.len(), 0);
    assert_eq!(younger.len(), 1);
}
