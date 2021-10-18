/*
 * Copyright © 2005-2020 Rich Felker, et al.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/*
 * This code is from:
 * https://git.musl-libc.org/cgit/musl/tree/arch/s390x/syscall_arch.h
 */
#define __asm_syscall(ret, ...)                                   \
  do {                                                            \
    __asm__ __volatile__("svc 0\n" : ret:__VA_ARGS__ : "memory"); \
    return r2;                                                    \
  } while (0)

#ifndef SYS_mmap
#define SYS_mmap 90
#endif

long __syscall0(long n) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2");
  __asm_syscall("=r"(r2), "r"(r1));
}

long __syscall1(long n, unsigned long a) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  __asm_syscall("+r"(r2), "r"(r1));
}

long __syscall2(long n, unsigned long a, unsigned long b) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  register long r3 __asm__("r3") = b;
  __asm_syscall("+r"(r2), "r"(r1), "r"(r3));
}

long __syscall3(long n, unsigned long a, unsigned long b, unsigned long c) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  register long r3 __asm__("r3") = b;
  register long r4 __asm__("r4") = c;
  __asm_syscall("+r"(r2), "r"(r1), "r"(r3), "r"(r4));
}

long __syscall4(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  register long r3 __asm__("r3") = b;
  register long r4 __asm__("r4") = c;
  register long r5 __asm__("r5") = d;
  __asm_syscall("+r"(r2), "r"(r1), "r"(r3), "r"(r4), "r"(r5));
}

long __syscall5(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e) {
  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  register long r3 __asm__("r3") = b;
  register long r4 __asm__("r4") = c;
  register long r5 __asm__("r5") = d;
  register long r6 __asm__("r6") = e;
  __asm_syscall("+r"(r2), "r"(r1), "r"(r3), "r"(r4), "r"(r5), "r"(r6));
}

long __syscall6(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e,
                unsigned long f) {
  if (n == SYS_mmap)
    return __syscall1(n, (long)(long[]){a, b, c, d, e, f});

  register long r1 __asm__("r1") = n;
  register long r2 __asm__("r2") = a;
  register long r3 __asm__("r3") = b;
  register long r4 __asm__("r4") = c;
  register long r5 __asm__("r5") = d;
  register long r6 __asm__("r6") = e;
  register long r7 __asm__("r7") = f;
  __asm_syscall("+r"(r2), "r"(r1), "r"(r3), "r"(r4), "r"(r5), "r"(r6), "r"(r7));
}
