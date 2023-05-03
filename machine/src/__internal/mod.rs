use crate::config::StarkConfig;
use p3_air::two_row_matrix::TwoRowMatrixView;
use p3_air::Air;
use p3_field::field::{Field, FieldLike};
use p3_field::symbolic::SymbolicField;
use p3_matrix::dense::RowMajorMatrix;
use p3_mersenne_31::Mersenne31;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

pub type DefaultField = Mersenne31;

pub struct BasicFoldingAirBuilder<'a, F, FL, V>
where
    F: Field,
    FL: FieldLike<F>,
    V: Into<FL> + Copy + Add<V, Output = FL> + Sub<V, Output = FL> + Mul<V, Output = FL>,
{
    main: TwoRowMatrixView<'a, V>,
    is_first_row: FL,
    is_last_row: FL,
    is_transition: FL,
    _phantom_f: PhantomData<F>,
}

pub struct BasicSymVar {
    pub row_offset: usize,
    pub column: usize,
}

pub fn prove<SC, A>(air: &A, trace: RowMajorMatrix<SC::F>)
where
    SC: StarkConfig,
    A: for<'a> Air<
        BasicFoldingAirBuilder<'a, SC::F, <SC::F as Field>::Packing, <SC::F as Field>::Packing>,
    >,
    A: for<'a> Air<
        BasicFoldingAirBuilder<'a, SC::F, SymbolicField<SC::F, BasicSymVar>, BasicSymVar>,
    >,
{
}