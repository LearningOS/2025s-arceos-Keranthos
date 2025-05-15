use super::fs::File;
use crate::imp::fd_ops::FileLike;
use alloc::sync::Arc;
use alloc::vec;
use axerrno::{LinuxError, LinuxResult};
use axhal::mem::VirtAddr;
use axhal::paging::MappingFlags;
use axmm::AddrSpace;
use core::ffi::c_int;
use axhal::mem::phys_to_virt;
use core::ptr;

pub fn sys_mmap(
    addr: VirtAddr,
    size: usize,
    flag: MappingFlags,
    f: Option<Arc<File>>,
    space: &mut AddrSpace,
) -> c_int {
    let start = if addr == VirtAddr::from_ptr_of(ptr::null::<u8>()) {
        space.find_free_area(addr, size, space.va_range).unwrap()
    } else { addr };
    ax_println!("sys_mmap: start={:#x}, size={}, flag={:?}, file={:?}", start, size, flag, f.is_some());
    syscall_body!(sys_mmap, {
        match f {
            None => space
                .map_alloc(start, size, flag, true)
                .map_err(axerrno::LinuxError::from)
                .map(|_| 0),
            // 如果是文件映射，首先读取文件内容
            Some(file) => {
                ax_println!("Mapping file with size {}", size);
                let mut buf = vec![0u8; size];
                file.as_ref().read(&mut buf);
                ax_println!("Read {} bytes from file", size);

                // 将文件内容映射到内存
                space.map_alloc(start, size, flag, true);
                let (paddr, _, _) = space
                    .page_table()
                    .query(start)
                    .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", start));

                ax_println!("Mapped segment: VA={:#x} -> PA={:#x}", start, paddr);
                ax_println!("paddr: {:#x}", paddr);

                unsafe {
                    ax_println!("Copying {} bytes to physical address {:#x}", size, paddr);
                    core::ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        phys_to_virt(paddr).as_mut_ptr(),
                        size,
                    );
                }
                
                ax_print!("Buffer content ({} bytes): ", buf.len());
/*for byte in &buf {
    ax_print!("{:02x} ", byte);
}
ax_println!("");*/

                ax_println!("sys_mmap finish");
                Ok(start.into())
            }
        }
        /*space
        .map_alloc(start, size, flag, true)
        .map_err(axerrno::LinuxError::from)
        .map(|_| 0)*/
    })
}
