use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum ShellMode {
    Operational,
    Configuration,
}
