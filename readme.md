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