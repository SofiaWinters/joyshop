# joyshop

A small application to use joycon as a Photoshop's shortcut launcher on windows 10.

## Usage

1. Connect JoyCon via bluetooth.
2. Run `joyshop.exe`.
3. Edit `settings.json` generated next to `joyshop.exe`.
4. After editting, close and rerun the application.

Some pc doesn't have a bluetooth adapter, so you may need a BlueTooth adapter to use JoyCon by your pc.

## About settings.json

Currently joyshop doesn't have an utility for configuration.
You have to edit settings.json manually.
You can see all available keys for settings.json in [here](https://github.com/SofiaWinters/joyshop/blob/main/src/configuration.rs#L10-L191).

## Battery indicator

joyshop uses JoyCon's lights as a battery indicator.
The lower a battery is, the less lights glow.
If a battery is running out, lights start to flash.
