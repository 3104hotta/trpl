fn main() {
    print_labled_measurement(5, 'h');
}

fn print_labled_measurement(value: i32, unit_label: char) {
    println!("The measurement of is: {value}{unit_label}");
}
