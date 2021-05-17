#![allow(dead_code)]

const GP_FSEL0: *mut u32 = 0x7e200000 as *mut u32;
const GP_FSEL1: *mut u32 = 0x7e200004 as *mut u32;
const GP_FSEL2: *mut u32 = 0x7e200008 as *mut u32;
const GP_FSEL3: *mut u32 = 0x7e20000c as *mut u32;
const GP_FSEL4: *mut u32 = 0x7e200010 as *mut u32;
const GP_FSEL5: *mut u32 = 0x7e200014 as *mut u32;
const GP_SET0: *mut u32 = 0x7e20001c as *mut u32;
const GP_SET1: *mut u32 = 0x7e200020 as *mut u32;
const GP_CLR0: *mut u32 = 0x7e200028 as *mut u32;
const GP_CLR1: *mut u32 = 0x7e20002c as *mut u32;
const GP_LEV0: *mut u32 = 0x7e200034 as *mut u32;
const GP_LEV1: *mut u32 = 0x7e200038 as *mut u32;
const GP_EDS0: *mut u32 = 0x7e200040 as *mut u32;
const GP_EDS1: *mut u32 = 0x7e200044 as *mut u32;
const GP_REN0: *mut u32 = 0x7e20004c as *mut u32;
const GP_REN1: *mut u32 = 0x7e200050 as *mut u32;
const GP_FEN0: *mut u32 = 0x7e200058 as *mut u32;
const GP_FEN1: *mut u32 = 0x7e20005c as *mut u32;
const GP_HEN0: *mut u32 = 0x7e200064 as *mut u32;
const GP_HEN1: *mut u32 = 0x7e200068 as *mut u32;
const GP_HEN2: *mut u32 = 0x7e20006c as *mut u32;
const GP_LEN0: *mut u32 = 0x7e200070 as *mut u32;
const GP_LEN1: *mut u32 = 0x7e200074 as *mut u32;
const GP_AREN0: *mut u32 = 0x7e20007c as *mut u32;
const GP_AREN1: *mut u32 = 0x7e200080 as *mut u32;
const GP_AFEN0: *mut u32 = 0x7e200088 as *mut u32;
const GP_AFEN1: *mut u32 = 0x7e20008c as *mut u32;
const GP_PUD: *mut u32 = 0x7e200094 as *mut u32;
const GP_PUDCLK0: *mut u32 = 0x7e200098 as *mut u32;
const GP_PUDCLK1: *mut u32 = 0x7e20009c as *mut u32;
const GP_GPTEST: *mut u32 = 0x7e2000b0 as *mut u32;


