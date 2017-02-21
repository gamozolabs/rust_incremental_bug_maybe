all:
	rustup run nightly cargo build --release
	link /SUBSYSTEM:CONSOLE /ENTRY:_start /OUT:test.exe target/release/rusttest.lib

clean:
	del test.exe 2> NUL
	rustup run nightly cargo clean

