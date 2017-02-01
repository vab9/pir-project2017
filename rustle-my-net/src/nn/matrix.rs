use blas::c::Layout;
use rand::distributions::normal::Normal;
use rand::Rng;

const layout: Layout = Layout::RowMajor;

pub struct Matrix {
    rows: u16,
    columns: u16,
    entries: [f32],
    layout: Layout,
}

impl Matrix {
    /*pub fn new(rows: u16, columns: u16) -> Self {
        Matrix {
            rows: rows,
            columns: columns,
            entires: [f32: rows*columns],
            layout: layout,
        }
    }*/

    pub fn random_new(rows: u16, columns: u16) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0,1);
        let arr = [f32; rows*columns];
        for elem in arr.iter() {
            elem = normal.ind_sample(&rng) as f32;
        }
        Matrix {
            rows: rows,
            columns: columns,
            entires: arr,
            layout: layout,
        }
    }

    pub fn as_slice(&self) -> &[f32] {
        &entries
    }

    pub fn as_mut_slice(&mut self) -> &mut[f32] {
        &mut entries
    }

    pub fn layout(&self) -> Layout {
        layout
    }
}
