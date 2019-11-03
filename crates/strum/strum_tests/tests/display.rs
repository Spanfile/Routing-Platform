extern crate strum;
#[macro_use]
extern crate strum_macros;

#[derive(Debug, Eq, PartialEq, EnumString, Display)]
enum Color {
    #[strum(to_string = "RedRed")]
    Red,
    #[strum(serialize = "b", to_string = "blue")]
    Blue { hue: usize },
    #[strum(serialize = "y", serialize = "yellow")]
    Yellow,
    #[strum(default = "true")]
    Green(String),
}

#[test]
#[ignore]
fn to_blue_string() {
    assert_eq!(String::from("blue"), format!("{}", Color::Blue { hue: 0 }));
}

#[test]
#[ignore]
fn to_yellow_string() {
    assert_eq!(String::from("yellow"), format!("{}", Color::Yellow));
}

#[test]
#[ignore]
fn to_red_string() {
    assert_eq!(String::from("RedRed"), format!("{}", Color::Red));
}

#[derive(Display, Debug, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Brightness {
    DarkBlack,
    Dim {
        glow: usize,
    },
    #[strum(serialize = "bright")]
    BrightWhite,
}

#[test]
#[ignore]
fn brightness_to_string() {
    assert_eq!(
        String::from("dark_black"),
        Brightness::DarkBlack.to_string().as_ref()
    );
    assert_eq!(
        String::from("dim"),
        Brightness::Dim { glow: 0 }.to_string().as_ref()
    );
    assert_eq!(
        String::from("bright"),
        Brightness::BrightWhite.to_string().as_ref()
    );
}
