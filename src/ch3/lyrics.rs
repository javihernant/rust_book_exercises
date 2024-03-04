pub fn play_song() {
    let ordinals = vec![
        "first", "second", "third", "fourth", "fith", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelveth",
    ];
    let presents = vec![
        "partridge in a pear tree!",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas
My true love sent to me",
            ordinals[i]
        );
        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                print!("And a ")
            }
            if i == 0 && j == 0 {
                print!("A ")
            }
            println!("{}", presents[j]);
        }
        if i < 11 {
            println!("");
        }
    }
}
