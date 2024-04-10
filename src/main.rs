fn main() {
    let sequence = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelveth",
    ];

    let lines = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut i = 0;

    'whole_song: loop {
        let day = sequence[i];
        println!("\nOn the {day} day of Christmas my true love sent to me");

        let mut j = 0;

        while j < i {
            let line = lines[j];

            if i > 0 {
                println!("{line}");
            }

            j += 1;
        }

        if i == 0 {
            println!("A patridge in a pear tree");
        } else {
            println!("And a patridge in a pear tree\n");
        }

        if i == sequence.len() - 1 {
            break 'whole_song;
        }

        i += 1;
    }

    println!("we finished the whole song!");
}
