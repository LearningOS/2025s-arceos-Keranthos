#include <stddef.h>
#include <stdio.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <unistd.h>
#include <errno.h>
#include <stdint.h>

// TODO:
void *mmap(void *addr, size_t len, int prot, int flags, int fildes, off_t off)
{
    register uintptr_t ret asm("a0") = (uintptr_t)addr;
    register uintptr_t r_len asm("a1") = (uintptr_t)len;
    register uintptr_t r_prot asm("a2") = (uintptr_t)prot;
    register uintptr_t r_flags asm("a3") = (uintptr_t)flags;
    register uintptr_t r_fd asm("a4") = (uintptr_t)fildes;
    register uintptr_t r_off asm("a5") = (uintptr_t)off;
    register uintptr_t syscall_id asm("a7") = 222;

    printf("begin to change in libc");

    asm volatile (
        "ecall"
        : "=r"(ret)
        : "r"(ret), "r"(r_len), "r"(r_prot), "r"(r_flags), "r"(r_fd), "r"(r_off), "r"(syscall_id)
        : "memory"
    );
    printf("%d", ret);

    // 检查返回值是否是错误
    if ((intptr_t)ret < 0) {
        errno = -ret;
        return MAP_FAILED;
    }
    return (void *)ret;
}

// TODO:
int munmap(void *addr, size_t length)
{
    unimplemented();
    return 0;
}

// TODO:
void *mremap(void *old_address, size_t old_size, size_t new_size, int flags,
             ... /* void *new_address */)
{
    unimplemented();
    return NULL;
}

// TODO
int mprotect(void *addr, size_t len, int prot)
{
    unimplemented();
    return 0;
}

// TODO
int madvise(void *addr, size_t len, int advice)
{
    unimplemented();
    return 0;
}
