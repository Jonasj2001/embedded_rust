# Embedded Rust

This repository was part of Barebone Embedded Rust introductory lecture in spring and fall of 2024.  

The examples is using a STM32Fxx HAL version, based on Embedded Hal 1.0

## Get started
### Windows
Install C++ development and linker environment.
`https://rust-lang.github.io/rustup/installation/windows-msvc.html#installing-only-the-required-components-optionali`

If you only want the minimal install type into your terminal:  
`winget install Microsoft.VisualStudio.2022.BuildTools --custom "--add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.26100"`

### Unix 
```
# Prerequisites for building probe-rs

# Debian/Ubuntu
sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev libssl-dev

# Fedora / RedHat / CentOS
sudo dnf install libusbx-devel libftdi-devel libudev-devel openssl-devel

# macOS
brew install libftdi
```
Install Rustup:  
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Common:
```
# Add Cortex M4F target
rustup target add thumbv7em-none-eabihf`
cargo install flip-link
cargo install cargo-binstall
cargo binstall probe-rs-tools

## Nice to have:
rustup component add llvm-tools
cargo install cargo-binutils
```

## Future work
- [ x ] Migrate to Embedded Hal 1.0
- [ ] Add more IO examples.
  - [ x ] ADC
  - [ ] DAC
  - [ ] RTC
 
