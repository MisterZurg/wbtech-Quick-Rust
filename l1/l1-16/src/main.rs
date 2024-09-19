use std::cmp::Ordering;

fn binary_search(nums: &[i32], target: i32) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;

    while l < r {
        let m = (l + r) / 2;
        match nums[m].cmp(&target) {
            Ordering::Equal => return m as i32,
            Ordering::Less => {
                l = m + 1;
            }
            Ordering::Greater => {
                r = m;
            }
        }
    }

    -1
}

fn main() {
    let mut nums = [1, 228, 13, 42, 64, 3, 6, 70, 1488];
    println!("Raw: {:?}", nums);
    nums.sort();
    println!("Sorted: {:?}", nums);

    println!("got 228 at idx {}", binary_search(&nums, 228));

    println!("non exist number {}", binary_search(&nums, 4269));
}
