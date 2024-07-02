use halo2_code::circuit::MyCircuit;
use halo2_proofs::{
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, Error, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Blake2bWrite, Challenge255},
};

use pasta_curves::EqAffine;
use rand_core::OsRng;

fn main() {
    let params: Params<EqAffine> = Params::new(3);
    let vk = keygen_vk(&params, &MyCircuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk, &MyCircuit).expect("keygen_pk should not fail");
    let mut transcript_for_proof = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);

    // Create proof with wrong number of instances
    let proof = create_proof(
        &params,
        &pk,
        &[MyCircuit, MyCircuit],
        &[], // failed
        OsRng,
        &mut transcript_for_proof,
    );
    // because of the instances.len != circuit.len so must be failed.
    assert!(matches!(proof.unwrap_err(), Error::InvalidInstances));

    // Create proof with correct number of instances
    create_proof(
        &params,
        &pk,
        &[MyCircuit, MyCircuit],
        &[&[], &[]],
        OsRng,
        &mut transcript_for_proof,
    )
    .expect("proof generation should not fail");

    let strategy = SingleVerifier::new(&params);

    let proof = transcript_for_proof.finalize();
    let mut transcipt_for_verify = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    assert!(verify_proof(
        &params,
        pk.get_vk(),
        strategy,
        &[&[]],
        &mut transcipt_for_verify
    )
    .is_ok());
}
