target extended-remote :3333
break main
monitor arm semihosting enable

load
continue