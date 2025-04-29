use std::collections::HashMap;

fn group_temps(temps: &[f64]) -> HashMap<std::ops::Range<i32>, Vec<f64>> {
    let mut groups: HashMap<std::ops::Range<i32>, Vec<f64>> = HashMap::new();

    for temp in temps {
        let key = (*temp as i32 / 10) * 10..(*temp as i32 / 10 + 1) * 10;

        groups
            .entry(key)
            .or_insert(Vec::new())
            .push(*temp);
    }

    groups
}

fn main() {
    // From example
    let temps = &[-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let groups = group_temps(temps);

    for (range, values) in groups {
        println!("{:?}: {:?}", range, values);
    }
}