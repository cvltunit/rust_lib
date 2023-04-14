use split_work::split_work;

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let result = split_work(input, |x| x * 2, 2);
    println!("{:?}", result); // Выводит "[2, 4, 6, 8, 10]"
}