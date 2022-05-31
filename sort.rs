fn main() {
    let bubble = BubbleSort::new(vec![5, 4, 3, 1, 2, 1]);

    println!("sorted. {:?}", bubble_sort(bubble.data));
}

struct BubbleSort<T> {
    data: Vec<T>,
}
impl<T> BubbleSort<T>
where
    T: Ord + Copy,
{
    fn new(data: Vec<T>) -> BubbleSort<T> {
        BubbleSort { data: data }
    }
}

fn bubble_sort<T: Ord + Copy>(mut arr: Vec<T>) -> Vec<T> {
    let mut length: usize = arr.len();

    while length > 1 {
        for index in 1..length {
            if arr[index] < arr[index - 1] {
                let temp = arr[index];
                arr[index] = arr[index - 1];
                arr[index - 1] = temp;
            }
        }
        length = length - 1
    }
    arr
}
