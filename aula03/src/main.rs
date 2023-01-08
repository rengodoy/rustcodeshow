fn main() {
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}", numbers);
    let chars = ['a', '👌', '\u{1F480}'];
    println!("{:?}", chars[2]);
    println!("{:?}", chars);
    {
        let mut numbers = [1.0, 2.0, 3.5];
        numbers[0] = 3.0;
        numbers[2] = 4.5; // ou numbers = [3.0, 2.0, 4.5]
        println!("{:?}", &numbers[0..1]);
        println!("{:?}", &numbers[1..]);
    }
}
