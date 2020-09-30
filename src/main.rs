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

#[derive(Copy, Clone)]
enum BroomIntent { FetchWater, DumpWater } // Broomができる2つの活動

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom { height: b.height / 2, .. b };
    let mut broom2 = Broom { name: broom1.name.clone(), .. broom1 };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = new_map((width, height), vec![0; width * height]);

    assert_eq!(image.size, (1024, 576));
    assert_eq!(image.pixels.len(), 1024 * 576);
}
