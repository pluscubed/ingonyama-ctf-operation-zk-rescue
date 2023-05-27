use ark_ff::{FftField, to_bytes};
use ark_poly::univariate::{DenseOrSparsePolynomial, DensePolynomial};
use ark_poly_commit::{LabeledCommitment, LabeledPolynomial, PolynomialCommitment, QuerySet};
use ark_std::{rand::RngCore};
use std::{thread, time};
use ark_poly::{EvaluationDomain, UVPolynomial};
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
// thread::sleep(time::Duration::from_secs(2));

    // choose random p
    let p: DensePolynomial<F> = DensePolynomial::rand(f.degree(), rng);

    let f_mod = f.polynomial() + &p;

    let (h, r) = f_mod.divide_by_vanishing_poly(statement.domain).unwrap();
    let x = DensePolynomial::from_coefficients_vec(vec![F::zero(), F::one()]);
    let (g, y) = &DenseOrSparsePolynomial::from(r.clone())
        .divide_with_q_and_r(&DenseOrSparsePolynomial::from(x))
        .unwrap();

    // (f+p)(x) = h(x)Z_H(x) + xg(x) + y(x)
    // f(x) + p(x)-y(x) = h(x)Z_H(x) + xg(x)
    // s(x) := p(x) - y(x)
    // f(x) + s(x) = h(x)Z_H(x) + xg(x)

    let s = &p - y;

    let g = LabeledPolynomial::new(
        "g".into(),
        g.clone(),
        Some(statement.domain.size() - 2),
        Some(1),
    );
    let h = LabeledPolynomial::new("h".into(), h, None, Some(1));
    let s = LabeledPolynomial::new("s".into(), s, None, Some(1));

    let (commitments, rands) =
        PC::commit(ck, &[s.clone(), h.clone(), g.clone()], Some(rng)).unwrap();

    let f_labeled_commitment = LabeledCommitment::new("f".into(), statement.f.clone(), None);
    let s_labeled_commitment = commitments[0].clone();
    let h_labeled_commitment = commitments[1].clone();
    let g_labeled_commitment = commitments[2].clone();

    let s_rand = rands[0].clone();
    let h_rand = rands[1].clone();
    let g_rand = rands[2].clone();

    let s_commitment = s_labeled_commitment.commitment().clone();
    let h_commitment = h_labeled_commitment.commitment().clone();
    let g_commitment = g_labeled_commitment.commitment().clone();

    let mut fs_rng = FS::initialize(&to_bytes![&PROTOCOL_NAME, statement].unwrap());
    fs_rng.absorb(&to_bytes![s_commitment, h_commitment, g_commitment].unwrap());

    let xi = F::rand(&mut fs_rng);
    let opening_challenge = F::rand(&mut fs_rng);

    let point_label = String::from("xi");
    let query_set = QuerySet::from([
        ("f".into(), (point_label.clone(), xi)),
        ("h".into(), (point_label.clone(), xi)),
        ("g".into(), (point_label.clone(), xi)),
        ("s".into(), (point_label, xi)),
    ]);

    let batch_proof = PC::batch_open(
        ck,
        &[f.clone(), s.clone(), h.clone(), g.clone()],
        &[
            f_labeled_commitment,
            s_labeled_commitment,
            h_labeled_commitment,
            g_labeled_commitment,
        ],
        &query_set,
        opening_challenge,
        &[f_rand.clone(), s_rand, h_rand, g_rand],
        Some(rng),
    )
        .unwrap();

    let proof: Proof<F, PC> = Proof {
        f_opening: f.evaluate(&xi),
        s: s_commitment,
        s_opening: s.evaluate(&xi),
        g: g_commitment,
        g_opening: g.evaluate(&xi),
        h: h_commitment,
        h_opening: h.evaluate(&xi),
        pc_proof: batch_proof,
    };

    // println!("f_opening: {}", proof.f_opening);
    // println!("s_opening: {}", proof.s_opening);
    // println!("g_opening: {}", proof.g_opening);
    // println!("h_opening: {}", proof.h_opening);
    // for coeff in f_mod.clone().coeffs.iter() {
    //     println!("s: {}", coeff);
    // }


println!("End Proof generation.. \n");
// thread::sleep(time::Duration::from_secs(2));

    Ok(proof)
}
