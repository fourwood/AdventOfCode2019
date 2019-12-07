fn test_criteria_one(number: u32) -> bool {
    (number >= 100000) && (number <= 999999)
}

fn test_criteria_two(number: u32, start: u32, end: u32) -> bool {
    (number >= start) && (number <= end)
}

fn test_criteria_three(number: u32) -> bool {
    let e0 = number % 10;
    let e1 = (number / 10) % 10;
    let e2 = (number / 100) % 10;
    let e3 = (number / 1000) % 10;
    let e4 = (number / 10000) % 10;
    let e5 = number / 100000;

    (e5 == e4 && e4 != e3)
        || (e5 != e4 && e4 == e3 && e3 != e2)
        || (e4 != e3 && e3 == e2 && e2 != e1)
        || (e3 != e2 && e2 == e1 && e1 != e0)
        || (e2 != e1 && e1 == e0)
}

fn test_criteria_four(number: u32) -> bool {
    let e0 = number % 10;
    let e1 = (number / 10) % 10;
    let e2 = (number / 100) % 10;
    let e3 = (number / 1000) % 10;
    let e4 = (number / 10000) % 10;
    let e5 = number / 100000;

    (e5 <= e4) && (e4 <= e3) && (e3 <= e2) && (e2 <= e1) && (e1 <= e0)
}

fn fourdotone(start: u32, end: u32) -> u32 {
    let mut n_matching = 0;
    for i in start..end {
        if test_criteria_one(i)
            && test_criteria_two(i, start, end)
            && test_criteria_three(i)
            && test_criteria_four(i)
        {
            n_matching += 1;
        }
    }

    n_matching
}

//fn fourdottwo() {}

fn main() {
    let n_matching = fourdotone(124075, 580769);
    println!("Number of matching passwords: {}", n_matching);
}
