use std::cmp::Ordering;

/*
/// My code form coursera Data-Structures-and-Algorithms-Specialization
/// - Algorithmic-Toolbox
/// - Week-4-Divide-and-conquer
*/

/// partition3 is a helper function for ImprovingQuickSort function
// realizes 3-way partition to handle few equal elements in slice
fn partition3(sequence: &mut [i32], l: usize, r: usize) -> (usize, usize) {
    let pivot = sequence[l];
    let mut m1 = l; // We initiate m1 to be the part that is less than the pivot
    let mut m2 = r; // The part that is greater than the pivot
    let mut i = l;

    while i <= m2 {
        match sequence[i].cmp(&pivot) {
            Ordering::Less => {
                (sequence[m1], sequence[i]) = (sequence[i], sequence[m1]);
                m1 += 1;
                i += 1;
            }
            Ordering::Greater => {
                (sequence[m2], sequence[i]) = (sequence[i], sequence[m2]);
                m2 -= 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    (m1, m2)
}

/// improving_quicksort returns given sequence sorted in non-decreasing order.
fn improving_quicksort(sequence: &mut [i32], l: usize, r: usize) {
    // Here we have to implement 3-way partition
    if l >= r {
        return;
    }
    let (m1, m2) = partition3(sequence, l, r);

    improving_quicksort(sequence, l, m1 - 1);
    improving_quicksort(sequence, m2 + 1, r);
}

fn main() {
    let mut numbers: [i32; 20] = rand::random();
    println!("=== Before ===");
    println!("{:?}", numbers);

    println!("=== QuickSort ===");
    let size = numbers.len() - 1;
    improving_quicksort(&mut numbers, 0, size);
    println!("{:?}", numbers);
}
