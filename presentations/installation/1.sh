# Installation of a toolchain (here: the stable release channel)
$ rustup install stable

# Selection of a default toolchain
$ rustup default stable

# Display documentation in browser
$ rustup doc [--std]

# Override the default toolchain in your directory
$ rustup override set stable

# List supported targets
$ rustup target list

# Add and install a target to the toolchain (here: to cross-compile for an ARMv6-M target)
$ rustup target add thumbv6m-none-eabi
