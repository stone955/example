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
        // while
        let mut x = 1;

        while x < 1000 {
            x *= 2;
            println!("x = {}", x);
        }

        // loop => while true
        let mut y = 1;
        loop {
            y *= 2;
            println!("y = {}", y);
            if y > 1000 {
                break;
            }
        }

        // for loop
        for z in 0..10 {
            if z % 2 == 0 {
                continue;
            }
            println!("z = {}", z);
        }

        for (pos, val) in (20..30).enumerate() {
            println!("pos = {}, value= {}", pos, val);
        }
    }
}
