pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

//pub struct Screen<T: Draw> {
//    pub components: Vec<T>,
//}

//impl<T> Screen<T> where T: Draw{
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//}

pub struct Point  {
    pub x: i32,
    pub y: i32,
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("this is circle impl");
    }
}

pub struct Rectangle {
    lt: Point,
    width: i32,
    height: i32,
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("this is rectangle impl");
    }
}

#[test]
fn test_screen_draw() {
    let circle = Circle {
        center: Point {
            x: 50,
            y: 50,
        },
        radius: 50,
    };

    let rect = Rectangle {
        lt: Point {
            x: 10,
            y: 10,
        },
        width: 100,
        height: 100,
    };

    let screen = Screen {
        components: vec![
            Box::new(circle),
            Box::new(rect),
        ],
    };
    screen.run();
}