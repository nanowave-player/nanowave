# uname -a
Linux ingenic 4.4.94+ #12 PREEMPT Mon Dec 29 18:21:01 CST 2025 mips GNU/Linux
# cat /proc/version
Linux version 4.4.94+ (zcz@androidserver3) (gcc version 5.2.0 (Ingenic r3.2.1-gcc520 2017.12-15) ) #12 PREEMPT Mon Dec 29 18:21:01 CST 2025



# readelf -A /lib/libc.so.6                                                                                                                                                                                  🕰 391ms  | 12:07:25
Attribute Section: gnu
File Attributes
Tag_GNU_MIPS_ABI_FP: Hard float (double precision)

MIPS ABI Flags Version: 0

ISA: MIPS32r2
GPR size: 32
CPR1 size: 32
CPR2 size: 0
FP ABI: Hard float (double precision)
ISA Extension: None
ASEs:
None
FLAGS 1: 00000001
FLAGS 2: 00000000

Primary GOT:
Canonical gp value: 00188e10

Reserved entries:
Address     Access  Initial Purpose
00180e20 -32752(gp) 00000000 Lazy resolver
00180e24 -32748(gp) 80000000 Module pointer (GNU extension)


Gerät:
MIPS32r2
O32 ABI
Hard-Float
deine Buildroot-Libs:
MIPS32r2
O32 ABI
Soft-Float
dein Rust-Target:
MIPS32r2
+fpxx
Hard-Float