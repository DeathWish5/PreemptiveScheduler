pub use crate::arch::ContextData;

#[derive(Debug, Default)]
pub struct Context {
    context: usize,
}

impl Context {
    pub fn set_context(&mut self, addr: usize) {
        self.context = addr;
    }

    pub fn get_context(&self) -> usize {
        self.context
    }

    pub fn get_context_data(&self) -> &ContextData {
        unsafe {
            let context = self.context as *const ContextData;
            &*context
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn get_sp(&self) -> usize {
        self.context
    }

    #[cfg(target_arch = "x86_64")]
    pub fn get_pc(&self) -> usize {
        let context_data = self.get_context_data();
        context_data.rip
    }

    #[cfg(target_arch = "riscv64")]
    pub fn get_sp(&self) -> usize {
        let context_data = self.get_context_data();
        context_data.sp
    }

    #[cfg(target_arch = "riscv64")]
    pub fn get_pc(&self) -> usize {
        let context_data = self.get_context_data();
        context_data.ra
    }
}
