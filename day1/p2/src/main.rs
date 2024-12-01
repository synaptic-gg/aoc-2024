mod input;
fn main() {
    const LENGTH: usize = 1000;
    let mut array1 = input::array1();
    let mut array2 = input::array2();
    array1.sort();
    array2.sort();
    let mut distance: [i32; LENGTH] = [1; LENGTH];
    for n in 0..LENGTH {
        let val = array1[n];
        let count: i32 = array2
            .iter()
            .filter(|n| **n == val)
            .count()
            .try_into()
            .unwrap();
        let r = (array1[n]) * count;
        distance[n] = r.abs();
        println!("{} {}", val, count)
    }
    println!("{}", distance.iter().sum::<i32>())
}
