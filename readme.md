Sample ffi project for iOS

# Build static libs for iOS

The following commands

```
cargo build --target aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-ios-sim
```

will build the static libraries for the iOS arm64 arch (simulation).

If you want you can also create static libs for the 
`aarch64-apple-ios` or `x86_64-apple-ios` targets.
 
Those static libraries `*a.` should be added as static libraries in the `Build Phase`
for the Xcode project. Here only one is necessary, as only the name is important (which is the same for all configurations).

The paths have to be added to the `Library Search Path` in the Xcode `Build Settings`.
At best use different paths for the different architectures in configs (arm64/arm64-sim/x86_64 and Debug/Release),
so the correct static lib `*.a` are fetched.

But as mentioned - the static library has only been added once - for the name. The correct one is fetched via
the `Library Search Paths`.

# Create Bridging Header

Also bridging headers are needed, so it can be used in Xcode. Therefore, add a `Objective-C Bridging Header`.
Create a simple header with the same C signature like the `#[no_mangle]` annotated functions.

A bridging header can be dead simple as this one:

```C
#ifndef FfiBridge_h
#define FfiBridge_h

int fetch_google();

#endif /* FfiBridge_h */
```

which has the same signature as the exported function in Rust in the `lib.rs` file.

```Rust
#[no_mangle]
pub extern "C" fetch_google() -> i32 {
   //... fetch https://google.com and return the len() of the string
}
```

# Using the bridging header in Swift

Inside the hello world iOS Swift project, you can just call the function like so:

```Swift
import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, Rust \(fetch_google())")
        }
        .padding()
    }
}
```