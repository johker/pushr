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
    columns: Vec<Column>,
    synapses: Vec<Synapse>,
}

impl TemporalMemory {
    pub fn new(num_columns: usize, num_cells: usize) -> Self {
        let mut columns = vec![];
        let mut cells = vec![];
        for j in 0..num_cells {
            cells.push(Cell::new());
        }
        for i in 0..num_columns {
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
            write!(
                f,
                "TMEM [{}x0]",
                self.columns.len().to_string()
            )
        }
    }
}

impl<'a> PartialEq for TemporalMemory {
    fn eq(&self, other: &Self) -> bool {
        let mut is_eq = true;
        if self.columns.len() != other.columns.len() {
            return false;
        }
        for i in 0..self.columns.len() {
           is_eq &= (self.columns[i].column_id == other.columns[i].column_id)
        }
        return is_eq;
    }
}
