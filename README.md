# flutter_rust_bridge_template

This repository serves as a template for Flutter projects calling into native Rust
libraries via `flutter_rust_bridge`.

## Getting Started

To begin, ensure that you have a working installation of the following items:
- [Flutter SDK](https://docs.flutter.dev/get-started/install)
- [Rust language](https://rustup.rs/)
- `flutter_rust_bridge_codegen` [cargo package](https://cjycode.com/flutter_rust_bridge/integrate/deps.html#build-time-dependencies)
- Appropriate [Rust targets](https://rust-lang.github.io/rustup/cross-compilation.html) for cross-compiling to your device
- For Android targets:
    - Install [cargo-ndk](https://github.com/bbqsrc/cargo-ndk#installing)
    - Install [Android NDK 22](https://github.com/android/ndk/wiki/Unsupported-Downloads#r22b), then put its path in one of the `gradle.properties`, e.g.:

```
echo "ANDROID_NDK=.." >> ~/.gradle/gradle.properties
```

- For iOS targets:
  - Install [cargo-xcode](https://gitlab.com/kornelski/cargo-xcode#installation)
- [Web dependencies](http://cjycode.com/flutter_rust_bridge/template/setup_web.html) for the Web

Then go ahead and run `flutter run` (for web, run `dart run flutter_rust_bridge:serve` instead). When you're ready, refer to our documentation
[here](https://fzyzcjy.github.io/flutter_rust_bridge/index.html) to learn how to write and use binding code.

Once you have edited `api.rs` to incorporate your own Rust code, the bridge files `bridge_definitions.dart` and `bridge_generated.dart` are generated using the following command (note: append ` --wasm` to add web support):

### Windows
```
flutter_rust_bridge_codegen --rust-input native\src\api.rs --dart-output .\lib\bridge_generated.dart --dart-decl-output .\lib\bridge_definitions.dart
```

### Linux/MacOS/any other Unix
```
flutter_rust_bridge_codegen --rust-input native/src/api.rs --dart-output ./lib/bridge_generated.dart --dart-decl-output ./lib/bridge_definitions.dart
```

## Scaffolding in existing projects

If you would like to generate boilerplate for using `flutter_rust_bridge` in your existing projects,
check out the [`flutter_rust_bridge` brick](https://brickhub.dev/bricks/flutter_rust_bridge/)
for more details.

## Disclaimer

This template is not affiliated with flutter_rust_bridge. Please file issues and PRs related to the template here,
not flutter_rust_bridge.

## License

Copyright 2022 Viet Dinh.

This template is licensed under either of
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))

at your option.

The [SPDX](https://spdx.dev/) license identifier for this project is `MIT OR Apache-2.0`.

# CAN YOU FEEL THE PAIN?
Flutter: 3.10.x
java 18.0.1.1 2022-04-22
Java(TM) SE Runtime Environment (build 18.0.1.1+2-6)
Java HotSpot(TM) 64-Bit Server VM (build 18.0.1.1+2-6, mixed mode, sharing)


- installed rust for windows 64
- git clone flutter_bridge with_flutter example
- tried running it on the emulator
---
Launching lib\main.dart on sdk gphone64 x86 64 in debug mode...
main.dart:1
Upgrading build.gradle
Conflict detected between Android Studio Java version and Gradle version, upgrading Gradle version from 6.7 to 7.6.1.
Upgrading gradle-wrapper.properties
---
FAILURE: Build failed with an exception.

* What went wrong:
Execution failed for task ':app:processDebugMainManifest'.
> Unable to make field private final java.lang.String java.io.File.path accessible: module java.base does not "opens java.io" to unnamed module @1beef67a
---

- installed NDK (side by side) 25.2.9519653
- is installed under C:\Anwendungen\AndroidSDK\ndk\25.2.9519653
- updated C:\Users\info\.gradle - there is no gradle.properties
- updated android/gradle/gradle.properties in project
---
org.gradle.jvmargs=-Xmx1536M
android.useAndroidX=true
android.enableJetifier=true
ANDROID_NDK=C:\Anwendungen\AndroidSDK\\ndk\25.2.9519653
---
- run cargo install cargo-ndk
- run cargo ndk -o ../android/app/src/main/jniLibs build -  Failed loading manifest: Das System kann die angegebene Datei nicht finden. (os error 2)
- PS C:\Development\rust\flutter_rust_bridge\frb_example\with_flutter> cargo ndk -o .\android\app\src\main\jniLibs build 
[2023-08-16T06:37:04Z INFO  cargo_ndk::cli] Using NDK at path: C:\Anwendungen\AndroidSDK\ndk\25.2.9519653 (ANDROID_HOME)-

There is no jniLibs folder? Where does it come from?

Launching lib\main.dart on sdk gphone64 x86 64 in debug mode...
main.dart:1
Warning: Mapping new ns http://schemas.android.com/repository/android/common/02 to old ns http://schemas.android.com/repository/android/common/01
Warning: Mapping new ns http://schemas.android.com/repository/android/generic/02 to old ns http://schemas.android.com/repository/android/generic/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/addon2/02 to old ns http://schemas.android.com/sdk/android/repo/addon2/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/addon2/03 to old ns http://schemas.android.com/sdk/android/repo/addon2/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/repository2/02 to old ns http://schemas.android.com/sdk/android/repo/repository2/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/repository2/03 to old ns http://schemas.android.com/sdk/android/repo/repository2/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/sys-img2/03 to old ns http://schemas.android.com/sdk/android/repo/sys-img2/01
Warning: Mapping new ns http://schemas.android.com/sdk/android/repo/sys-img2/02 to old ns http://schemas.android.com/sdk/android/repo/sys-img2/01
Warning: unerwartetes Element (URI:"", lokal:"base-extension"). Erwartete Elemente sind <{}codename>,<{}layoutlib>,<{}api-level>

CHANGED TO TEMPLATE
- did run: rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
- success, then tried flutter run:
---
FAILURE: Build failed with an exception.

* Where:
Build file 'C:\Development\rust\flutter_rust_metronome\android\app\build.gradle' line: 87

* What went wrong:
A problem occurred evaluating project ':app'.
> Could not create task ':app:cargoBuildDebug'.
   > Could not get unknown property 'ANDROID_NDK' for task ':app:cargoBuildDebug' of type org.gradle.api.tasks.Exec.
---
- added ANDROID_NDK=C:\Anwendungen\AndroidSDK\ndk to gradle.properties
- tried running flutter run again
---
[2023-08-16T07:10:57Z INFO  cargo_ndk::cli] Using NDK at path: C:AnwendungenAndroidSDK
    dk (ANDROID_NDK_HOME)
thread 'main' panicked at 'could not resolve NDK version: Os { code: 123, kind: InvalidFilename, message: "Die Syntax für den Dateinamen, Verzeichnisnamen oder die Datenträgerbezeichnung ist falsch." }', C:\Users\info\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cargo-ndk-3.2.2\src\cli.rs:281:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
- tried changing it to: C:\Anwendungen\AndroidSDK\ndk\25.2.9519653
---
Launching lib\main.dart on sdk gphone64 x86 64 in debug mode...
main.dart:1
[2023-08-16T07:12:07Z INFO  cargo_ndk::cli] Using NDK at path: C:AnwendungenAndroidSDK
    dk25.2.9519653 (ANDROID_NDK_HOME)
thread 'main' panicked at 'could not resolve NDK version: Os { code: 123, kind: InvalidFilename, message: "Die Syntax für den Dateinamen, Verzeichnisnamen oder die Datenträgerbezeichnung ist falsch." }', C:\Users\info\.cargo\registry\src\index.crates.io-6f17d22bba15001f\cargo-ndk-3.2.2\src\cli.rs:281:53
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

FAILURE: Build failed with an exception.
---
- changed it to: ANDROID_NDK=C:/Anwendungen/AndroidSDK/ndk/25.2.9519653
- IT WORKS

- added LLVM (windows) 
- C:\Program Files\LLVM

bookmark:
https://github.com/erikas-taroza/simple_audio.git
https://github.com/fzyzcjy/flutter_rust_bridge/issues/1058
https://github.com/fzyzcjy/flutter_rust_bridge/issues/1271