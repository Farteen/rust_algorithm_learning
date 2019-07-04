
#[test]
fn test_bubble_sort() {
    let arr:Vec<i32> = [1, 3, 2].to_vec();
    let result = bubble_sort(&arr);
    for i in result {
        println!("{}", i);
    }
}

fn bubble_sort(arr: &Vec<i32>) -> Vec<i32> {

    let mut target = arr.clone();
    let length = target.len();
    for outerIndex in 0..length-1 {
        for innerIndex in outerIndex..length - 1 {
            let leftValue = target[innerIndex];
            let rightValue = target[innerIndex + 1];
            if leftValue > rightValue {
                target[innerIndex] = rightValue;
                target[innerIndex + 1] = leftValue;
            }
        }
    }
    target
}

fn bubble_sort_from_end(arr: &Vec<i32>) -> Vec<i32> {
    let mut target = arr.clone();
    let length = target.len();
    for i in (0..length).rev() {
        
    }


    target
}
