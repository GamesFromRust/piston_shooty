# piston_shooty

(Title WIP) is a 2D gun-hell / puzzle shooter about guns that shoot other guns that shoot other guns (or bullets).  It's our first foray into the Rust programming language, and primarily uses the Piston engine project.  You can see the progress of our first year (~100 hours) of work on [youtube](https://www.youtube.com/watch?v=JL33RTniXtU).  You can also check out our [Game Design Doc](https://docs.google.com/document/d/1o4KA7FMiAnsUhNSu8TVDPX4hDut6xDh9fK15ulkDJVI/edit?usp=sharing). (Now accepting low-brow comments & suggestions only.  Puns must be level 7 or higher to be accepted.)

![](https://media.giphy.com/media/l1J3VPHSE8ybjels4/giphy.gif)

## Setup

Install:

```
rustup component add rustfmt
rustup component add clippy 
```

Run:

```
cargo clippy && cargo run [--release]
```

## Dependencies

Sound:

- http://www.openal.org/downloads/
  - Download both the SDK and the Installer. Install both.
  - Take OpenAL32.dll from C:\Windows\SysWOW64
  - Take OpenAL32.lib from C:\Program Files (x86)\OpenAL 1.1 SDK\libs\Win64
    - ears will make you rename this to openal.lib
- http://www.mega-nerd.com/libsndfile/
  - Download the Win64 installer. Install.
  - Take libsndfile-1.dll from C:\Program Files\Mega-Nerd\libsndfile\bin
  - Take libsndfile-1.lib from C:\Program Files\Mega-Nerd\libsndfile\lib
    - ears will make you rename this to sndfile.lib
- Copy .lib and .dll files from extern into target/\*/deps.
- If piston_shooty is crashing on startup with error code 3221225595, make sure you've installed both of these via their installers.
