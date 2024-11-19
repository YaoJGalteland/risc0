// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "num-bigint-dig")]
extern crate num_bigint_dig as num_bigint;

use num_bigint::BigUint;
use std::rc::Rc;

use risc0_bigint2::ec::{SECP256K1_PRIME, AffinePoint, WeierstrassCurve};
#[allow(unused)]
use risc0_zkvm::guest::env;

fn main() {
    let curve = Rc::new(WeierstrassCurve::<8> {
        prime: BigUint::from_slice(&SECP256K1_PRIME),
        a: BigUint::ZERO,
        b: BigUint::from(7u32),
    });

    let scalar = BigUint::from_slice(&[
        0x409f9918, 0xd218afb5, 0x81d5a9ae, 0x1aabde69, 0xe5cd569f, 0x478b33e5, 0xd5ff94e4,
        0x232ad1e3,
    ]);
    let point = AffinePoint {
        x: BigUint::from_slice(&[
            0x16f81798, 0x59f2815b, 0x2dce28d9, 0x029bfcdb, 0xce870b07, 0x55a06295, 0xf9dcbbac,
            0x79be667e,
        ]),
        y: BigUint::from_slice(&[
            0xfb10d4b8, 0x9c47d08f, 0xa6855419, 0xfd17b448, 0x0e1108a8, 0x5da4fbfc, 0x26a3c465,
            0x483ada77,
        ]),
        curve: Rc::clone(&curve),
    };

    let expected = AffinePoint {
        x: BigUint::from_slice(&[
            0xd430a92d, 0x5fdd93b4, 0x23c8434f, 0x1616b5ae, 0x2570e09c, 0x673f0dec, 0xb6bdef51, 0x2985d840,
        ]),
        y: BigUint::from_slice(&[
            0x34ab3c01, 0x0abd13e0, 0x8060d279, 0xa37beeeb, 0xb083593d, 0x4679f415, 0x6c4af2e8,
            0x1af7251d,
        ]),
        curve,
    };

    let result = risc0_bigint2::ec::mul(&scalar, &point);
    assert_eq!(result, expected);
}
