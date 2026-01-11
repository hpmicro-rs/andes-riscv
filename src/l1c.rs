//! L1C, level 1 cache

use cctl_cmds::L1D_VA_LOCK;

use crate::register;
use crate::riscv;

// cctl_cmd
pub mod cctl_cmds {
    pub const L1D_VA_INVAL: u8 = 0;
    pub const L1D_VA_WB: u8 = 1;
    pub const L1D_VA_WBINVAL: u8 = 2;
    pub const L1D_VA_LOCK: u8 = 3;
    pub const L1D_VA_UNLOCK: u8 = 4;
    pub const L1D_WBINVAL_ALL: u8 = 6;
    pub const L1D_WB_ALL: u8 = 7;

    pub const L1I_VA_INVAL: u8 = 8;
    pub const L1I_VA_LOCK: u8 = 11;
    pub const L1I_VA_UNLOCK: u8 = 12;

    pub const L1D_IX_INVAL: u8 = 16;
    pub const L1D_IX_WB: u8 = 17;
    pub const L1D_IX_WBINVAL: u8 = 18;

    pub const L1D_IX_RTAG: u8 = 19;
    pub const L1D_IX_RDATA: u8 = 20;
    pub const L1D_IX_WTAG: u8 = 21;
    pub const L1D_IX_WDATA: u8 = 22;

    pub const L1D_INVAL_ALL: u8 = 23;

    pub const L1I_IX_INVAL: u8 = 24;
    pub const L1I_IX_RTAG: u8 = 27;
    pub const L1I_IX_RDATA: u8 = 28;
    pub const L1I_IX_WTAG: u8 = 29;
    pub const L1I_IX_WDATA: u8 = 30;
}

#[inline(always)]
pub fn dc_is_enabled() -> bool {
    register::mcache_ctl().read().dc_en()
}

#[inline(always)]
pub fn ic_is_enabled() -> bool {
    register::mcache_ctl().read().ic_en()
}

#[inline(always)]
pub unsafe fn dc_enable() {
    unsafe {
        if !dc_is_enabled() {
            register::mcache_ctl().modify(|w| w.set_dc_warnd(0b11));
            register::mcache_ctl().modify(|w| {
                // TODO: set dc_warnd
                w.set_dpref_en(true);
                w.set_dc_en(true);
            });
        }
    }
}

#[inline(always)]
pub unsafe fn dc_disable() {
    unsafe {
        if dc_is_enabled() {
            register::mcache_ctl().modify(|w| w.set_dc_en(false));
        }
    }
}

#[inline(always)]
pub unsafe fn ic_enable() {
    unsafe {
        if !ic_is_enabled() {
            register::mcache_ctl().modify(|w| {
                w.set_ic_en(true);
                w.set_ipref_en(true);
                w.set_cctl_suen(true);
            });
        }
    }
}

#[inline(always)]
pub unsafe fn ic_disable() {
    unsafe {
        if ic_is_enabled() {
            register::mcache_ctl().modify(|w| w.set_ic_en(false));
        }
    }
}

#[inline(always)]
pub unsafe fn dc_invalidate_all() {
    unsafe {
        register::mcctlcommand().write_value(cctl_cmds::L1D_INVAL_ALL as u32);
    }
}

#[inline(always)]
pub unsafe fn dc_writeback_all() {
    unsafe {
        register::mcctlcommand().write_value(cctl_cmds::L1D_WB_ALL as u32);
    }
}

#[inline(always)]
pub unsafe fn dc_flush_all() {
    unsafe {
        register::mcctlcommand().write_value(cctl_cmds::L1D_WBINVAL_ALL as u32);
    }
}

#[inline(always)]
fn cctl_get_address() -> u32 {
    register::mcctlbeginaddr().read()
}

// HPM_L1C_CACHELINE_SIZE
fn cacheline_size() -> u32 {
    let dsz = register::mdcm_cfg().read().dsz();
    match dsz {
        0 => 0,
        1 => 8,
        2 => 16,
        3 => 32,
        4 => 64,
        5 => 128,
        _ => 0,
    }
}

pub unsafe fn l1c_op(opcode: u8, address: u32, size: u32) {
    unsafe {
        let mstatus = riscv::register::mstatus::read();
        if mstatus.mie() {
            riscv::register::mstatus::clear_mie();
        }

        let ver = register::mmsc_cfg().read().vcctl();

        if ver != 0 {
            register::mcctlbeginaddr().write_value(address);
            let mut next_address = address;

            while next_address < address + size && next_address >= address {
                register::mcctlcommand().write_value(opcode as u32);
                next_address = cctl_get_address();
            }
        } else {
            let cl_size = cacheline_size();
            if cl_size == 0 {
                return;
            }
            let mut i = 0;
            while i < size {
                register::mcctlbeginaddr().write_value(address + i);
                register::mcctlcommand().write_value(opcode as u32);
                i += cl_size;
            }
        }

        if mstatus.mie() {
            riscv::register::mstatus::set_mie();
        }
    }
}

fn check_addr_size(address: u32, size: u32) {
    let cl_sz = cacheline_size();

    assert!(
        address % cl_sz == 0,
        "address must be aligned to cacheline size"
    );
    assert!(size % cl_sz == 0, "size must be multiple of cacheline size");
}

pub unsafe fn dc_fill_lock(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(L1D_VA_LOCK, address, size);
    }
}

pub unsafe fn dc_invalidate(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(cctl_cmds::L1D_VA_INVAL, address, size);
    }
}

pub unsafe fn dc_writeback(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(cctl_cmds::L1D_VA_WB, address, size);
    }
}

pub unsafe fn dc_flush(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(cctl_cmds::L1D_VA_WBINVAL, address, size);
    }
}

pub unsafe fn ic_invalidate(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(cctl_cmds::L1I_VA_INVAL, address, size);
    }
}

pub unsafe fn ic_fill_lock(address: u32, size: u32) {
    unsafe {
        check_addr_size(address, size);
        l1c_op(cctl_cmds::L1I_VA_LOCK, address, size);
    }
}

#[inline]
pub fn cacheline_align_down(addr: u32) -> u32 {
    addr & !(cacheline_size() - 1)
}

#[inline]
pub fn cacheline_align_up(addr: u32) -> u32 {
    cacheline_align_down(addr + cacheline_size() - 1)
}
