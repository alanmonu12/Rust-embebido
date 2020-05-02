target remote :3333

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions, hard faults and panics
#break DefaultHandler
#break HardFault
#break rust_begin_unwind
break main

monitor arm semihosting enable

load

# start the process but immediately halt the processor
c

# Se activa el cliente grafico de GDB
tui enable
