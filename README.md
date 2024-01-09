# debounce-rust

Ported from https://github.com/toothbrush/debounce-mac

Debounces repeating keys on macOS. This is commonly caused by Apple's "butterfly" key keyboard design.

## Build

Run

```
cargo build --release
```

## Install And Run At Startup

Run

```
cp ./target/release/debounce-rust /usr/local/bin
cp com.debounceRust.app.plist ~/Library/LaunchAgents
launchctl load ~/Library/LaunchAgents/com.debounceRust.app.plist
```

At this point, you will be prompted to allow accessibility permissions for the app. You must enable this permission for `debounce-rust`.

Finally, run

```
launchctl start com.debounceRust.app.plist
```

## Development

If you make changes to this app and want to install your updated binary, you'll first want to copy your binary into place and then revoke the accessibility permissions you granted to it. You will be re-prompted to grant permissions. If you don't revoke the permissions manually, they'll remain associated with the old binary and you'll get really confused/frustrated.