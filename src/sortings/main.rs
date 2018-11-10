mod bubble;

fn main() {
    println!("from sortings");
    let arr = vec![3, 2, 1, 0];

    println!("{:?}", bubble::bubble_sort(&arr));
}
