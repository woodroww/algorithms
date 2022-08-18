
fn best_sum(target_sum: u16, numbers: &Vec<u16>) -> Option<Vec<u16>> {

    if target_sum == 0 {
        return None;
    }
    let mut shortest: Option<Vec<u16>> = None;

    for num in numbers {
        let mut combo = None;
        if target_sum > *num {
            let remainder = target_sum - num;
            combo = best_sum(remainder, numbers);
        }
        if let Some(mut remainder_combo) = combo {
            remainder_combo.push(*num); 
            if let Some(short) = &shortest {
                if remainder_combo.len() < short.len() {
                    shortest = Some(remainder_combo.to_vec());
                }
            } else {
                shortest = Some(remainder_combo.to_vec());
            }
        }
    }
    shortest
}



fn main() {
    println!("Hello, world!");
}

#[test]
fn a() {
    assert_eq!(best_sum(7, &vec![5, 3, 4, 7]), Some(vec![7]));
}

#[test]
fn b() {
    assert_eq!(best_sum(8, &vec![2, 3, 5]), Some(vec![5, 3]));
}

#[test]
fn c() {
    assert_eq!(best_sum(8, &vec![1, 4, 5]), Some(vec![4, 4]));
}


