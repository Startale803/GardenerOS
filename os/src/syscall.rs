use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize;3]) -> isize {
    let ret: isize;
    unsafe {
        asm!("ecall",
            inlateout("a0") args[0] => ret,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") id,
            options(nostack)
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize,0,0])
}

pub fn sys_write(fd: usize, s: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd,s.as_ptr() as usize,s.len()])
}
