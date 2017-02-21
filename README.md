# rust_incremental_bug_maybe
Testing incremental rust build bug

~~~
C:\rust_incremental_bug_maybe>set CARGO_INCREMENTAL=

C:\rust_incremental_bug_maybe>nmake clean all

Microsoft (R) Program Maintenance Utility Version 14.00.24210.0
Copyright (C) Microsoft Corporation.  All rights reserved.

        del test.exe 2> NUL
        rustup run nightly cargo clean
        rustup run nightly cargo build --release
   Compiling rusttest v0.1.0 (file:///C:/rust_incremental_bug_maybe)
    Finished release [optimized] target(s) in 0.57 secs
        link /SUBSYSTEM:CONSOLE /ENTRY:_start /OUT:test.exe target/release/rusttest.lib
Microsoft (R) Incremental Linker Version 14.00.24215.1
Copyright (C) Microsoft Corporation.  All rights reserved.

LINK : warning LNK4001: no object files specified; libraries used
LINK : warning LNK4068: /MACHINE not specified; defaulting to X64

C:\rust_incremental_bug_maybe>set CARGO_INCREMENTAL=1

C:\rust_incremental_bug_maybe>nmake clean all

Microsoft (R) Program Maintenance Utility Version 14.00.24210.0
Copyright (C) Microsoft Corporation.  All rights reserved.

        del test.exe 2> NUL
        rustup run nightly cargo clean
        rustup run nightly cargo build --release
   Compiling rusttest v0.1.0 (file:///C:/rust_incremental_bug_maybe)
    Finished release [optimized] target(s) in 0.39 secs
        link /SUBSYSTEM:CONSOLE /ENTRY:_start /OUT:test.exe target/release/rusttest.lib
Microsoft (R) Incremental Linker Version 14.00.24215.1
Copyright (C) Microsoft Corporation.  All rights reserved.

LINK : warning LNK4001: no object files specified; libraries used
LINK : warning LNK4068: /MACHINE not specified; defaulting to X64
rusttest.lib(rusttest-a3fd6da3fa2840a2.rusttest.o) : error LNK2019: unresolved external symbol _ZN4core9panicking5panic17hb780b24c11f4ce8cE referenced in function _start
test.exe : fatal error LNK1120: 1 unresolved externals
NMAKE : fatal error U1077: '"C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\BIN\amd64\link.EXE"' : return code '0x460'
Stop.
~~~

