#![feature(libc)]
extern crate libc;

//use libc;
use libc::O_RDWR;

fn main() {

    // from man pty
    // what are flags to posix_openpt ? O_RDWR is normal.
    let fd = unsafe { libc::posix_openpt(O_RDWR) };

    if fd < 0 {
        panic!("opening terminal failed")
    }

    println!("Hello, world!");
}

/*
 * process that expects to be connected to a  termi-
 * nal, can open the slave end of a pseudoterminal and then be driven by a program that has
 * opened
 * the master end.  Anything that is written on the master end is provided to the
 * process  on  the slave end as though it was input typed on a terminal.  For example, writing
 * the interrupt char-
 * acter (usually control-C) to the master device would cause an
 * interrupt signal (SIGINT)  to  be
 * generated  for  the  foreground process group that is
 * connected to the slave.
 */
