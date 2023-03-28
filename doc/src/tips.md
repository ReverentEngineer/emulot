# Tips for emulation

## Serial port geometry

If you use `emulot run` to connect to the virtual machine via the serial port,
it will likely cause the default geometry of the shell to be 80x24. This may
not be obvious until you run an application that uses these values like `vim`
or other text editors which rely on `curses`. To fix this on Linux, you can 
run `stty rows <rows> cols <cols>` to set a new expected geometry.
