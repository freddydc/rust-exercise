// vectors2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!

pub fn run_vectors2() {
    let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();

    let vec_loop_data = vec_loop(v.clone());
    let vec_map_data = vec_map(&v);

    println!("Vec using loop {:?}", vec_loop_data);
    println!("Vec using map {:?}", vec_map_data);
}

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for num in v.iter_mut() {
        // dereference `*num`
        *num *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &[i32]) -> Vec<i32> {
    v.iter().map(|num| num * 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
