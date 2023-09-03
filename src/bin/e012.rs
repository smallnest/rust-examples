// Idiom #12 Check if list contains a value
// Check if the list contains the value x.
// list is an iterable finite container.

fn main() {
    let list = [10, 40, 30];

    {
        let num = 30;

        if list.contains(&num) {
            println!("{:?} contains {}", list, num);
        } else {
            println!("{:?} doesn't contain {}", list, num);
        }
    }

    {
        let num = 42;

        if list.contains(&num) {
            println!("{:?} contains {}", list, num);
        } else {
            println!("{:?} doesn't contain {}", list, num);
        }
    }

    {
        let x = 30;

        if (&list).into_iter().any(|v| v == &x) {
            println!("{:?} contains {}", list, x);
        } else {
            println!("{:?} doesn't contain {}", list, x);
        }
    }

    {
        let x = 30;

        if  list.binary_search(&x).is_ok() {
            println!("{:?} contains {}", list, x);
        } else {
            println!("{:?} doesn't contain {}", list, x);
        }
    }

    {
        let x = 30;

        if  list.iter().any(|v| v == &x) {
            println!("{:?} contains {}", list, x);
        } else {
            println!("{:?} doesn't contain {}", list, x);
        }

    }
}
