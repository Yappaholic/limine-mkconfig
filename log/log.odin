package log

import "core:fmt"
import "core:os"

INFO_GREEN :: "\033[1;32m*\033[0m "
INFO_RED :: "\033[1;31m*\033[0m "
INFO_YELLOW :: "\033[1;33m*\033[0m "

info :: proc(args: ..any) {
	fmt.print(INFO_GREEN)
	fmt.println(..args)
}

warning :: proc(args: ..any) {
	fmt.print(INFO_YELLOW)
	fmt.println(..args)
}

error :: proc(args: ..any, exit_code := 1) {
	fmt.eprint(INFO_RED)
	fmt.eprintln(..args)
	os.exit(exit_code)
}

