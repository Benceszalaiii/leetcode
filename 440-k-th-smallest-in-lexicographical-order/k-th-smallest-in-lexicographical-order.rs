impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut value = 1;
        let mut n_remaining = k;

        while n_remaining > 1 {
            let distance = distance_to_next_sibling(value, n);
            if n_remaining > distance {
                // Move to right sibling.
                n_remaining -= distance;
                value += 1;
            } else {
                // Descend from current node.
                n_remaining -= 1;
                value *= 10;
            }
        }
        value
    }
}

fn distance_to_next_sibling(val: i32, n: i32) -> i32 {
    let mut right_sibling = val + 1;
    let mut current_node = val;
    let mut count = 0;

    while current_node <= n {
        count += (n - current_node + 1).min(right_sibling - current_node);
        if let Some(sib) = right_sibling.checked_mul(10) {
            right_sibling = sib;
            current_node *= 10;
        } else {
            break;
        }
    }
    count
}