// struct Emp {
//     id: i8
// }

// impl Emp {
//     fn set_some(id: i8) -> Self {
//         Self {
//             id
//         }
//     }
//     fn show_id(&self) {
//         println!("{}", self.id)
//     }
    
// }


fn main() {
    // Emp::set_some(2).show_id();
    let roman = String::from("MCMXCIV");
    let mut result: i32 = 0;
    let mut prev_value: i32 = 0;
    fn get_roman_value(current_roman: char) -> i32 {
        match current_roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0
        }
    }
    for curr_roman in roman.chars().rev() {
        let current_value = get_roman_value(curr_roman);
        if current_value < prev_value {
            result -= current_value
        } else {
            result += current_value
        }
        prev_value = current_value;
    }
    println!("{}", result);
}