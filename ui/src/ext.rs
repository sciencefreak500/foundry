use forge::debug::{DebugStep, Instruction};
use revm::opcode;

/// Helpers for working with [Instruction]s
pub trait InstructionExt {
    /// Get the stack items affected by the instruction.
    fn affected_stack_items(&self) -> Vec<(usize, &'static str)>;
}

impl InstructionExt for Instruction {
    fn affected_stack_items(&self) -> Vec<(usize, &'static str)> {
        match self {
            Instruction::OpCode(op) => stack_indices_affected(*op),
            _ => Vec::new(),
        }
    }
}

/// Helpers for working with [DebugStep]s
pub trait DebugStepExt {
    /// Get the word being written to or read by the instruction (if any)
    fn affected_memory_word(&self) -> Option<(u8, usize)>;
}

impl DebugStepExt for DebugStep {
    fn affected_memory_word(&self) -> Option<(u8, usize)> {
        if self.stack.is_empty() {
            return None
        }

        match self.instruction {
            Instruction::OpCode(op) if op == opcode::MSTORE || op == opcode::MLOAD => {
                Some((op, self.stack[self.stack.len() - 1].as_usize() / 32))
            }
            _ => None,
        }
    }
}

/// Maps an opcode and returns a vector of named affected indices
fn stack_indices_affected(op: u8) -> Vec<(usize, &'static str)> {
    match op {
        0x01 => vec![(0, "a"), (1, "b")],
        0x02 => vec![(0, "a"), (1, "b")],
        0x03 => vec![(0, "a"), (1, "b")],
        0x04 => vec![(0, "a"), (1, "b")],
        0x05 => vec![(0, "a"), (1, "b")],
        0x06 => vec![(0, "a"), (1, "b")],
        0x07 => vec![(0, "a"), (1, "b")],
        0x08 => vec![(0, "a"), (1, "b"), (2, "N")],
        0x09 => vec![(0, "a"), (1, "b"), (2, "N")],
        0x0a => vec![(0, "a"), (1, "exponent")],
        0x0b => vec![(0, "b"), (1, "x")],
        0x10 => vec![(0, "a"), (1, "b")],
        0x11 => vec![(0, "a"), (1, "b")],
        0x12 => vec![(0, "a"), (1, "b")],
        0x13 => vec![(0, "a"), (1, "b")],
        0x14 => vec![(0, "a"), (1, "b")],
        0x15 => vec![(0, "a")],
        0x16 => vec![(0, "a"), (1, "b")],
        0x17 => vec![(0, "a"), (1, "b")],
        0x18 => vec![(0, "a"), (1, "b")],
        0x19 => vec![(0, "a")],
        0x1a => vec![(0, "i"), (1, "x")],
        0x1b => vec![(0, "shift"), (1, "value")],
        0x1c => vec![(0, "shift"), (1, "value")],
        0x1d => vec![(0, "shift"), (1, "value")],
        0x20 => vec![(0, "offset"), (1, "size")],
        0x31 => vec![(0, "address")],
        0x35 => vec![(0, "offset")],
        0x37 => vec![(0, "destOffset"), (1, "offset"), (2, "size")],
        0x39 => vec![(0, "destOffset"), (1, "offset"), (2, "size")],
        0x3b => vec![(0, "address")],
        0x3c => vec![(0, "address"), (1, "destOffset"), (2, "offset"), (3, "size")],
        0x3e => vec![(0, "destOffset"), (1, "offset"), (2, "size")],
        0x3f => vec![(0, "address")],
        0x40 => vec![(0, "blockNumber")],
        0x50 => vec![(0, "y")],
        0x51 => vec![(0, "offset")],
        0x52 => vec![(0, "offset"), (1, "value")],
        0x53 => vec![(0, "offset"), (1, "value")],
        0x54 => vec![(0, "key")],
        0x55 => vec![(0, "key"), (1, "value")],
        0x56 => vec![(0, "jump_to")],
        0x57 => vec![(0, "jump_to"), (1, "if")],
        0x80 => vec![(0, "dup_value")],
        0x81 => vec![(1, "dup_value")],
        0x82 => vec![(2, "dup_value")],
        0x83 => vec![(3, "dup_value")],
        0x84 => vec![(4, "dup_value")],
        0x85 => vec![(5, "dup_value")],
        0x86 => vec![(6, "dup_value")],
        0x87 => vec![(7, "dup_value")],
        0x88 => vec![(8, "dup_value")],
        0x89 => vec![(9, "dup_value")],
        0x8a => vec![(10, "dup_value")],
        0x8b => vec![(11, "dup_value")],
        0x8c => vec![(12, "dup_value")],
        0x8d => vec![(13, "dup_value")],
        0x8e => vec![(14, "dup_value")],
        0x8f => vec![(15, "dup_value")],
        0x90 => vec![(0, "a"), (1, "swap_value")],
        0x91 => vec![(0, "a"), (2, "swap_value")],
        0x92 => vec![(0, "a"), (3, "swap_value")],
        0x93 => vec![(0, "a"), (4, "swap_value")],
        0x94 => vec![(0, "a"), (5, "swap_value")],
        0x95 => vec![(0, "a"), (6, "swap_value")],
        0x96 => vec![(0, "a"), (7, "swap_value")],
        0x97 => vec![(0, "a"), (8, "swap_value")],
        0x98 => vec![(0, "a"), (9, "swap_value")],
        0x99 => vec![(0, "a"), (10, "swap_value")],
        0x9a => vec![(0, "a"), (11, "swap_value")],
        0x9b => vec![(0, "a"), (12, "swap_value")],
        0x9c => vec![(0, "a"), (13, "swap_value")],
        0x9d => vec![(0, "a"), (14, "swap_value")],
        0x9e => vec![(0, "a"), (15, "swap_value")],
        0x9f => vec![(0, "a"), (16, "swap_value")],
        0xa0 => vec![(0, "offset"), (1, "size")],
        0xa1 => vec![(0, "offset"), (1, "size"), (2, "topic")],
        0xa2 => vec![(0, "offset"), (1, "size"), (2, "topic1"), (3, "topic2")],
        0xa3 => vec![(0, "offset"), (1, "size"), (2, "topic1"), (3, "topic2"), (4, "topic3")],
        0xa4 => vec![
            (0, "offset"),
            (1, "size"),
            (2, "topic1"),
            (3, "topic2"),
            (4, "topic3"),
            (5, "topic4"),
        ],
        0xf0 => vec![(0, "value"), (1, "offset"), (2, "size")],
        0xf1 => vec![
            (0, "gas"),
            (1, "address"),
            (2, "value"),
            (3, "argsOffset"),
            (4, "argsSize"),
            (5, "retOffset"),
            (6, "retSize"),
        ],
        0xf2 => vec![
            (0, "gas"),
            (1, "address"),
            (2, "value"),
            (3, "argsOffset"),
            (4, "argsSize"),
            (5, "retOffset"),
            (6, "retSize"),
        ],
        0xf3 => vec![(0, "offset"), (1, "size")],
        0xf4 => vec![
            (0, "gas"),
            (1, "address"),
            (2, "argsOffset"),
            (3, "argsSize"),
            (4, "retOffset"),
            (5, "retSize"),
        ],
        0xf5 => vec![(0, "value"), (1, "offset"), (2, "size"), (3, "salt")],
        0xfa => vec![
            (0, "gas"),
            (1, "address"),
            (2, "argsOffset"),
            (3, "argsSize"),
            (4, "retOffset"),
            (5, "retSize"),
        ],
        0xfd => vec![(0, "offset"), (1, "size")],
        0xff => vec![(0, "address")],
        _ => vec![],
    }
}