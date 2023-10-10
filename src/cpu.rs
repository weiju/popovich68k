/*
 * cpu.rs - CPU implementation. Implements the registers and CPU states
 * as well as the instructions
 */

pub struct CPUState {
    a: [u32; 8],
    d: [u32; 8],
    pc: u32,
    decode_pc: u32,  // decode pc
    sr: u32,         // Status register
    ssp: u32,        // supervisor stack pointer
    usp: u32         // user stack pointer
}

type Instruction = fn(s: &mut CPUState);

impl CPUState {
    pub fn new() -> CPUState {
        CPUState {
            a: [0; 8],
            d: [0; 8],
            pc: 0,
            decode_pc: 0,
            sr: 0x2700,
            ssp: 0,
            usp: 0
        }
    }
}

fn nop(_cpu_state: &mut CPUState) {
    // do nothing
}

static INSTRUCTION_TABLE: [Instruction; 1] = [ nop ];


// This part only gets compiled in test scenarios
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state: CPUState = CPUState::new();
        assert_eq!(0, state.a[0]);
        assert_eq!(0, state.d[0]);
        assert_eq!(0, state.pc);
        assert_eq!(0, state.decode_pc);
        assert_eq!(0x2700, state.sr);
        assert_eq!(0, state.ssp);
        assert_eq!(0, state.usp);
    }

    #[test]
    fn test_nop() {
        let mut state: CPUState = CPUState::new();
        let instr = INSTRUCTION_TABLE[0];
        instr(&mut state);
        assert_eq!(0, state.a[0]);
        assert_eq!(0, state.d[0]);
        assert_eq!(0, state.pc);
        assert_eq!(0, state.decode_pc);
        assert_eq!(0x2700, state.sr);
        assert_eq!(0, state.ssp);
        assert_eq!(0, state.usp);
    }
}
