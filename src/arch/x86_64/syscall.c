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
 * https://git.musl-libc.org/cgit/musl/tree/arch/x86_64/syscall_arch.h
 */
long __syscall0(long n) {
  unsigned long ret;
  __asm__ __volatile__("syscall" : "=a"(ret) : "a"(n) : "rcx", "r11", "memory");
  return ret;
}

long __syscall1(long n, unsigned long a1) {
  unsigned long ret;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1)
                       : "rcx", "r11", "memory");
  return ret;
}

long __syscall2(long n, unsigned long a1, unsigned long a2) {
  unsigned long ret;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1), "S"(a2)
                       : "rcx", "r11", "memory");
  return ret;
}

long __syscall3(long n, unsigned long a1, unsigned long a2, unsigned long a3) {
  unsigned long ret;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1), "S"(a2), "d"(a3)
                       : "rcx", "r11", "memory");
  return ret;
}

long __syscall4(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4) {
  unsigned long ret;
  register unsigned long r10 __asm__("r10") = a4;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10)
                       : "rcx", "r11", "memory");
  return ret;
}

long __syscall5(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4,
                unsigned long a5) {
  unsigned long ret;
  register unsigned long r10 __asm__("r10") = a4;
  register unsigned long r8 __asm__("r8") = a5;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8)
                       : "rcx", "r11", "memory");
  return ret;
}

long __syscall6(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4,
                unsigned long a5,
                unsigned long a6) {
  unsigned long ret;
  register unsigned long r10 __asm__("r10") = a4;
  register unsigned long r8 __asm__("r8") = a5;
  register unsigned long r9 __asm__("r9") = a6;
  __asm__ __volatile__("syscall"
                       : "=a"(ret)
                       : "a"(n), "D"(a1), "S"(a2), "d"(a3), "r"(r10), "r"(r8),
                         "r"(r9)
                       : "rcx", "r11", "memory");
  return ret;
}
