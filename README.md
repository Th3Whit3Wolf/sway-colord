# Sway Color Daemon

<p align="center">
  <a href="https://github.com/Th3Whit3Wolf/sway-colord/actions?query=workflow%3A%22Continuous+Integration%22">
    <img src="https://github.com/Th3Whit3Wolf/sway-colord/workflows/Continuous%20Integration/badge.svg?branch=main" alt="rust stable build">
  </a>
  <a href="https://github.com/Th3Whit3Wolf/sway-colord/blob/master/LICENSE">
    <img alt="GitHub" src="https://img.shields.io/github/license/Th3Whit3Wolf/sway-colord">
  </a>
  <a href="http://makeapullrequest.com">
    <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg">
  </a>
  <br>
  <i>Color daemon for sway that automatically change light and dark themes based on the time of day</i>
</p>

## What is currently supported?

You change choose between settings a rigid timer (change occurs at 7Am & 7PM) or a solar timer (change occurs at sunrise and sunset)

Currently the following applications will switch between light and dark colorschemes automatically:

* Alacritty
* Bat
* GTK (on sway)
  * GTK Theme
  * Icon Theme
  * Cursor Theme
  * Font Name
* Lighting
  * Monitor
  * Keyboard
* Mako
* Neovim(v0.5+) with [Dusk-til-Dawn.nvim](https://github.com/Th3Whit3Wolf/Dusk-til-Dawn.nvim)
* Spotify with [spicetify](https://github.com/khanhas/spicetify-cli)
* Vscode

Sway Color Daemon also creates `/tmp/sway-colord/dawn` & `/tmp/sway-colord/dusk` files
that can be read to find out when the next timechange is from a shell script

## Install

### Alpine Linux

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-x86_64.apk -o sway-colord-0.1.0-x86_64.apk
sudo apk add --allow-untrusted sway-colord-0.1.0-x86_64.apk
```

### Arch Linux

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-x86_64.pkg.tar.zst -o sway-colord-0.1.0-x86_64.pkg.tar.zst
sudo pacman -U sway-colord-0.1.0-x86_64.pkg.tar.zst
```

### Debian/Ubuntu

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord_0.1.0_amd64.deb -o sway-colord_0.1.0_amd64.deb
sudo dpkg -i sway-colord_0.1.0_amd64.deb
```

### Gentoo

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-x86_64.ebuild -o sway-colord-0.1.0-x86_64.ebuild 
ebuild sway-colord-0.1.0-x86_64.ebuild compile
ebuild sway-colord-0.1.0-x86_64.ebuild install
```

### RHEL

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-x86_64.rpm -o sway-colord-0.1.0-x86_64.rpm
sudo rpm –i sway-colord.rpm
```

### Other Linux x86-64bit distro

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-linux-amd64 -o sway-colord
sudo mv sway-colord /usr/bin/
```

### Linux ARM

```shell
curl -Lj https://github.com/Th3Whit3Wolf/sway-colord/releases/download/v0.1.0/sway-colord-0.1.0-linux-amd64 -o sway-colord
sudo mv sway-colord /usr/bin/
```

## Goals

### Theme Switching

- [ ] Atom
- [ ] Kitty

### UI

- [ ] tui
- [ ] gui

## Instructions

Create file `~/.config/sway-colord/config.ron`

```ron
(
    timechange: Solar(52.4045, 0.5613),
    alacritty: Some(
		Alacritty(
			dark_theme: Some("dark"),
			light_theme: Some("light")
		)
    ),
    bat: Some(
		Bat(
			dark_theme: Some("OneHalfDark"),
			light_theme: Some("OneHalfLight")
		)
    ),
    gsettings: Some(
		GSettings(
			dark_gtk_theme: Some("Space-Dark"),
			dark_icon_theme: Some("Space-Dark"),
			dark_cursor_theme: None,
			dark_font_name: None,
			light_gtk_theme: Some("Space-Light"),
			light_icon_theme: Some("Space-Light"),
			light_cursor_theme: None,
			light_font_name: None,
		)
    ),
    lighting: Some(
		Lighting(
			monitor: Some(
				Monitor(
					device: "amdgpu_bl0",
					dark_perc: 20,
					light_perc: 50
				)
			),
			keyboard: Some(
				Keyboard(
					device: "asus::kbd_backlight",
					dark_perc: 34,
					light_perc: 0
				)
			)
		)
    ),
    mako: Some(
		Mako(
	    	dark_theme: Some("Dark"),
	    	light_theme: Some("Light")
		)
    ),
    vscode: Some(
		VSCode(
	    	dark_theme: Some("Spacemacs - dark"),
	    	light_theme: Some("Spacemacs - light")
		)
    )
)
```

## How to get waybar theme to change automatically?

Waybar will change automatically with the gtk theme if you [make waybar follow the gtk theme](https://github.com/Alexays/Waybar/wiki/Styling#making-waybar-follow-the-gtk-theme).

## How to get my other utilities to switch themes with sway-colord?

You can make an executable script for any utility that accepts a theme or config as a parameter. Place it in `~/.local/bin` and make sure `~/.local/bin` is in your `$PATH`. Here is an example for wofi.

```shell
#!/usr/bin/env bash

# place this in ~/.local/bin/wofi
dawn=$(cat /tmp/sway-colord/dawn)
dusk=$(cat /tmp/sway-colord/dusk)
now=$(date +%H:%M:%S)

if [[ "$now" < "$dawn" ]] || [[ "$now" > "$dusk" ]]; then
    # Dark Theme
    /usr/bin/wofi -s ~/.config/wofi/dark.css
else
    # Light Theme
    /usr/bin/wofi -s ~/.config/wofi/light.css
fi
```

