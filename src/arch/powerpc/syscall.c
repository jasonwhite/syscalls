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
 * https://git.musl-libc.org/cgit/musl/tree/arch/powerpc/syscall_arch.h
 */

long __syscall0(long n) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3");
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "=r"(r3)::"memory", "cr0", "r4", "r5", "r6",
                         "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long __syscall1(long n, unsigned long a) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3)::"memory", "cr0", "r4", "r5", "r6",
                         "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long __syscall2(long n, unsigned long a, unsigned long b) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  register long r4 __asm__("r4") = b;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4)::"memory", "cr0", "r5",
                         "r6", "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long __syscall3(long n, unsigned long a, unsigned long b, unsigned long c) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  register long r4 __asm__("r4") = b;
  register long r5 __asm__("r5") = c;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5)::"memory",
                         "cr0", "r6", "r7", "r8", "r9", "r10", "r11", "r12");
  return r3;
}

long __syscall4(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  register long r4 __asm__("r4") = b;
  register long r5 __asm__("r5") = c;
  register long r6 __asm__("r6") = d;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5),
                         "+r"(r6)::"memory", "cr0", "r7", "r8", "r9", "r10",
                         "r11", "r12");
  return r3;
}

long __syscall5(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  register long r4 __asm__("r4") = b;
  register long r5 __asm__("r5") = c;
  register long r6 __asm__("r6") = d;
  register long r7 __asm__("r7") = e;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5), "+r"(r6),
                         "+r"(r7)::"memory", "cr0", "r8", "r9", "r10", "r11",
                         "r12");
  return r3;
}

long __syscall6(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e,
                unsigned long f) {
  register long r0 __asm__("r0") = n;
  register long r3 __asm__("r3") = a;
  register long r4 __asm__("r4") = b;
  register long r5 __asm__("r5") = c;
  register long r6 __asm__("r6") = d;
  register long r7 __asm__("r7") = e;
  register long r8 __asm__("r8") = f;
  __asm__ __volatile__("sc ; bns+ 1f ; neg %1, %1 ; 1:"
                       : "+r"(r0), "+r"(r3), "+r"(r4), "+r"(r5), "+r"(r6),
                         "+r"(r7), "+r"(r8)::"memory", "cr0", "r9", "r10",
                         "r11", "r12");
  return r3;
}
