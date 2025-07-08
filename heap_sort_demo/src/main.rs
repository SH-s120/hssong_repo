mod heap_sort;

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let original: Vec<i32> = (0..15).map(|_| rng.gen_range(0..100)).collect();
    println!("\nBefore      : {:?}", original);

    // 오름차순 정렬
    let mut asc = original.clone();
    heap_sort::heap_sort(&mut asc, true);
    println!("Ascending   : {:?}", asc);

    // 내림차순 정렬
    let mut desc = original.clone();
    heap_sort::heap_sort(&mut desc, false);
    println!("Descending  : {:?}", desc);
}