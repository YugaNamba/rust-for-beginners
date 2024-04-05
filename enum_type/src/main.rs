enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let color = Color::Red;
    println!("Color: {}", color_to_str(&color));
}

fn color_to_str(color: &Color) -> &str {
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
        Color::Yellow => "#FFFF00",
    }
}
