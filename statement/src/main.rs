fn main() {
    // if statement
    {
        let temperature = 25; // 5 10
        if temperature > 30 {
            println!("{} ℃ really hot", temperature);
        } else if temperature < 10 {
            println!("{} ℃ really cold", temperature);
        } else {
            println!("{} ℃ OK", temperature);
        }

        let day = if temperature > 30 {
            "really hot"
        } else if temperature < 10 {
            "really cold"
        } else {
            "OK"
        };
        println!("Today is {}", day);
    }

    // while and loop
    {

    }
}
