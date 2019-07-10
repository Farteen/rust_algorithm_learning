#[test]
fn test_refutability() {
    let some_option_value: Option<i32> = None;

//    let Some(x) = some_option_value;
//    println!("{}", x);
}

#[test]
fn test_some() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(5) => println!("Got 50"),
        Some(y) => println!("Got {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}",x ,y);
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn desctruct_point() {
    let points = vec![
        Point{x: 0, y: 0},
        Point{x: 1, y: 1},
        Point{x: 2, y: 2},
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point{x, y}| x * x + y * y )
        .sum();
}

#[test]
fn test_underscore() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cant overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

enum Message {
    Hello { id: i32},
}

#[test]
fn test_at_operator() {
    let msg = Message::Hello { id: 5};

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("found an id in range {}", id_variable);
        },
        Message::Hello { id: 10...12} => {
            println!("found an id in another range");
        }
        Message::Hello {id} => {
            println!("default match");
        }
    }
}

#[test]
fn test_ref() {
    let robot_name = &Some(String::from("Bors"));

    match robot_name {
        &Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is {:?}", robot_name);
}