const COLOR_TEXT: &str = "var(--color-text)";
const COLOR_GRAY: &str = "var(--color-subtext)";
const COLOR_BACKGROUND: &str = "var(--color-base)";
const COLOR_BROWN: &str = "var(--color-maroon)";

const COLOR_CYAN: &str = "var(--color-sapphire)";
const COLOR_BLUE: &str = "var(--color-blue)";
const COLOR_PURPLE: &str = "var(--color-lavander)";
const COLOR_MAGENTA: &str = "var(--color-mavue)";
const COLOR_PINK: &str = "var(--color-pink)";

const COLOR_GREEN: &str = "var(--color-green)";
const COLOR_TEAL: &str = "var(--color-teal)";
const COLOR_DARK_ORANGE: &str = "var(--color-yellow)";
const COLOR_BRIGHT_ORANGE: &str = "var(--color-peach)";
const COLOR_RED: &str = "var(--color-red)";

pub fn replace_color(excalidraw_color: &str) -> String {
    match excalidraw_color {
        "#ffffff" => COLOR_BACKGROUND,
        "#868e96" | "#e9ecef" => COLOR_GRAY,
        "#1e1e1e" => COLOR_TEXT,
        "#846358" | "#eaddd7" => COLOR_BROWN,
        "#0c8599" | "#99e9f2" => COLOR_CYAN,
        "#1971c2" | "#a5d8ff" => COLOR_BLUE,
        "#6741d9" | "#d0bfff" => COLOR_PURPLE,
        "#9c36b5" | "#eebefa" => COLOR_MAGENTA,
        "#c2255c" | "#fcc2d7" => COLOR_PINK,
        "#2f9e44" | "#b2f2bb" => COLOR_GREEN,
        "#099268" | "#96f2d7" => COLOR_TEAL,
        "#f08c00" | "#ffec99" => COLOR_DARK_ORANGE,
        "#e8590c" | "#ffd8a8" => COLOR_BRIGHT_ORANGE,
        "#e03131" | "#ffc9c9" => COLOR_RED,
        color => {
            panic!("unknown color: {}", color);
        }
    }
    .to_string()
}
