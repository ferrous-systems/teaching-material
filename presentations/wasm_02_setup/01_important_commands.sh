# Installation of a toolchain
$ rustup install stable
# Selection of a default toolchain
$ rustup default stable
# Override the default toolchain in your directory
$ rustup override stable
# Display documentation in browser
$ rustup doc [--std]
# List supported targets
$ rustup target list
# Add and install a target to the toolchain
$ rustup target add <target>
# List and add components
$ rustup component list|add