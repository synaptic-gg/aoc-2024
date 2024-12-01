mod input;
fn main() {
    let mut array1 = input::array1();
    let mut array2 = input::array2();
    println!("{:?}",array1);
    array1.sort();
    array2.sort();
    let mut distance: [i32; 1000] = [1; 1000];
    for n in 0..1000 {
        let r = array1[n] - array2[n];
        distance[n] = r.abs();
        println!("{}", r.abs())
    }
    println!("{}", distance.iter().sum::<i32>())
}
