/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

// The STM32F04 only has one DMA peripheral from what I've read. The data sheet contains
// lots of information about the STM32F09 which seems to have 2 DMA peripherals. Pretty
// sure the base address listed here is the same regardless of which STM32F0xx device.
//
// The DMA peripheral for the STM32F04 has 5 channels.

// Base addresses for DMA 1 and 2
pub const DMA_ADDR: *const u32 = 0x4002_0000 as *const _;
const ADDR_OFFSET: u32 = 0x0D20

// ------------------------------------
// DMAx - ISR Bit definitions
// ------------------------------------
pub const ISR_OFFSET: u32 = 0x00;
// These bits are set by hardware, and cleared in the IFCR Register by
// writing a 1 to the correct bits.

// ------------------------------------
// DMAx - IFCR Bit definitions
// ------------------------------------
pub const IFCR_OFFSET: u32 = 0x04;

// IFCR Channel 1 - DMA 1 and 2
// ------------------------------------
pub const DMAx_CGIF_1:  u32 = 0b1;
pub const DMAx_CTCIF_1: u32 = 0b1 << 1;
pub const DMAx_CHTIF_1: u32 = 0b1 << 2;
pub const DMAx_CTEIF_1: u32 = 0b1 << 3;

// IFCR Channel 2 - DMA 1 and 2
// ------------------------------------
pub const DMAx_CGIF_2:  u32 = 0b1 << 4;
pub const DMAx_CTCIF_2: u32 = 0b1 << 5;
pub const DMAx_CHTIF_2: u32 = 0b1 << 6;
pub const DMAx_CTEIF_2: u32 = 0b1 << 7;

// IFCR Channel 3 - DMA 1 and 2
// ------------------------------------
pub const DMAx_CGIF_3:  u32 = 0b1 << 8;
pub const DMAx_CTCIF_3: u32 = 0b1 << 9;
pub const DMAx_CHTIF_3: u32 = 0b1 << 10;
pub const DMAx_CTEIF_3: u32 = 0b1 << 11;

// IFCR Channel 4 - DMA 1 and 2
// ------------------------------------
pub const DMAx_CGIF_4:  u32 = 0b1 << 12;
pub const DMAx_CTCIF_4: u32 = 0b1 << 13;
pub const DMAx_CHTIF_4: u32 = 0b1 << 14;
pub const DMAx_CTEIF_4: u32 = 0b1 << 15;

// IFCR Channel 5 - DMA 1 and 2
// ------------------------------------
pub const DMAx_CGIF_5:  u32 = 0b1 << 16;
pub const DMAx_CTCIF_5: u32 = 0b1 << 17;
pub const DMAx_CHTIF_5: u32 = 0b1 << 18;
pub const DMAx_CTEIF_5: u32 = 0b1 << 19;

// IFCR Channel 6 - DMA 1 Only
// ------------------------------------
pub const DMA1_CGIF_6:  u32 = 0b1 << 20;
pub const DMA1_CTCIF_6: u32 = 0b1 << 21;
pub const DMA1_CHTIF_6: u32 = 0b1 << 22;
pub const DMA1_CTEIF_6: u32 = 0b1 << 23;

// IFCR Channel 7 - DMA 1 Only
// ------------------------------------
pub const DMA1_CGIF_1:  u32 = 0b1 << 24;
pub const DMA1_CTCIF_1: u32 = 0b1 << 25;
pub const DMA1_CHTIF_1: u32 = 0b1 << 26;
pub const DMA1_CTEIF_1: u32 = 0b1 << 27;
// Bits 28 - 31 are reserved.

// ------------------------------------
// CCRx Bit definitions (x = 1-7 for DMA and 1-5 for DMA2)
// ------------------------------------
// NOTE: CCRx offset is calculated by: 0x08 + 0d20 * (channel number - 1)
pub const CCRx_EN:      u32 = 0b1;
pub const CCRx_TCIE:    u32 = 0b1  << 1;
pub const CCRx_HTIE:    u32 = 0b1  << 2;
pub const CCRx_TEIE:    u32 = 0b1  << 3;
pub const CCRx_DIR:     u32 = 0b1  << 4;
pub const CCRx_CIRC:    u32 = 0b1  << 5;
pub const CCRx_PINC:    u32 = 0b1  << 6;
pub const CCRx_MINC:    u32 = 0b1  << 7;
pub const CCRx_PSIZE_8: u32 = 0b1  << 8;
pub const CCRx_PSIZE_9: u32 = 0b1  << 9;
pub const CCRx_MSIZE0:  u32 = 0b1  << 10;
pub const CCRx_MSIZE1:  u32 = 0b1  << 11;
pub const CCRx_PL0:     u32 = 0b1  << 12;
pub const CCRx_PL1:     u32 = 0b1  << 13;
pub const CCRx_MEM2MEM: u32 = 0b1  << 14;
// Bits 15 - 31 are reserved.

// ------------------------------------
// CNDTRx Bit definitions (x = 1-7 for DMA and 1-5 for DMA2)
// ------------------------------------
// NOTE: CNDTRx offset is calculated by: 0x0C + 0d20 * (channel number - 1)
// CNDTRx_NDT stores the number of data to be transferred (0 - 65535)

// ------------------------------------
// CPARx Bit definitions (x = 1-7 for DMA and 1-5 for DMA2)
// ------------------------------------
// NOTE: CPARx offset is calculated by: 0x10 + 0d20 * (channel number - 1)
// CPAR_PA stores the base address of the data peripheral register from/to which the
// data will be read/written. PSIZE is related to this, read data sheet.

// ------------------------------------
// CMARx Bit definitions (x = 1-7 for DMA and 1-5 for DMA2)
// ------------------------------------
// NOTE: CPARx offset is calculated by: 0x14 + 0d20 * (channel number - 1)
// CMAR_MA stores the base address of the memory area from/to which the
// data will be read/written. MSIZE is related to this, read data sheet.


