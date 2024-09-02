//! L1C, level 1 cache

pub mod cctl_cmd {
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
