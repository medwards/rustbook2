// TODO: str vs String?
const LYRICS: &'static [&'static str] = &[
    "Twelve Ladies Dancing",
    "Eleven Lords a-Leaping",
    "Ten Drummers Drumming",
    "Nine Pipers Piping",
    "Eight Maids a-Milking",
    "Seven Swans a-Swimming",
    "Six Geese a-Laying",
    "Five Golden Rings",
    "Four Calling Bird",
    "Three French Hens",
    "Two Turtle Doves",
    "a Partridge in a Pear Tree.",
];

fn main() {
    for day in 1..13 {
        println!("On the {} day of Christmas my true love gave to me", ordinal(day));
        for index in (12 - day)..12 {
            println!("{}", LYRICS[index as usize]);
        }
        println!("");
    }
}

fn ordinal(x: u32) -> String {
    match x {
        1 => String::from("First"),
        2 => String::from("Second"),
        3 => String::from("Third"),
        4 => String::from("Fourth"),
        5 => String::from("Fifth"),
        6 => String::from("Sixth"),
        7 => String::from("Seventh"),
        8 => String::from("Eighth"),
        9 => String::from("Ninth"),
        10 => String::from("Tenth"),
        11 => String::from("Eleventh"),
        12 => String::from("Twelfth"),
        x => format!("{}th", x),
    }
}
