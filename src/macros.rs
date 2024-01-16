#[macro_export]
macro_rules! write_memory {
    ($vm:expr, $($addr:expr => $val:expr),+) => {
        $(
            $vm.memory.write($addr, $val)?;
        )+
    };
  }
