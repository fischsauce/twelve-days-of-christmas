fn main() {
    let days = 12;

    let lyrics = [
        "a partridge in a pear tree.",
        "2 turtle doves and", 
        "3 French hens,", 
        "4 calling birds,", 
        "5 gold rings,", 
        "6 geese a-laying,",
        "7 swans a-swimming,",
        "8 maids a-milking,",
        "9 ladies dancing,",
        "10 lords a-leaping,",
        "11 pipers piping,",
        "12 drummers drumming,"];

    let mut previous_verses = [""; 12];

    for day in 0..days {

        let lyric = lyrics[day].trim();

        println!("\n\nOn the {} day of Christmas, my true love sent to me {}", (day +1), lyric);

        for element in previous_verses.iter().rev() {
            println!("{}", element.trim());
        }

        previous_verses[day] = lyric;
    }
}
