fn build_heap<T: Ord>(arr: &mut [T], is_max_heap: bool) {
    if arr.len() <= 1 { return; }
    let mut i = (arr.len() - 1) / 2;
    while i > 0 {
        heapify(arr, i, is_max_heap);
        i -= 1;
    }
    heapify(arr, 0, is_max_heap);
}

fn heapify<T: Ord>(arr: &mut [T], i: usize, is_max_heap: bool) {
    if arr.len() <= 1 { return; }

    let mut extreme = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    // 함수 포인터 대신 인라인 비교
    let better = |a: &T, b: &T| {
        if is_max_heap { a > b } else { a < b }
    };

    if l < arr.len() && better(&arr[l], &arr[extreme]) {
        extreme = l;
    }
    if r < arr.len() && better(&arr[r], &arr[extreme]) {
        extreme = r;
    }
    if extreme != i {
        arr.swap(i, extreme);
        heapify(arr, extreme, is_max_heap);
    }
}

pub fn make_heap<T: Ord>(arr: &mut [T], is_max_heap: bool) {
    if arr.len() <= 1 { return; }
    build_heap(arr, is_max_heap);
}

use std::fmt::Debug;

/// 배열을 레벨 단위로 나누어 출력해 힙 구조를 시각화
pub fn print_tree<T: Debug>(arr: &[T]) {
    if arr.is_empty() { return; }

    let mut level = 0;
    let mut nodes_in_level = 1;
    let mut idx = 0;

    while idx < arr.len() {
        print!("L{}: ", level);

        // 현 레벨의 노드들을 한 줄에 출력
        let end = (idx + nodes_in_level).min(arr.len());
        for i in idx..end {
            print!("[{:?}] ", arr[i]);
        }
        println!();

        // 다음 레벨로
        idx = end;
        nodes_in_level <<= 1; // *2
        level += 1;
    }
}

/// ascending == true  → 오름차순 결과  
/// ascending == false → 내림차순 결과
pub fn heap_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    if arr.len() <= 1 { return; }
    build_heap(arr, ascending);
    let mut end = arr.len() - 1;
    while end > 0 {
        arr.swap(0, end);
        heapify(&mut arr[..end], 0, ascending);
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_check(){
        let original = vec![5, 4, 3, 2, 1];

        // 오름차순 테스트
        let mut asc = original.clone();
        heap_sort(&mut asc, true);
        assert!(is_sorted(&asc));
        assert!(same_elems(&original, &asc));

        // 내림차순 테스트
        let mut desc = original.clone();
        heap_sort(&mut desc, false);
        assert!(is_desc_sorted(&desc));
        assert!(same_elems(&original, &desc));
    }
    
    fn is_sorted<T: Ord>(a: &[T]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }
    fn is_desc_sorted<T: Ord>(a: &[T]) -> bool {
        a.windows(2).all(|w| w[0] >= w[1])
    }
    fn same_elems<T: Ord + std::fmt::Debug + Clone>(a: &[T], b: &[T]) -> bool {
        let mut x = a.to_vec(); x.sort();
        let mut y: Vec<T> = b.to_vec(); y.sort();
        x == y
    }
}