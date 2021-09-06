# README 🎫

![example workflow](https://github.com/mistydemeo/kensatsu/actions/workflows/quickstart.yaml/badge.svg)

Kensatsu is a tool that allows mapping IC card inputs to input events.

## But why?

When I was on my honeymoon in Japan, I noticed that pretty much every arcade machine had a transit card reader on it. It allows you to play arcade games without coins on hand, with your card being directly debited whatever the game costs to play. When I got an arcade game to play at home, I decided I wanted to replicate the setup myself by hooking up an IC card reader to insert virtual coins. If you want to do the same, Kensatsu is here to help.

## How does it work?

Kensatsu sits between a USB card reader and your game. It waits for your card reader to register that a transit card has been tapped, then emits an input event that other software on your system can read. Right now, the main intended use for Kensatsu is [OpenJVS](https://github.com/openjvs/openjvs), an open source implementation of the JVS standard used by real arcade boards. If you're using OpenJVS, you can hook up your card reader to the same Raspberry Pi you connect your controller to.

## What's supported?

- Sony [PaSoRi](https://www.sony.net/Products/felica/business/products/RC-S380.html) USB card readers.
- [FeliCa](https://en.wikipedia.org/wiki/FeliCa) transit cards used in Japan such as Suica, Pasmo or ICOCA.
- Mobile Suica cards on iOS and Android phones.
- Other FeliCa cards such as Edy are untested, but will probably work. FeliCa cards from other countries will probably also work.
- OpenJVS.

## What's not supported?

Other transit card standards are unsupported, since I don't own any card readers that work with them. I'm happy to help anyone who wants to add support for another card reader type.

## How to install

- Install [libpafe](https://github.com/rfujita/libpafe).
- If using Linux, install `libevdev-dev` and `libusb-1.0.0-dev`.
- If using macOS, install `libusb`.
- Install Rust via [rustup](https://rustup.rs).
- Clone this repo, then run `make` and `sudo make install`.

On Linux, you may have to use [ldconfig](https://linux.die.net/man/8/ldconfig) to add `/usr/local/lib` to the default library lookup path in order for Kensatsu to be able to find libpafe.

## How to use with OpenJVS

Before using Kensatsu for the first time, add the following line to your OpenJVS game config, where `CONTROLLER_2` is the controller number OpenJVS assigned to your PaSoRi:

```
CONTROLLER_BUTTON_COIN CONTROLLER_2 COIN PLAYER_1
```

After installing, attach your PaSoRi to the same Raspberry Pi you use for OpenJVS, then run `sudo kensatsu`. Once this this is done, you should be able to tap your card in order to insert virtual coins into your game.

## What features are coming in the future?

- Support for mapping to additional input types. This will allow you to use Kensatsu with games on your PC, as a coin input for MAME cabinets, &c.
- Commandline options to override the configuration file.
- Additional configuration options.

## Debugging features

Kensatsu has an optional debug mode where, instead of sending input to other programs, it prints its output to your terminal. This mode is active if you're using Kensatsu on any OS other than Linux. On Linux, you can activate it by editing your `~/.config/kensatsu/config.yml` and changing the value for the `emitter` field to `io`.

## How can I help?

Thanks for asking! There's plenty of ways you can help.

- If you try out Kensatsu, please report back how it works.
- Tell me about any bugs you find.
- If Kensatsu doesn't support something you'd like to do, please open an issue to tell me.

## FAQ

Q: My PaSoRi S380 isn't detected.<br>
A: The official libpafe repository doesn't support the S380 model yet. You'll need to use [my fork](https://github.com/mistydemeo/libpafe/tree/s380_fix) of libpafe with the `s380_fix` branch in order to use an S380.

Q: Will this actually take money off my card?<br>
A: No. This works purely on the honour system. Kensatsu won't even read how much money is on your card, and it's not capable of removing money from your card.

Q: OpenJVS sees my PaSoRi as an S320, but I'm using a different model.<br>
A: Right now, Kensatsu will always report itself as an S320. In a future update I'll fix this so that it reports the correct model.
