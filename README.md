# Sway Color Daemon

This is a WIP daemon for sway to automatically change light and dark themes based on the time of day.

## What is currently supported?

Currently the following applications will switch between light and dark colorschemes automatically:

* Alacritty
* Vscode
* GTK
  * GTK Theme
  * Icon Theme
  * Cursor Theme
  * Font Name

## How to get waybar theme to change automatically?

Waybar will change automatically with the gtk theme if you [make waybar follow the gtk theme](https://github.com/Alexays/Waybar/wiki/Styling#making-waybar-follow-the-gtk-theme).

## Instructions

Create file `~/.config/sway-colord/config.ron`

```ron
    alacritty: Alacritty(
        dark_theme: Some("dark"),
        light_theme: Some("light")
    ),
    gsettings: GSettings(
        dark_gtk_theme: Some("Space-Dark"),
        dark_icon_theme: Some("Space-Dark"),
        dark_cursor_theme: None,
        dark_font_name: None,
        light_gtk_theme: Some("Space-Light"),
        light_icon_theme: Some("Space-Light"),
        light_cursor_theme: None,
        light_font_name: None,
    ),
    vscode: VSCode(
        dark_theme: Some("Spacemacs - dark"),
        light_theme: Some("Spacemacs - light")
    )
)
```