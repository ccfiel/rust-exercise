

fn verse(n: usize) -> String {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
        ];

    let mut gifts = [
            "a Partridge in a Pear Tree.\n",
            "two Turtle Doves",
            "three French Hens",
            "four Calling Birds",
            "five Gold Rings",
            "six Geese-a-Laying",
            "seven Swans-a-Swimming",
            "eight Maids-a-Milking",
            "nine Ladies Dancing",
            "ten Lords-a-Leaping",
            "eleven Pipers Piping",
            "twelve Drummers Drumming"    
        ];
        
    
    let last_line  = format!(", and {}", gifts[0]);
    let statement = format!("On the {} day of Christmas my true love gave to me, " , days[n - 1]);
    let reverse_string = &mut gifts[1..n];
    reverse_string.reverse();
    let combine = format!("{}", reverse_string.join(", "));
    
    if n == 1 {
        format!("{}{}", statement, gifts[0]).to_string()
    } else {
        format!("{}{}{}",statement, combine, last_line).to_string()
    }
}

fn verses(n: usize, m: usize) {
    for number in n..m + 1 {
        println!("{}", verse(number));
    }
}

fn  main() {
    verses(1, 12);
}
