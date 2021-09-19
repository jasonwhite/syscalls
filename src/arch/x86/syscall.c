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
 * https://git.musl-libc.org/cgit/musl/tree/arch/i386/syscall_arch.h
 */
#if SYSCALL_NO_TLS
#define SYSCALL_INSNS "int $128"
#else
#define SYSCALL_INSNS "call *%%gs:16"
#endif

#define SYSCALL_INSNS_12 \
  "xchg %%ebx,%%edx ; " SYSCALL_INSNS " ; xchg %%ebx,%%edx"
#define SYSCALL_INSNS_34 \
  "xchg %%ebx,%%edi ; " SYSCALL_INSNS " ; xchg %%ebx,%%edi"

long __syscall0(long n) {
  unsigned long __ret;
  __asm__ __volatile__(SYSCALL_INSNS : "=a"(__ret) : "a"(n) : "memory");
  return __ret;
}

long __syscall1(long n, unsigned long a1) {
  unsigned long __ret;
  __asm__ __volatile__(SYSCALL_INSNS_12
                       : "=a"(__ret)
                       : "a"(n), "d"(a1)
                       : "memory");
  return __ret;
}

long __syscall2(long n, unsigned long a1, unsigned long a2) {
  unsigned long __ret;
  __asm__ __volatile__(SYSCALL_INSNS_12
                       : "=a"(__ret)
                       : "a"(n), "d"(a1), "c"(a2)
                       : "memory");
  return __ret;
}

long __syscall3(long n, unsigned long a1, unsigned long a2, unsigned long a3) {
  unsigned long __ret;
#if !defined(__PIC__) || !defined(BROKEN_EBX_ASM)
  __asm__ __volatile__(SYSCALL_INSNS
                       : "=a"(__ret)
                       : "a"(n), "b"(a1), "c"(a2), "d"(a3)
                       : "memory");
#else
  __asm__ __volatile__(SYSCALL_INSNS_34
                       : "=a"(__ret)
                       : "a"(n), "D"(a1), "c"(a2), "d"(a3)
                       : "memory");
#endif
  return __ret;
}

long __syscall4(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4) {
  unsigned long __ret;
#if !defined(__PIC__) || !defined(BROKEN_EBX_ASM)
  __asm__ __volatile__(SYSCALL_INSNS
                       : "=a"(__ret)
                       : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4)
                       : "memory");
#else
  __asm__ __volatile__(SYSCALL_INSNS_34
                       : "=a"(__ret)
                       : "a"(n), "D"(a1), "c"(a2), "d"(a3), "S"(a4)
                       : "memory");
#endif
  return __ret;
}

long __syscall5(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4,
                unsigned long a5) {
  unsigned long __ret;
#if !defined(__PIC__) || !defined(BROKEN_EBX_ASM)
  __asm__ __volatile__(SYSCALL_INSNS
                       : "=a"(__ret)
                       : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4), "D"(a5)
                       : "memory");
#else
  __asm__ __volatile__(
      "pushl %2 ; push %%ebx ; mov 4(%%esp),%%ebx ; " SYSCALL_INSNS
      " ; pop %%ebx ; add $4,%%esp"
      : "=a"(__ret)
      : "a"(n), "g"(a1), "c"(a2), "d"(a3), "S"(a4), "D"(a5)
      : "memory");
#endif
  return __ret;
}

long __syscall6(long n,
                unsigned long a1,
                unsigned long a2,
                unsigned long a3,
                unsigned long a4,
                unsigned long a5,
                unsigned long a6) {
  unsigned long __ret;
#if !defined(__PIC__) || !defined(BROKEN_EBX_ASM)
  __asm__ __volatile__(
      "pushl %7 ; push %%ebp ; mov 4(%%esp),%%ebp ; " SYSCALL_INSNS
      " ; pop %%ebp ; add $4,%%esp"
      : "=a"(__ret)
      : "a"(n), "b"(a1), "c"(a2), "d"(a3), "S"(a4), "D"(a5), "g"(a6)
      : "memory");
#else
  unsigned long a1a6[2] = {a1, a6};
  __asm__ __volatile__(
      "pushl %1 ; push %%ebx ; push %%ebp ; mov 8(%%esp),%%ebx ; mov "
      "4(%%ebx),%%ebp ; mov (%%ebx),%%ebx ; " SYSCALL_INSNS
      " ; pop %%ebp ; pop %%ebx ; add $4,%%esp"
      : "=a"(__ret)
      : "g"(&a1a6), "a"(n), "c"(a2), "d"(a3), "S"(a4), "D"(a5)
      : "memory");
#endif
  return __ret;
}
