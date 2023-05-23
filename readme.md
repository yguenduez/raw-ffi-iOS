Sample ffi project for iOS

# Build static libs for iOS

The following commands

```
cargo build --target aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-ios-sim
```

If you want you can also create static libs for the 
`aarch64-apple-ios` or `x86_64-apple-ios` targets.

will generate static `*.a` files, which can be added as static libraries in the `Build Phase`
for the Xcode project.
The paths have to be added to the `Library Search Path` in the Xcode `Build Settings`

# Create Bridging Header

Also bridging headers are needed, so it can be used in Xcode. Therefore, add a `Objective-C Bridging Header`.
Create a simple header with the same C signature, as the `#[no_mangle]` annotated functions.

A bridging header can be dead simple as this 

```C
#ifndef FfiBridge_h
#define FfiBridge_h

int fetch_google();

#endif /* FfiBridge_h */
```

which has the same signature as the exported function in Rust

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