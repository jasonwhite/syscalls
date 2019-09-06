extern "C" {
    pub fn untraced_syscall(
        no: i32,
        a0: i64,
        a1: i64,
        a2: i64,
        a3: i64,
        a4: i64,
        a5: i64,
    ) -> i64;
    pub fn traced_syscall(
        no: i32,
        a0: i64,
        a1: i64,
        a2: i64,
        a3: i64,
        a4: i64,
        a5: i64,
    ) -> i64;
}
