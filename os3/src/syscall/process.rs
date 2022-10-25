//! Process management syscalls

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{
    exit_current_and_run_next, get_start_time, get_syscall_map, suspend_current_and_run_next,
    TaskStatus,
};
use crate::timer::get_time_us;
use crate::syscall::*;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let syscall_map = get_syscall_map();
    let mut syscall_times = [0; MAX_SYSCALL_NUM];
    for i in 0..5 {
        syscall_times[match i {
            0 => SYSCALL_WRITE,
            1 => SYSCALL_EXIT,
            2 => SYSCALL_YIELD,
            3 => SYSCALL_GET_TIME,
            4 => SYSCALL_TASK_INFO,
            _ => panic!(),
        }] += syscall_map[i];
    }
    unsafe {
        *ti = TaskInfo {
            status: TaskStatus::Running,
            time: (get_time_us() - get_start_time()) / 1_000,
            syscall_times,
        }
    }
    0
}
