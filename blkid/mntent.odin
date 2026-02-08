package blkid

import "core:c"

Mntent :: struct {
	fsname: cstring,
	dir:    cstring,
	type:   cstring,
	opts:   cstring,
	freq:   int,
	passno: int,
}

foreign import mntent "system:c"
foreign mntent {
	setmntent :: proc(filename: cstring, type: cstring) -> ^c.FILE ---
	getmntent :: proc(stream: ^c.FILE) -> ^Mntent ---
	endmntent :: proc(streamp: ^c.FILE) -> int ---
}

