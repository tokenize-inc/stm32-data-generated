MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH =  128K /* BANK_1 */
    RAM   : ORIGIN = 0x24000000, LENGTH = 1024K
    OTP   : ORIGIN = 0x08fff000, LENGTH = 1024
}