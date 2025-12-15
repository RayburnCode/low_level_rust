/* Memory layout for nRF52833 (micro:bit v2) */
MEMORY
{
  /* NOTE: Flash and RAM regions for nRF52833 */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
