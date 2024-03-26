MEMORY
{
  FLASH   (rx):  ORIGIN = 0x0,        LENGTH = 0x80000
  RAM     (rwx): ORIGIN = 0x10000000, LENGTH = 0x8000
  AHBRAM0 (rwx): ORIGIN = 0x2007c000, LENGTH = 0x4000
  AHBRAM1 (rwx): ORIGIN = 0x20080000, LENGTH = 0x4000
}


SECTIONS
{
  .ram0 (NOLOAD) :
  {
    *(.ram0 .ram0.*);
  } > AHBRAM0

  .ram1 (NOLOAD) :
  {
    *(.ram1 .ram1.*);
  } > AHBRAM1
}
