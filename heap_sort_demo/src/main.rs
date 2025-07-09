mod heap_sort;
use heap_sort::{make_heap, print_tree};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let mut data: Vec<i32> = (0..15).map(|_| rng.gen_range(0..100)).collect();
    println!("\nBefore      : {:?}", data);

    //  힙으로 변환
    make_heap(&mut data, true);                // 최대 힙
    println!("\nAs Max-Heap (array) : {:?}", data);
    print_tree(&data);                         // 트리 형태로 출력

    // 오름차순 정렬
    let mut asc = data.clone();
    heap_sort::heap_sort(&mut asc, true);
    println!("\nAscending   : {:?}", asc);

    // 내림차순 정렬
    let mut desc = data.clone();
    heap_sort::heap_sort(&mut desc, false);
    println!("\nDescending  : {:?}", desc);
}