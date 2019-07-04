
#[test]
fn test_insertion_sort() {
    let arr = [1, 3, 5, 2, 4].to_vec();
    let result = insertion_sort(arr);
    for i in result {
        println!("{}", i);
    }
}

fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut target = arr.clone();
    let length = target.len();

    for i in 1..length {
        for j in (1..i).rev() {
            let leftValue = target[j - 1];
            let currentValue = target[j];
            if leftValue > currentValue {
                target[j - 1] = currentValue;
                target[j] = leftValue;
            }
        }
    }

    target
}

