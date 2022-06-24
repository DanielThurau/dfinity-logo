fn main() {
    let welcome_art = dfinity_logo();
    println!("\n\n{}", welcome_art);
}

pub fn dfinity_logo() -> String {
    if atty::is(atty::Stream::Stdout) {
        //MacOS's Terminal.app does not support Truecolor (RGB-colored characters) properly.
        //Therefore we use xterm256 coloring when the program is running on macos
        if std::env::consts::OS == "macos" {
            include_str!("../assets/dfinity-color-xterm256.aart").to_string()
        } else {
            include_str!("../assets/dfinity-color.aart").to_string()
        }
    } else {
        include_str!("../assets/dfinity-nocolor.aart").to_string()
    }
}