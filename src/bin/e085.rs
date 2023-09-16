// Idiom #85 Check if integer addition will overflow
// Write boolean function addingWillOverflow which takes two integers x, y and return true if (x+y) overflows.

// An overflow may be above the max positive value, or below the min negative value.

fn adding_will_overflow(x: usize, y: usize) -> bool {
    x.checked_add(y).is_none()
}

fn main() {
    {
        let (x, y) = (2345678, 9012345);

        let overflow = adding_will_overflow(x, y);

        println!(
            "{} + {} {}",
            x,
            y,
            if overflow {
                "overflows"
            } else {
                "doesn't overflow"
            }
        );
    }
    {
        let (x, y) = (2345678901, 9012345678);

        let overflow = adding_will_overflow(x, y);

        println!(
            "{} + {} {}",
            x,
            y,
            if overflow {
                "overflows"
            } else {
                "doesn't overflow"
            }
        );
    }
    {
        let (x, y) = (2345678901234, 9012345678901);

        let overflow = adding_will_overflow(x, y);

        println!(
            "{} + {} {}",
            x,
            y,
            if overflow {
                "overflows"
            } else {
                "doesn't overflow"
            }
        );
    }
    {
        let (x, y) = (23456789012345678, 90123456789012345);

        let overflow = adding_will_overflow(x, y);

        println!(
            "{} + {} {}",
            x,
            y,
            if overflow {
                "overflows"
            } else {
                "doesn't overflow"
            }
        );
    }
    {
        let (x, y) = (12345678901234567890, 9012345678901234567);

        let overflow = adding_will_overflow(x, y);

        println!(
            "{} + {} {}",
            x,
            y,
            if overflow {
                "overflows"
            } else {
                "doesn't overflow"
            }
        );
    }
}
