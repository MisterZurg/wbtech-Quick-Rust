use std::collections::HashSet;

fn intersection_of_two_sets(f: Vec<i32>, s: Vec<i32>) -> HashSet<i32> {
    let fs: HashSet<i32> = f.into_iter().collect();
    let ss: HashSet<i32> = s.into_iter().collect();

    fs.intersection(&ss)
        .cloned()
        .collect()
}

fn main() {
    let first_set = vec![1, 9, 2, 7];
    let second_set = vec![5, 9, 6, 7];

    println!("{:?}", intersection_of_two_sets(first_set, second_set))
}
