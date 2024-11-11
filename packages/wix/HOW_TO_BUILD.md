If you want to build an MSI Windows installer run 
`wix build packages/wix/windows-installer.wxs -o target/wix/what-rs-amd64-windows-installer.msi -d EXE_FILE_PATH=./target/release/what-rs.exe`
at the project root.

A release version of what-rs needs to be built before you can do this.
To built what-rs run `cargo build --release`.
The build needs to be located in `target/release/what-rs.exe` afterward 
and if moved will result in the msi installer failing to build.

Note: Commands are to be run from project root.