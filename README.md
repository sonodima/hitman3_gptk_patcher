# HITMAN3 GPTK Patcher

Experimental Patcher to run HITMAN3 on Apple's Game Porting Toolkit

## Update - GPTK beta3 ⚠️

Good news! Apple has swiftly fixed what was preventing the game to launch, and as a result, HITMAN3 now launches seamlessly without requiring any additional patches.

### For the more curious

The missing function was DXGIAdapter::RegisterVideoMemoryBudgetChangeNotificationEvent, and on launch, the game was checking for its return value.

In the previous versions of D3DMetal, the function was implemented like this:

```cpp
int64_t DXGIAdapter::RegisterVideoMemoryBudgetChangeNotificationEvent(...) {
    return DXGI_ERROR_UNSUPPORTED;
}
```

But from the beta3, the same function is now:

```cpp
int64_t DXGIAdapter::RegisterVideoMemoryBudgetChangeNotificationEvent(
  int64_t, int64_t, int64_t, int64_t, int32_t* arg5
) {
    *arg5 = 1;
    return S_OK;
}
```

This means that whilst the functionality is not implemented _(it's probably not going to be an issue with any game)_, games that rely on that function to succeed now will no longer refuse to run.

## Build

1. Install Rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Compile `cargo build --release`

## Usage

```bash
./target/release/h3_patcher <HITMAN3_PATH>
```

_(where <HITMAN3_PATH> is the main install directory, containing the Launcher.exe binary)_
