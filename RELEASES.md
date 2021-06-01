# Linux Commands for Windows 0.1.2 (01-06-2021)

The third release!

Only the target `x86_64-pc-windows-msvc` is supported for the commands as of now.

## Added Commands

### [`man` 0.1.0](https://github.com/LinuxCommandsForWindows/LinuxCommandsOnWindows/tree/main/src/man)

`man` is the `man` command from Linux ported over to Windows.

`man` already exists as a PowerShell Help Command - however I'd like to include it because at the end of
the day we are porting Linux commands over, in which we will need to write the manual pages for it. So this
is also a good inclusion.

You should rename the executable file to `man` before adding it to the environment variables or adding it to
your PowerShell profile - if you're using PowerShell; so that you don't have to type `man-x86_64-pc-windows-msvc`
when you want to use `man`!

## Updated Commands

### [`whoami` 0.1.1](https://github.com/LinuxCommandsForWindows/LinuxCommandsOnWindows/tree/main/src/whoami)

Updated some naming in the source code.

# Linux Commands for Windows 0.1.1 (01-06-2021)

The second release after the initial release!

Only the target `x86_64-pc-windows-msvc` is supported for the commands as of now.

## Added Commands

### [`whoami` 0.1.0](https://github.com/LinuxCommandsForWindows/LinuxCommandsOnWindows/tree/main/src/whoami)

`whoami` is the `whoami` command from Linux ported over to Windows.

Even though it already exists for Windows, I still think it'd be a great addition to the collection
to feel more like a Linux-ish guy before making my final transition to Linux.

You should rename the executable file to `whoami` before adding it to the environment variables or adding it to
your PowerShell profile - if you're using PowerShell; so that you don't have to type `whoami-x86_64-pc-windows-msvc`
when you want to use `whoami`!

# Linux Commands for Windows 0.1.0 (31-05-2021)

The initial release!

Only the target `x86_64-pc-windows-msvc` is supported for the commands as of now.

## Added Commands

### [`winfetch` 0.1.0](https://github.com/LinuxCommandsForWindows/LinuxCommandsOnWindows/tree/main/src/winfetch)

`winfetch` is a `neofetch`-like command line utility, but ported over to Windows; therefore named as `winfetch`!

This release of `winfetch` is the initial release. It has all the basic functionality to function properly.

You should rename the executable file to `winfetch` before adding it to the environment variables or adding it to
your PowerShell profile - if you're using PowerShell; so that you don't have to type `winfetch-x86_64-pc-windows-msvc`
when you want to use `winfetch`!