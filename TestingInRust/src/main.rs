struct Rectangle {
    w: i32,
    h: i32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub fn greet(name: &str) -> String {
    format!("Hi, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            w:8,
            h:7
        };
        let smaller = Rectangle {
            w:5,
            h:1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            w:8,
            h:7
        };
        let smaller = Rectangle {
            w:5,
            h:1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greet_Renan() {
        let res = greet("Renan");
        assert!(res.contains("Hello"), "Message does not contains the hello special word! (res = {})", res);
    }
}

//use: cargo test  || on terminal.


