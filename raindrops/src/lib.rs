pub fn raindrops(n: u32) -> String {
    let mut string = "".to_string();

    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return format!("{}", n);
    }

    if n % 3 == 0 {
        string += "Pling";
    }

    if n % 5 == 0 {
        string += "Plang";
    }

    if n % 7 == 0 {
        string += "Plong";
    }

    return string;
}
