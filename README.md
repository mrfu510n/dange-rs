# dange-rs

Windows shenanigans in Rust. Inspired by [Slackor](https://github.com/Coalfire-Research/Slackor).

## Examples

### List Processes

```powershell
PS C:\dange-rs> cargo run
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x3698
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x2744
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x7c0
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x1ea4
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 8
  Priority class    = 32
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x3f8c
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0xca8
  Thread count      = 14
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
=======================================================
PROCESS NAME:  chrome.exe
-------------------------------------------------------
  Process ID        = 0x2fe4
  Thread count      = 13
  Parent process ID = 0x3390
  Priority base     = 4
  Priority class    = 64
```