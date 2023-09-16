// Idiom #86 Check if integer multiplication will overflow
// Write the boolean function multiplyWillOverflow which takes two integers x, y and returns true if (x*y) overflows.
//
// An overflow may reach above the max positive value, or below the min negative value

fn main() {
    {
        let (x, y) = (2345, 6789);

        let overflow = multiply_will_overflow(x, y);

        println!(
            "{} * {} {}",
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
        let (x, y) = (2345678, 9012345);

        let overflow = multiply_will_overflow(x, y);

        println!(
            "{} * {} {}",
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

        let overflow = multiply_will_overflow(x, y);

        println!(
            "{} * {} {}",
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

fn multiply_will_overflow(x: i64, y: i64) -> bool {
    x.checked_mul(y).is_none()
}
