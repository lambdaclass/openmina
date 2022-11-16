use ark_ff::{BigInteger256, Field, FromBytes, PrimeField};
use kimchi::proof::ProofEvaluations;
use mina_hasher::Fp;

use crate::{alpha_pows, PlonkMinimal};

pub enum CurrOrNext {
    Curr,
    Next,
}

pub enum Column {
    Witness(usize),
    Poseidon,
}

// Helpers methods

fn square<F>(field: F) -> F
where
    F: Field,
{
    field * field
}

fn cell<T>(v: T) -> T {
    v
}

fn double<F>(fp: F) -> F
where
    F: Field,
{
    fp.double()
}

fn field(mut s: &str) -> Fp {
    if s.starts_with("0x") {
        s = &s[2..];
    }

    let mut bytes: Vec<u8> = hex::decode(s).unwrap();
    bytes.reverse();

    let bigint = BigInteger256::read(&bytes[..]).unwrap();
    Fp::from_repr(bigint).unwrap()
}

fn get_var<F>(evals: &ProofEvaluations<[F; 2]>) -> impl Fn(Column, CurrOrNext) -> F + '_
where
    F: Field,
{
    use Column::*;
    use CurrOrNext::*;

    // Use a closure to capture `evals`
    |col: Column, row: CurrOrNext| match (col, row) {
        (Witness(i), Curr) => evals.w[i][0],
        (Witness(i), Next) => evals.w[i][1],
        (Poseidon, Curr) => evals.poseidon_selector[0],
        (Poseidon, Next) => evals.poseidon_selector[1],
    }
}

fn get_alpha_pow(minimal: &PlonkMinimal) -> impl Fn(usize) -> Fp {
    let alpha_pows = alpha_pows(minimal.alpha);
    move |i: usize| alpha_pows[i]
}

// Actual methods

#[allow(unused_parens)]
pub fn complete_add(evals: &ProofEvaluations<[Fp; 2]>, minimal: &PlonkMinimal) -> Fp {
    use Column::*;
    use CurrOrNext::*;

    let var = get_var(evals);
    let alpha_pow = get_alpha_pow(minimal);

    // Auto-generated code with the test `generate_plonk`
    let x_0 = { (cell(var(Witness(2), Curr)) - cell(var(Witness(0), Curr))) };
    let x_1 = { (cell(var(Witness(3), Curr)) - cell(var(Witness(1), Curr))) };
    let x_2 = { (cell(var(Witness(0), Curr)) * cell(var(Witness(0), Curr))) };
    ((((((((cell(var(Witness(10), Curr)) * x_0)
        - (field("0x0000000000000000000000000000000000000000000000000000000000000001")
            - cell(var(Witness(7), Curr))))
        + (alpha_pow(1) * (cell(var(Witness(7), Curr)) * x_0)))
        + (alpha_pow(2)
            * ((cell(var(Witness(7), Curr))
                * (((double(cell(var(Witness(8), Curr))) * cell(var(Witness(1), Curr)))
                    - double(x_2))
                    - x_2))
                + ((field(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                ) - cell(var(Witness(7), Curr)))
                    * ((x_0 * cell(var(Witness(8), Curr))) - x_1)))))
        + (alpha_pow(3)
            * (((cell(var(Witness(0), Curr)) + cell(var(Witness(2), Curr)))
                + cell(var(Witness(4), Curr)))
                - (cell(var(Witness(8), Curr)) * cell(var(Witness(8), Curr))))))
        + (alpha_pow(4)
            * (((cell(var(Witness(8), Curr))
                * (cell(var(Witness(0), Curr)) - cell(var(Witness(4), Curr))))
                - cell(var(Witness(1), Curr)))
                - cell(var(Witness(5), Curr)))))
        + (alpha_pow(5) * (x_1 * (cell(var(Witness(7), Curr)) - cell(var(Witness(6), Curr))))))
        + (alpha_pow(6) * ((x_1 * cell(var(Witness(9), Curr))) - cell(var(Witness(6), Curr)))))
}

#[allow(unused_parens)]
pub fn var_base_mul(evals: &ProofEvaluations<[Fp; 2]>, minimal: &PlonkMinimal) -> Fp {
    use Column::*;
    use CurrOrNext::*;

    let var = get_var(evals);
    let alpha_pow = get_alpha_pow(minimal);

    // Auto-generated code with the test `generate_plonk`
    let x_0 = { (cell(var(Witness(7), Next)) * cell(var(Witness(7), Next))) };
    let x_1 = {
        let x_0 = { (cell(var(Witness(7), Next)) * cell(var(Witness(7), Next))) };
        (cell(var(Witness(2), Curr))
            - ((x_0 - cell(var(Witness(2), Curr))) - cell(var(Witness(0), Curr))))
    };
    let x_2 = {
        let x_1 = {
            let x_0 = { (cell(var(Witness(7), Next)) * cell(var(Witness(7), Next))) };
            (cell(var(Witness(2), Curr))
                - ((x_0 - cell(var(Witness(2), Curr))) - cell(var(Witness(0), Curr))))
        };
        (double(cell(var(Witness(3), Curr))) - (x_1 * cell(var(Witness(7), Next))))
    };
    let x_3 = { (cell(var(Witness(8), Next)) * cell(var(Witness(8), Next))) };
    let x_4 = {
        let x_3 = { (cell(var(Witness(8), Next)) * cell(var(Witness(8), Next))) };
        (cell(var(Witness(7), Curr))
            - ((x_3 - cell(var(Witness(7), Curr))) - cell(var(Witness(0), Curr))))
    };
    let x_5 = {
        let x_4 = {
            let x_3 = { (cell(var(Witness(8), Next)) * cell(var(Witness(8), Next))) };
            (cell(var(Witness(7), Curr))
                - ((x_3 - cell(var(Witness(7), Curr))) - cell(var(Witness(0), Curr))))
        };
        (double(cell(var(Witness(8), Curr))) - (x_4 * cell(var(Witness(8), Next))))
    };
    let x_6 = { (cell(var(Witness(9), Next)) * cell(var(Witness(9), Next))) };
    let x_7 = {
        let x_6 = { (cell(var(Witness(9), Next)) * cell(var(Witness(9), Next))) };
        (cell(var(Witness(9), Curr))
            - ((x_6 - cell(var(Witness(9), Curr))) - cell(var(Witness(0), Curr))))
    };
    let x_8 = {
        let x_7 = {
            let x_6 = { (cell(var(Witness(9), Next)) * cell(var(Witness(9), Next))) };
            (cell(var(Witness(9), Curr))
                - ((x_6 - cell(var(Witness(9), Curr))) - cell(var(Witness(0), Curr))))
        };
        (double(cell(var(Witness(10), Curr))) - (x_7 * cell(var(Witness(9), Next))))
    };
    let x_9 = { (cell(var(Witness(10), Next)) * cell(var(Witness(10), Next))) };
    let x_10 = {
        let x_9 = { (cell(var(Witness(10), Next)) * cell(var(Witness(10), Next))) };
        (cell(var(Witness(11), Curr))
            - ((x_9 - cell(var(Witness(11), Curr))) - cell(var(Witness(0), Curr))))
    };
    let x_11 = {
        let x_10 = {
            let x_9 = { (cell(var(Witness(10), Next)) * cell(var(Witness(10), Next))) };
            (cell(var(Witness(11), Curr))
                - ((x_9 - cell(var(Witness(11), Curr))) - cell(var(Witness(0), Curr))))
        };
        (double(cell(var(Witness(12), Curr))) - (x_10 * cell(var(Witness(10), Next))))
    };
    let x_12 = { (cell(var(Witness(11), Next)) * cell(var(Witness(11), Next))) };
    let x_13 = {
        let x_12 = { (cell(var(Witness(11), Next)) * cell(var(Witness(11), Next))) };
        (cell(var(Witness(13), Curr))
            - ((x_12 - cell(var(Witness(13), Curr))) - cell(var(Witness(0), Curr))))
    };
    let x_14 = {
        let x_13 = {
            let x_12 = { (cell(var(Witness(11), Next)) * cell(var(Witness(11), Next))) };
            (cell(var(Witness(13), Curr))
                - ((x_12 - cell(var(Witness(13), Curr))) - cell(var(Witness(0), Curr))))
        };
        (double(cell(var(Witness(14), Curr))) - (x_13 * cell(var(Witness(11), Next))))
    };
    (((((((((((((((((((((cell(var(Witness(5), Curr))
        - (cell(var(Witness(6), Next))
            + double(
                (cell(var(Witness(5), Next))
                    + double(
                        (cell(var(Witness(4), Next))
                            + double(
                                (cell(var(Witness(3), Next))
                                    + double(
                                        (cell(var(Witness(2), Next))
                                            + double(cell(var(Witness(4), Curr)))),
                                    )),
                            )),
                    )),
            )))
        + (alpha_pow(1)
            * (square(cell(var(Witness(2), Next)))
                - cell(var(Witness(2), Next)))))
        + (alpha_pow(2)
            * (((cell(var(Witness(2), Curr)) - cell(var(Witness(0), Curr)))
                * cell(var(Witness(7), Next)))
                - (cell(var(Witness(3), Curr))
                    - ((double(cell(var(Witness(2), Next)))
                        - field(
                            "0x0000000000000000000000000000000000000000000000000000000000000001",
                        ))
                        * cell(var(Witness(1), Curr)))))))
        + (alpha_pow(3)
            * ((x_2 * x_2)
                - ((x_1 * x_1)
                    * ((cell(var(Witness(7), Curr))
                        - cell(var(Witness(0), Curr)))
                        + x_0)))))
        + (alpha_pow(4)
            * (((cell(var(Witness(8), Curr)) + cell(var(Witness(3), Curr)))
                * x_1)
                - ((cell(var(Witness(2), Curr)) - cell(var(Witness(7), Curr)))
                    * x_2))))
        + (alpha_pow(5)
            * (square(cell(var(Witness(3), Next))) - cell(var(Witness(3), Next)))))
        + (alpha_pow(6)
            * (((cell(var(Witness(7), Curr)) - cell(var(Witness(0), Curr)))
                * cell(var(Witness(8), Next)))
                - (cell(var(Witness(8), Curr))
                    - ((double(cell(var(Witness(3), Next)))
                        - field(
                            "0x0000000000000000000000000000000000000000000000000000000000000001",
                        ))
                        * cell(var(Witness(1), Curr)))))))
        + (alpha_pow(7)
            * ((x_5 * x_5)
                - ((x_4 * x_4)
                    * ((cell(var(Witness(9), Curr)) - cell(var(Witness(0), Curr)))
                        + x_3)))))
        + (alpha_pow(8)
            * (((cell(var(Witness(10), Curr)) + cell(var(Witness(8), Curr))) * x_4)
                - ((cell(var(Witness(7), Curr)) - cell(var(Witness(9), Curr))) * x_5))))
        + (alpha_pow(9)
            * (square(cell(var(Witness(4), Next))) - cell(var(Witness(4), Next)))))
        + (alpha_pow(10)
            * (((cell(var(Witness(9), Curr)) - cell(var(Witness(0), Curr)))
                * cell(var(Witness(9), Next)))
                - (cell(var(Witness(10), Curr))
                    - ((double(cell(var(Witness(4), Next)))
                        - field(
                            "0x0000000000000000000000000000000000000000000000000000000000000001",
                        ))
                        * cell(var(Witness(1), Curr)))))))
        + (alpha_pow(11)
            * ((x_8 * x_8)
                - ((x_7 * x_7)
                    * ((cell(var(Witness(11), Curr)) - cell(var(Witness(0), Curr)))
                        + x_6)))))
        + (alpha_pow(12)
            * (((cell(var(Witness(12), Curr)) + cell(var(Witness(10), Curr))) * x_7)
                - ((cell(var(Witness(9), Curr)) - cell(var(Witness(11), Curr))) * x_8))))
        + (alpha_pow(13)
            * (square(cell(var(Witness(5), Next))) - cell(var(Witness(5), Next)))))
        + (alpha_pow(14)
            * (((cell(var(Witness(11), Curr)) - cell(var(Witness(0), Curr)))
                * cell(var(Witness(10), Next)))
                - (cell(var(Witness(12), Curr))
                    - ((double(cell(var(Witness(5), Next)))
                        - field(
                            "0x0000000000000000000000000000000000000000000000000000000000000001",
                        ))
                        * cell(var(Witness(1), Curr)))))))
        + (alpha_pow(15)
            * ((x_11 * x_11)
                - ((x_10 * x_10)
                    * ((cell(var(Witness(13), Curr)) - cell(var(Witness(0), Curr))) + x_9)))))
        + (alpha_pow(16)
            * (((cell(var(Witness(14), Curr)) + cell(var(Witness(12), Curr))) * x_10)
                - ((cell(var(Witness(11), Curr)) - cell(var(Witness(13), Curr))) * x_11))))
        + (alpha_pow(17) * (square(cell(var(Witness(6), Next))) - cell(var(Witness(6), Next)))))
        + (alpha_pow(18)
            * (((cell(var(Witness(13), Curr)) - cell(var(Witness(0), Curr)))
                * cell(var(Witness(11), Next)))
                - (cell(var(Witness(14), Curr))
                    - ((double(cell(var(Witness(6), Next)))
                        - field(
                            "0x0000000000000000000000000000000000000000000000000000000000000001",
                        ))
                        * cell(var(Witness(1), Curr)))))))
        + (alpha_pow(19)
            * ((x_14 * x_14)
                - ((x_13 * x_13)
                    * ((cell(var(Witness(0), Next)) - cell(var(Witness(0), Curr))) + x_12)))))
        + (alpha_pow(20)
            * (((cell(var(Witness(1), Next)) + cell(var(Witness(14), Curr))) * x_13)
                - ((cell(var(Witness(13), Curr)) - cell(var(Witness(0), Next))) * x_14))))
}

#[allow(unused_parens)]
pub fn endo_mul(evals: &ProofEvaluations<[Fp; 2]>, minimal: &PlonkMinimal) -> Fp {
    use Column::*;
    use CurrOrNext::*;

    let var = get_var(evals);
    let alpha_pow = get_alpha_pow(minimal);
    let endo_coefficient: Fp = oracle::sponge::endo_coefficient();

    // Auto-generated code with the test `generate_plonk`
    let x_0 = {
        ((field("0x0000000000000000000000000000000000000000000000000000000000000001")
            + (cell(var(Witness(11), Curr))
                * (endo_coefficient
                    - field(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ))))
            * cell(var(Witness(0), Curr)))
    };
    let x_1 = {
        ((field("0x0000000000000000000000000000000000000000000000000000000000000001")
            + (cell(var(Witness(13), Curr))
                * (endo_coefficient
                    - field(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ))))
            * cell(var(Witness(0), Curr)))
    };
    let x_2 = { square(cell(var(Witness(9), Curr))) };
    let x_3 = { square(cell(var(Witness(10), Curr))) };
    let x_4 = { (cell(var(Witness(4), Curr)) - cell(var(Witness(7), Curr))) };
    let x_5 = { (cell(var(Witness(7), Curr)) - cell(var(Witness(4), Next))) };
    let x_6 = { (cell(var(Witness(5), Next)) + cell(var(Witness(8), Curr))) };
    let x_7 = { (cell(var(Witness(8), Curr)) + cell(var(Witness(5), Curr))) };
    (((((((((((square(cell(var(Witness(11), Curr))) - cell(var(Witness(11), Curr)))
        + (alpha_pow(1)
            * (square(cell(var(Witness(12), Curr))) - cell(var(Witness(12), Curr)))))
        + (alpha_pow(2)
            * (square(cell(var(Witness(13), Curr))) - cell(var(Witness(13), Curr)))))
        + (alpha_pow(3)
            * (square(cell(var(Witness(14), Curr))) - cell(var(Witness(14), Curr)))))
        + (alpha_pow(4)
            * (((x_0 - cell(var(Witness(4), Curr))) * cell(var(Witness(9), Curr)))
                - (((double(cell(var(Witness(12), Curr)))
                    - field(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ))
                    * cell(var(Witness(1), Curr)))
                    - cell(var(Witness(5), Curr))))))
        + (alpha_pow(5)
            * ((((double(cell(var(Witness(4), Curr))) - x_2) + x_0)
                * ((x_4 * cell(var(Witness(9), Curr))) + x_7))
                - (double(cell(var(Witness(5), Curr))) * x_4))))
        + (alpha_pow(6)
            * (square(x_7) - (square(x_4) * ((x_2 - x_0) + cell(var(Witness(7), Curr)))))))
        + (alpha_pow(7)
            * (((x_1 - cell(var(Witness(7), Curr))) * cell(var(Witness(10), Curr)))
                - (((double(cell(var(Witness(14), Curr)))
                    - field(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    ))
                    * cell(var(Witness(1), Curr)))
                    - cell(var(Witness(8), Curr))))))
        + (alpha_pow(8)
            * ((((double(cell(var(Witness(7), Curr))) - x_3) + x_1)
                * ((x_5 * cell(var(Witness(10), Curr))) + x_6))
                - (double(cell(var(Witness(8), Curr))) * x_5))))
        + (alpha_pow(9)
            * (square(x_6) - (square(x_5) * ((x_3 - x_1) + cell(var(Witness(4), Next)))))))
        + (alpha_pow(10)
            * ((double(
                (double(
                    (double((double(cell(var(Witness(6), Curr))) + cell(var(Witness(11), Curr))))
                        + cell(var(Witness(12), Curr))),
                ) + cell(var(Witness(13), Curr))),
            ) + cell(var(Witness(14), Curr)))
                - cell(var(Witness(6), Next)))))
}

#[allow(unused_parens)]
#[rustfmt::skip] // See below
pub fn endo_mul_scalar(evals: &ProofEvaluations<[Fp; 2]>, minimal: &PlonkMinimal) -> Fp {
    use Column::*;
    use CurrOrNext::*;

    let var = get_var(evals);
    let alpha_pow = get_alpha_pow(minimal);

    // Auto-generated code with the test `generate_plonk`
    let x_0 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(6), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(6), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(6), Curr)))
    };
    let x_1 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(7), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(7), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(7), Curr)))
    };
    let x_2 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(8), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(8), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(8), Curr)))
    };
    let x_3 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(9), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(9), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(9), Curr)))
    };
    let x_4 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(10), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(10), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(10), Curr)))
    };
    let x_5 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(11), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(11), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(11), Curr)))
    };
    let x_6 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(12), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(12), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(12), Curr)))
    };
    let x_7 = {
        (((((field("0x1555555555555555555555555555555560C232FEADC45309330F104F00000001")
            * cell(var(Witness(13), Curr)))
            + field("0x2000000000000000000000000000000011234C7E04A67C8DCC9698767FFFFFFE"))
            * cell(var(Witness(13), Curr)))
            + field("0x0AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB061197F56E229849987882780000002"))
            * cell(var(Witness(13), Curr)))
    };

    // Note: `rustfmt` is not able to format that, it runs undefinitely
    ((((((((((((double(double((double(double((double(double((double(double((double(double((double(double((double(double((double(double(cell(var(Witness(0), Curr)))) + cell(var(Witness(6), Curr))))) + cell(var(Witness(7), Curr))))) + cell(var(Witness(8), Curr))))) + cell(var(Witness(9), Curr))))) + cell(var(Witness(10), Curr))))) + cell(var(Witness(11), Curr))))) + cell(var(Witness(12), Curr))))) + cell(var(Witness(13), Curr))) - cell(var(Witness(1), Curr))) + (alpha_pow(1) * ((double((double((double((double((double((double((double((double(cell(var(Witness(2), Curr))) + x_0)) + x_1)) + x_2)) + x_3)) + x_4)) + x_5)) + x_6)) + x_7) - cell(var(Witness(4), Curr))))) + (alpha_pow(2) * ((double((double((double((double((double((double((double((double(cell(var(Witness(3), Curr))) + (x_0 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(6), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(6), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_1 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(7), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(7), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_2 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(8), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(8), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_3 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(9), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(9), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_4 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(10), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(10), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_5 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(11), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(11), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_6 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(12), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(12), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000"))))) + (x_7 + ((((field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000") * cell(var(Witness(13), Curr))) + field("0x0000000000000000000000000000000000000000000000000000000000000003")) * cell(var(Witness(13), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ED00000000")))) - cell(var(Witness(5), Curr))))) + (alpha_pow(3) * ((((((cell(var(Witness(6), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(6), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(6), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(6), Curr))))) + (alpha_pow(4) * ((((((cell(var(Witness(7), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(7), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(7), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(7), Curr))))) + (alpha_pow(5) * ((((((cell(var(Witness(8), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(8), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(8), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(8), Curr))))) + (alpha_pow(6) * ((((((cell(var(Witness(9), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(9), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(9), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(9), Curr))))) + (alpha_pow(7) * ((((((cell(var(Witness(10), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(10), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(10), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(10), Curr))))) + (alpha_pow(8) * ((((((cell(var(Witness(11), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(11), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(11), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(11), Curr))))) + (alpha_pow(9) * ((((((cell(var(Witness(12), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(12), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(12), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(12), Curr))))) + (alpha_pow(10) * ((((((cell(var(Witness(13), Curr)) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(13), Curr))) + field("0x000000000000000000000000000000000000000000000000000000000000000B")) * cell(var(Witness(13), Curr))) + field("0x40000000000000000000000000000000224698FC094CF91B992D30ECFFFFFFFB")) * cell(var(Witness(13), Curr)))))
}

#[cfg(test)]
mod tests {
    use kimchi::{
        circuits::expr::Linearization,
        linearization::{constraints_expr, linearization_columns},
    };
    use mina_hasher::Fp;
    #[cfg(target_family = "wasm")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn generate_plonk() {
        let lookup_configuration = None;
        let evaluated_cols = linearization_columns::<Fp>(lookup_configuration);
        let (linearization, _powers_of_alpha) =
            constraints_expr::<Fp>(false, false, lookup_configuration, false, false);

        let Linearization {
            constant_term: _,
            mut index_terms,
        } = linearization.linearize(evaluated_cols).unwrap();

        // HashMap deliberately uses an unstable order; here we sort to ensure that the output is
        // consistent when printing.
        index_terms.sort_by(|(x, _), (y, _)| x.cmp(y));

        let mut other_terms: Vec<_> = index_terms
            .iter()
            .map(|(col, expr)| (format!("{:?}", col), format!("{}", expr.ocaml_str())))
            .collect();

        // Convert to Rust code
        for (v, terms) in &mut other_terms {
            // Replace "let a = b in " with "let a = { b };", to make the output a Rust syntax
            *terms = terms.replace(" in ", "};");
            *terms = terms.replace("=", "={");

            println!("VALUE={:?} TERMS=\n{}\n", v, terms);
        }
    }
}
