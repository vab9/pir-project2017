use blas::c::Layout;
use rand::distributions::normal::Normal;
use rand;
use rand::distributions::IndependentSample;

const LAYOUT: Layout = Layout::RowMajor;

/// Note: The length of the rows should be equivalent to
/// the number of neurons in the previous layer.
pub struct Matrix {
    rows: u16,
    columns: u16,
    entries: Vec<f32>,
    layout: Layout,
}

impl Matrix {
    pub fn random_new(rows: u16, columns: u16) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0);
        let mut entries = Vec::with_capacity((rows*columns) as usize);
        for _ in 0..entries.len() {
            entries.push(normal.ind_sample(&mut rng) as f32);
        }
        Matrix {
            rows: rows,
            columns: columns,
            entries: entries,
            layout: LAYOUT,
        }
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.entries
    }

    pub fn as_mut_slice(&mut self) -> &mut[f32] {
        &mut self.entries
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }
}
