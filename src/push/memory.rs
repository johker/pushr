use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

static CELL_COUNTER: AtomicUsize = AtomicUsize::new(1);
static COLUMN_COUNTER: AtomicUsize = AtomicUsize::new(1);
static SEGMENT_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Debug)]
pub struct Node {
    column: usize,
    cell: usize,
}

#[derive(Clone, Debug)]
pub struct Synapse {
    pre: Node,
    post: Node,
    segment: usize,
    permanence: f32,
}

impl Synapse {
    pub fn new(pre: Node, post: Node, segment: usize, permanence: f32) -> Self {
        Self {
            pre: pre,
            post: post,
            segment: segment,
            permanence: permanence,
        }
    }

    pub fn adpat(&mut self, new_permanence: f32) {
        self.permanence = new_permanence;
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    cell_id: usize,
    state: u32,
    segments: usize,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            cell_id: CELL_COUNTER.fetch_add(1, Ordering::Relaxed),
            state: 0,
            segments: 0,
        }
    }

    pub fn grow_segment(&mut self) {
        self.segments += 1;
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    column_id: usize,
    active: bool,
    cells: Vec<Cell>,
}

impl Column {
    pub fn new(arg_cells: Vec<Cell>) -> Self {
        Self {
            column_id: COLUMN_COUNTER.fetch_add(1, Ordering::Relaxed),
            active: false,
            cells: arg_cells,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TemporalMemory {
    pub columns: Vec<Column>,
    pub synapses: Vec<Synapse>,
}

impl TemporalMemory {
    pub fn new(ncols: usize, ncells: usize) -> Self {
        let mut columns = vec![];
        let mut cells = vec![];
        for j in 0..ncells {
            cells.push(Cell::new());
        }
        for i in 0..ncols {
            columns.push(Column::new(cells.clone()));
        }
        Self {
            columns: columns,
            synapses: vec![],
        }
    }
}

impl fmt::Display for TemporalMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.columns.len() > 0 {
            write!(
                f,
                "TMEM [{}x{}]",
                self.columns.len().to_string(),
                self.columns[0].cells.len().to_string()
            )
        } else {
            write!(f, "TMEM [{}x0]", self.columns.len().to_string())
        }
    }
}

impl PartialEq for TemporalMemory {
    fn eq(&self, other: &Self) -> bool {
        let mut is_eq = true;
        if self.columns.len() != other.columns.len() {
            return false;
        }
        for i in 0..self.columns.len() {
            is_eq &= self.columns[i].column_id == other.columns[i].column_id
        }
        return is_eq;
    }
}

pub fn load_memory_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("MEMORY.NEW"), Instruction::new(memory_new));
}

/// MEMORY.ADD: Pushes a new instance of temporal memory where the
/// number of columns and cells per column are taken from the INTEGER stack.
fn memory_new(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        let ncols = i32::max(ivals[0], 0) as usize;
        let ncells = i32::max(ivals[1], 0) as usize;
        push_state
            .memory_stack
            .push(TemporalMemory::new(ncols, ncells));
    }
}

/// MEMORY.COLUMN*STATE: Pushes the state of the columns at index i to
/// the BOOLEAN stack where i is taken from the INTEGER stack.
fn memory_column_state(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(memory) = push_state.memory_stack.get(0) {
            let corr_index = i32::max(i32::min(index, memory.columns.len() as i32 - 1), 0) as usize;
            push_state
                .bool_stack
                .push(memory.columns[corr_index].active);
        }
    }
}
