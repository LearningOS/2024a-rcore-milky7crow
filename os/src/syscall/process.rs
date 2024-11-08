//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{write_byte_buffer, MapPermission}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, map_memory, suspend_current_and_run_next, TaskStatus, TASK_MANAGER
    }, timer::{get_time_ms, get_time_us}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    
    write_byte_buffer(current_user_token(), _ts as *mut u8, &ts as *const _ as *const u8, size_of::<TimeVal>());
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    let mut counter = [0u32; MAX_SYSCALL_NUM];
    for id in 0..MAX_SYSCALL_NUM {
        counter[id] = TASK_MANAGER.get_syscall_count(id);
    }
    let ti = TaskInfo {
        status: TASK_MANAGER.get_status(),
        syscall_times: counter,
        time: get_time_ms() - TASK_MANAGER.get_start_time(),
    };

    write_byte_buffer(current_user_token(), _ti as *mut u8, &ti as *const _ as *const u8, size_of::<TaskInfo>());
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    // start not page aligned
    if _start % PAGE_SIZE != 0 {
        return -1;
    }
    // unused prot bits are not zero
    if _prot & !0x7 != 0 {
        return -1;
    }
    // no perm
    if _prot & 0x7 == 0 {
        return -1;
    }
    // mapped
    //let page_count = (_len + PAGE_SIZE - 1) / PAGE_SIZE;
    //let mut vpn = VirtAddr::from(_start).floor();
    //for _i in 0..page_count {
    //    if contains_mapping(vpn) {
    //        return -1;
    //    }
    //    vpn.0 += 1;
    //}

    // out of mem

    let mut permission = MapPermission::U;
    if !(_prot & (1 << 0) == 0) {
        permission |= MapPermission::R;
    }
    if !(_prot & (1 << 1) == 0) {
        permission |= MapPermission::W;
    }
    if !(_prot & (1 << 2) == 0) {
        permission |= MapPermission::X;
    }

    //let _end = (_start + _len) & !((1 << PAGE_SIZE_BITS) - 1);
    let _end = _start + _len;
    map_memory(
        _start.into(),
        _end.into(),
        //MapPermission::U | MapPermission::R | MapPermission::W | MapPermission::X
        permission
        );
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {

    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
