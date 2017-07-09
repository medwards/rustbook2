// TODO: str vs String?
fn main() {
    let lines = [
        "Twelve Ladies Dancing".to_string(),
        "Eleven Lords a-Leaping".to_string(),
        "Ten Drummers Drumming".to_string(),
        "Nine Pipers Piping".to_string(),
        "Eight Maids a-Milking".to_string(),
        "Seven Swans a-Swimming".to_string(),
        "Six Geese a-Laying".to_string(),
        "Five Golden Rings".to_string(),
        "Four Calling Bird".to_string(),
        "Three French Hens".to_string(),
        "Two Turtle Doves".to_string(),
        "a Partridge in a Pear Tree.".to_string(),
    ];

    for day in 1..13 {
        println!("On the {} day of Christmas my true love gave to me", ordinal(day));
        for index in (12 - day)..12 {
            println!("{}", lines[index as usize]);
        }
        println!("");
    }
}

fn ordinal(x: u32) -> String {
    match x {
        1 => "First".to_string(),
        2 => "Second".to_string(),
        3 => "Third".to_string(),
        4 => "Fourth".to_string(),
        5 => "Fifth".to_string(),
        6 => "Sixth".to_string(),
        7 => "Seventh".to_string(),
        8 => "Eighth".to_string(),
        9 => "Ninth".to_string(),
        10 => "Tenth".to_string(),
        11 => "Eleventh".to_string(),
        12 => "Twelfth".to_string(),
        x => format!("{}th", x),
    }
}
