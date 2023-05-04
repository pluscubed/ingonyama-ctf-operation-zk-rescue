use ark_ff::{FftField};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{LabeledPolynomial, PolynomialCommitment};
use ark_std::{rand::RngCore};
use std::{thread, time};
use crate::{
    data_structures::{Proof, Statement},
    error::Error,
    rng::FiatShamirRng,
};
pub const PROTOCOL_NAME: &'static [u8] = b"OPERATION_ZK_RESCUE";

pub fn prove<
    F: FftField,
    PC: PolynomialCommitment<F, DensePolynomial<F>>,
    FS: FiatShamirRng,
    R: RngCore,
>(
    ck: &PC::CommitterKey,
    statement: &Statement<F, PC>,
    f: &LabeledPolynomial<F, DensePolynomial<F>>,
    f_rand: &PC::Randomness,
    rng: &mut R,
) -> Result<Proof<F, PC>, Error<PC::Error>> {
println!("Begin Proof generation.. \n");
thread::sleep(time::Duration::from_secs(2));

   /*
        ADD YOUR CODE HERE, use the document for reference on univariate sumcheck. 
        Some other good sources are in Justin Thaler's book
    */
    todo!();
println!("End Proof generation.. \n");
thread::sleep(time::Duration::from_secs(2));
}
