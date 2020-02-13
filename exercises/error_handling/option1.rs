// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last: i32 = match list.pop() {
        Some(item) => item,
        None => -1,
    };
    println!("The last item in the list is {:?}", last);

    let second_to_last = match list.pop() {
        Some(item) => item,
        None => -1,
    };
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
    if last == -1 || second_to_last == -1 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
