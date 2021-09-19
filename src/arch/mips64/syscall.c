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
 * https://git.musl-libc.org/cgit/musl/tree/arch/mips64/syscall_arch.h
 */

#if __mips_isa_rev >= 6
#define SYSCALL_CLOBBERLIST \
  "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "memory"
#else
#define SYSCALL_CLOBBERLIST                                                 \
  "$1", "$3", "$10", "$11", "$12", "$13", "$14", "$15", "$24", "$25", "hi", \
      "lo", "memory"
#endif

long __syscall0(long n) {
  register long r7 __asm__("$7");
  register long r2 __asm__("$2");
  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "=r"(r7)
                       : "ir"(n), "0"(r2)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall1(long n, unsigned long a) {
  register long r4 __asm__("$4") = a;
  register long r7 __asm__("$7");
  register long r2 __asm__("$2");
  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "=r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall2(long n, unsigned long a, unsigned long b) {
  register long r4 __asm__("$4") = a;
  register long r5 __asm__("$5") = b;
  register long r7 __asm__("$7");
  register long r2 __asm__("$2");

  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "=r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4), "r"(r5)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall3(long n, unsigned long a, unsigned long b, unsigned long c) {
  register long r4 __asm__("$4") = a;
  register long r5 __asm__("$5") = b;
  register long r6 __asm__("$6") = c;
  register long r7 __asm__("$7");
  register long r2 __asm__("$2");

  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "=r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4), "r"(r5), "r"(r6)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall4(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d) {
  register long r4 __asm__("$4") = a;
  register long r5 __asm__("$5") = b;
  register long r6 __asm__("$6") = c;
  register long r7 __asm__("$7") = d;
  register long r2 __asm__("$2");

  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "+r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4), "r"(r5), "r"(r6)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall5(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e) {
  register long r4 __asm__("$4") = a;
  register long r5 __asm__("$5") = b;
  register long r6 __asm__("$6") = c;
  register long r7 __asm__("$7") = d;
  register long r8 __asm__("$8") = e;
  register long r2 __asm__("$2");

  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "+r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4), "r"(r5), "r"(r6), "r"(r8)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}

long __syscall6(long n,
                unsigned long a,
                unsigned long b,
                unsigned long c,
                unsigned long d,
                unsigned long e,
                unsigned long f) {
  register long r4 __asm__("$4") = a;
  register long r5 __asm__("$5") = b;
  register long r6 __asm__("$6") = c;
  register long r7 __asm__("$7") = d;
  register long r8 __asm__("$8") = e;
  register long r9 __asm__("$9") = f;
  register long r2 __asm__("$2");

  __asm__ __volatile__("daddu $2,$0,%2 ; syscall"
                       : "=&r"(r2), "+r"(r7)
                       : "ir"(n), "0"(r2), "r"(r4), "r"(r5), "r"(r6), "r"(r8),
                         "r"(r9)
                       : SYSCALL_CLOBBERLIST);
  return r7 && r2 > 0 ? -r2 : r2;
}
