/// Simple macro to write instructions into
/// the machines memory. Usually used in
/// tests or very very simple programs
#[macro_export]
macro_rules! write_memory {
    ($vm:expr, $($addr:expr => $val:expr),+) => {
        $(
            $vm.memory.write($addr, $val)?;
        )+
    };
  }
