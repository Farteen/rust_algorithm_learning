
#[test]
fn test_merge_sort() {
    let arr = [1, 3, 5, 2, 4].to_vec();
    let result = merge_sort(arr);
    for i in result {
        println!("{}",i);
    }
}

fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut target = arr.clone();

    target
}

fn sort_process(arr: &[i32], l: i32, r: i32) {
    if l == r {
        return;
    }

}