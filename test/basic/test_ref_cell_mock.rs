
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTrakcer<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTrakcer<'a, T> where T: Messenger {

    pub fn new(messenger: &T, max: usize) -> LimitTrakcer<T> {
        LimitTrakcer {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f64 / self.max as f64;
        if percent_of_max >= 0.75 && percent_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percent_of_max >= 0.9 && percent_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up 90% of your quota!");
        } else if percent_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }

    fn send(&self, msg: &str) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;
    use std::borrow::ToOwned;
    use std::cell::RefCell;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.send_messages.borrow_mut().push(message.to_owned());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTrakcer::new(&mock_messenger, 100);
        limit_tracker.set_value(75);

        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
    }
}