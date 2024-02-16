#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let (res, _) = s.chars().fold((0 as i32, 0 as i32), |(acc, prev), x| {
        let v: i32 = match x {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if v > prev {
            (acc + v - prev - prev, v)
        } else {
            (acc + v, v)
        }
    });
    res
}
