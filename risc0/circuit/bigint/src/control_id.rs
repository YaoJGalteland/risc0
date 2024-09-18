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

// Automatically generated by:
//   cargo run -p risc0-circuit-bigint -F make_control_ids --bin make_control_ids > risc0/circuit/bigint/src/control_id.rs

use risc0_zkp::{core::digest::Digest, digest};

pub const RSA_256_X1_CONTROL_ID: Digest =
    digest!("b64d9520da37fe3fb06b5254065d140238bd165a7b41bf51ef3fc25433e98503");
pub const RSA_256_X1_CONTROL_ROOT: Digest =
    digest!("39b42c22d723a56dda3ab641aecbf92442a7ff4415fe800b814a3d086a159e2b");

pub const RSA_256_X2_CONTROL_ID: Digest =
    digest!("c9b17f4d82070833e6d6023304b4111ef1e48767f204d66e705eff2074c28655");
pub const RSA_256_X2_CONTROL_ROOT: Digest =
    digest!("32d34e64a3d70b09ed889d63d9df1c4ffa468b1a150fc31246bc994c73e43b14");

pub const RSA_3072_X15_CONTROL_ID: Digest =
    digest!("5b8c6338762a361b58cdf418544cb321a5b11961e6907e05759a30130f892801");
pub const RSA_3072_X15_CONTROL_ROOT: Digest =
    digest!("a5568767aa62fd111843d721c213910cdad0f03386db8269301f0009efd86041");

pub const NONDET_INV_TEST_8_CONTROL_ID: Digest =
    digest!("06b2d40f2642aa45bf36d376f8620e6cdc4e6b574924f31a1f229e6b9345422f");
pub const NONDET_INV_TEST_8_CONTROL_ROOT: Digest =
    digest!("9161051265033a043167c53126a2c052e9022d69ba54256bdd75260bfca99e37");

pub const CONST_ADD_TEST_8_CONTROL_ID: Digest =
    digest!("6a8afb2846b4af55dd68cb5b58a059493fbe0b365a21322fcd39b5308ee40917");
pub const CONST_ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("1fca6f6d47c86268fe193a123eae674f3b56e55fe454d7743583f1492e8d3865");

pub const CONST_ADD_ALT_TEST_16_CONTROL_ID: Digest =
    digest!("97a96a16ce3b5d0f84cbb4488f18a359d626ac0b74124413f679634df19b0567");
pub const CONST_ADD_ALT_TEST_16_CONTROL_ROOT: Digest =
    digest!("4bd8e046e365a53ba83f1e5860c9634379fd941863542135837faa302be8e706");

pub const CONST_MUL_TEST_8_CONTROL_ID: Digest =
    digest!("82aee8246da8193e045a901c078ee9752d742d35613f8f3e05ac1c18c6454e70");
pub const CONST_MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("28ac5116566e2b1f312d204cfd6d972e2d70a51a75d9f93b6dda101e2b513649");

pub const ADD_TEST_8_CONTROL_ID: Digest =
    digest!("7a5e693d29efca181c14524283d3c64a2e39d15b1e003744c985d46f62ccb554");
pub const ADD_TEST_8_CONTROL_ROOT: Digest =
    digest!("b6666525d30389428d32e651c6742647a1d6de69b6639e30d7c30c616c879855");

pub const ADD_TEST_16_CONTROL_ID: Digest =
    digest!("0949613353166c5c842b5a0b3fbc355f16a39b03d534652a2fbf870fa134c02a");
pub const ADD_TEST_16_CONTROL_ROOT: Digest =
    digest!("87b5c15b6f4e166af7ef4f30a9bb9319cfe07e11258a0f6c5f7e6d5f3f872c0e");

pub const ADD_TEST_128_CONTROL_ID: Digest =
    digest!("79b36c33fbb721755a0a3f1336e909712e2c6c4c4e2fab33ecfaf32d8ed32754");
pub const ADD_TEST_128_CONTROL_ROOT: Digest =
    digest!("0047b84a1718a954d33fda38e74627490ac1c60ab71f125c559d76754fe72e22");

pub const CONST_ONE_TEST_8_CONTROL_ID: Digest =
    digest!("1f8fc4087a0e1d4c2348c16cc0ce5e721776ad1803b3eb1754851a2e4e81f002");
pub const CONST_ONE_TEST_8_CONTROL_ROOT: Digest =
    digest!("a067705e9fc51c2b5128843c7f2c9a10691dbe271d40fd3f3d56cf00930e7775");

pub const CONST_TWOBYTE_TEST_16_CONTROL_ID: Digest =
    digest!("ee9b1730160a75140d0d470ab20f353eff375b346647012ec8733d41e821a733");
pub const CONST_TWOBYTE_TEST_16_CONTROL_ROOT: Digest =
    digest!("9a258702faa4b75a2e7fb96a254a0e14b65619562d0b5a0aa4c80917b6cdb62b");

pub const SUB_TEST_8_CONTROL_ID: Digest =
    digest!("91afae323ffc1f227c0d731ca06ec75890875a0b8fd1211cf4e4486df2b9813d");
pub const SUB_TEST_8_CONTROL_ROOT: Digest =
    digest!("ba12ed3af526b04e5248501219fa09271cd6a705ab0ce132c329ac7777f84b02");

pub const SUB_TEST_128_CONTROL_ID: Digest =
    digest!("60e1270202c0d9453c421a2cd02c773ed8efbe2fe38c4b479729546ed8e56c51");
pub const SUB_TEST_128_CONTROL_ROOT: Digest =
    digest!("57d56a4322dadb0b86a13670e4cc114550d0480c0f143423dd1fcc1632c3bd0d");

pub const MUL_TEST_8_CONTROL_ID: Digest =
    digest!("b84be75d8dc2e6270149bd07cc3fad38d8f84226400cc85dc7b3ab72b1aa6207");
pub const MUL_TEST_8_CONTROL_ROOT: Digest =
    digest!("08be6355563b4b47ade35c1391e89a450e5bf3264f41f03abf532e746864274b");

pub const MUL_TEST_128_CONTROL_ID: Digest =
    digest!("80b4823af8eaaa1c1a844f551a4ff055e49bfc589ed8d701069b64637694f770");
pub const MUL_TEST_128_CONTROL_ROOT: Digest =
    digest!("a3c5f2432fbcf66e70d9c1770ae8bd124c435b471c124e3f249b054ed14a0249");

pub const REDUCE_TEST_8_CONTROL_ID: Digest =
    digest!("9a32102c3c7772299d47435a0c079012c41573215b678454210a045615cd0f5e");
pub const REDUCE_TEST_8_CONTROL_ROOT: Digest =
    digest!("1a23e116f3e13f02e0c0325cb37ea75979ee7947bfd8940ca798350ad5ef273c");

pub const REDUCE_TEST_128_CONTROL_ID: Digest =
    digest!("d9803b4d5d64c238de13315770ed3b372332622fedd05a0950fa5b376b368c19");
pub const REDUCE_TEST_128_CONTROL_ROOT: Digest =
    digest!("9ea6c9016ca0036732577e6e4feabf43e418b800a9bb8d6d0ed5e04c47ef2757");

pub const ECDSA_VERIFY_8_CONTROL_ID: Digest =
    digest!("595cac4a638f747424ffe24dd008443fb9e0fb1c82b79a575998e7317dbb0417");
pub const ECDSA_VERIFY_8_CONTROL_ROOT: Digest =
    digest!("e1f16f4147846f04f828353d18fb4569dc99a509577b5040a29e994209243c05");

pub const ECDSA_VERIFY_32_CONTROL_ID: Digest =
    digest!("69a2b62b79639561ad98bc3341591f6f487843244312dc54300ff61cc5aec454");
pub const ECDSA_VERIFY_32_CONTROL_ROOT: Digest =
    digest!("961f5d4383fd6125631b47607c9b7617f9cbea1a6984911a8a451700abcc4212");

pub const EC_ADD_RZ8TEST1_CONTROL_ID: Digest =
    digest!("3e49b0741ef5b049ea19b462b660ed714d69612e45fadb60327e3972f1563009");
pub const EC_ADD_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("9b19373eb9704207c5b8f118b3ccad09b5a6bf607d9f3d5abf848d3f2889c80e");

pub const EC_ADD_SECP256K1_CONTROL_ID: Digest =
    digest!("b218f41b7c802d1595378a3164d0df6c7d196c1dfa3bed4956d2797248b8a224");
pub const EC_ADD_SECP256K1_CONTROL_ROOT: Digest =
    digest!("ba52d22246c7c5307498c6183cf9512f8a1e205589e2a46a23d01b73e024b529");

pub const EC_DOUB_RZ8TEST1_CONTROL_ID: Digest =
    digest!("1f7120464fe21d3ce400ff318079e85394661267ce06a01070be113dc3e7516d");
pub const EC_DOUB_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("a06ac142a65efa3f1e3bd7161bcf41304be53913e7065c01cec1c31006447821");

pub const EC_DOUB_SECP256K1_CONTROL_ID: Digest =
    digest!("1ecc8815d86f891d758c9b329350685b359ab769141d804934b5764023bdae62");
pub const EC_DOUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("24d66c2f3a3e914b694d2e4515f75369795f32324037425ee5242f1ff6a28f31");

pub const EC_MUL_RZ8TEST1_CONTROL_ID: Digest =
    digest!("6826671870573b6b451e932c7317ee4199eb2673b11f472776c88a3ef0bf6033");
pub const EC_MUL_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("82d6115ff921b5663208e2504288ca5a692d2c47bdc4661b73414311f253e74c");

pub const EC_MUL_SECP256K1_CONTROL_ID: Digest =
    digest!("20be4350394a966515f914452178c650627d6f57cdbe6b04c38b7c1e583f5f4a");
pub const EC_MUL_SECP256K1_CONTROL_ROOT: Digest =
    digest!("4dc8081c7eab765ef4af435618a5a43723d89849af81160998679620a48f1112");

pub const EC_NEG_RZ8TEST1_CONTROL_ID: Digest =
    digest!("bc8fd97360bb286bc468c320c435cb283c6f2400528b6a738865302da8417741");
pub const EC_NEG_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("1e92f00315ff353ce874d0711d3e3e3a5d13f368c254165e8dccb572e3c2d93a");

pub const EC_NEG_SECP256K1_CONTROL_ID: Digest =
    digest!("55494e3c5067575436d6b30b82465f60b5f7b947ec4e423eb0c0795cabd44c50");
pub const EC_NEG_SECP256K1_CONTROL_ROOT: Digest =
    digest!("6a57de42c3e336197f4ae46b183b1b57cba8090d4939a50c4dca3f0fdae7bb73");

pub const EC_SUB_RZ8TEST1_CONTROL_ID: Digest =
    digest!("54c2b4477749ea17aa64a51150e99202ca6afc2ae7457050cc4eca5edb78fd40");
pub const EC_SUB_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("db54be0189f02e5cdcf65a7502961600e8874f6b2f23f4378d550d73b492860b");

pub const EC_SUB_SECP256K1_CONTROL_ID: Digest =
    digest!("32874e743ee63a18c463df389d3a2c1692117d0fe3673b32d8490a38d7f35621");
pub const EC_SUB_SECP256K1_CONTROL_ROOT: Digest =
    digest!("c3c90b60f6cd790a9b410312834f66560e3ea324292bae543f4934613966564a");

pub const EC_PTS_EQ_RZ8TEST1_CONTROL_ID: Digest =
    digest!("23716c007eb3245bf1ca6244de8f4d090c99500cf8d2f42e9e96d763e1eff31c");
pub const EC_PTS_EQ_RZ8TEST1_CONTROL_ROOT: Digest =
    digest!("765de037db9ec4171e4d5c76e3935f0a3de2f575a7ce645c57c354212825c365");

pub const EC_PTS_EQ_SECP256K1_CONTROL_ID: Digest =
    digest!("b6f0cd6a1972504a7de3180fd9fc6b0c50f4656ae7a84a567744fd64d024dd56");
pub const EC_PTS_EQ_SECP256K1_CONTROL_ROOT: Digest =
    digest!("7b923f30d5298427e2ded313b50f1843ae73595115339b104a6fde13d42f505f");

pub const REP_EC_ADD_SECP256K1_R5_CONTROL_ID: Digest =
    digest!("e6654f20168370550b6578102e916e74c20157177b62b45d595c0922fd2a714f");
pub const REP_EC_ADD_SECP256K1_R5_CONTROL_ROOT: Digest =
    digest!("f7ff26598e3aff03bfa0491c0c837a28e8e52812969e082d2f95ff3e2ce65f43");

pub const REP_EC_ADD_SECP256K1_R10_CONTROL_ID: Digest =
    digest!("3272d27670acfc166aa124052242d226bc59715867dfdf420126f941b212a54b");
pub const REP_EC_ADD_SECP256K1_R10_CONTROL_ROOT: Digest =
    digest!("ac7a5703538216086938ae41d918b3612e38fd6c1906471e5ce6877314f95e4f");

pub const REP_EC_ADD_SECP256K1_R256_CONTROL_ID: Digest =
    digest!("9e1c0718d1f2a85b9410f9387ed6316d656daa3178df1e289254782dea7c190e");
pub const REP_EC_ADD_SECP256K1_R256_CONTROL_ROOT: Digest =
    digest!("0af3d939878d0921803a08350b6e605bc949f84d7c13a06912a41556b51f0505");

pub const REP_EC_DOUB_SECP256K1_R5_CONTROL_ID: Digest =
    digest!("318a500b5ec17e47fdd1d47624cb11532b24fe609a74c2095f6e26504ed31d2c");
pub const REP_EC_DOUB_SECP256K1_R5_CONTROL_ROOT: Digest =
    digest!("df0f3010a4be6571119afd13fa2842422821440786e6be581802fa6baaa6090b");

pub const REP_EC_DOUB_SECP256K1_R10_CONTROL_ID: Digest =
    digest!("e4b22a149a8019521a7c6a3ab4fa5f15a86bae24e7041c511cb43d6e99eae521");
pub const REP_EC_DOUB_SECP256K1_R10_CONTROL_ROOT: Digest =
    digest!("895b27089623015acd4c4904b34e063fad2d853f336ca96576586e35ffb05315");

pub const REP_EC_DOUB_SECP256K1_R256_CONTROL_ID: Digest =
    digest!("3f681837c6c2375586851645e7463042cfa46e5d35908362ab5b1c4c05b7b536");
pub const REP_EC_DOUB_SECP256K1_R256_CONTROL_ROOT: Digest =
    digest!("43eeb350be7e232701203c41ca2b7039eefde1650e3113341dbcfe210cc3e670");
