use super::Config;
use ron::de::from_str;

const CONFIG: &str = "(
    alacritty: Alacritty(
        dark_theme: Some(\"dark\"),
        light_theme: Some(\"light\")
    ),
    gsettings: GSettings(
        dark_gtk_theme: Some(\"Space-Dark\"),
        dark_icon_theme: Some(\"Space-Dark\"),
        dark_cursor_theme: None,
        dark_font_name: None,
        light_gtk_theme: Some(\"Space-Light\"),
        light_icon_theme: Some(\"Space-Light\"),
        light_cursor_theme: None,
        light_font_name: None,
    ),
    lighting: Lighting(
        monitor: Some(
            Monitor(
                device: \"amdgpu_bl0\",
                light_perc: 50,
                dark_perc: 20
            )
        ),
        keyboard: Some(
            Keyboard(
                device: \"asus::kbd_backlight\",
                light_perc: 0,
                dark_perc: 34
            )
        )
    ),
    vscode: VSCode(
        dark_theme: Some(\"Spacemacs - dark\"),
        light_theme: Some(\"Spacemacs - light\")
    )
)";


#[test]
fn test_read_config() {
    let conf: Config = from_str(CONFIG).unwrap();
    if !conf.alacritty.is_some() {
        panic!("Alacritty not detected")
    }
    if !conf.gsettings.is_some() {
        panic!("GSettings not detected")
    }
    if !conf.vscode.is_some() {
        panic!("VSCode not detected")
    }
}
