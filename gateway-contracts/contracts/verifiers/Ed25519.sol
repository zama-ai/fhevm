// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

import "./Sha512.sol";
import "./Ed25519_pow.sol";

library Ed25519 {
    function verify(
        bytes32 k,
        bytes32 r,
        bytes32 s,
        bytes memory m
    ) external pure returns (bool) {
        unchecked {
            uint256 hh;
            // Step 1: compute SHA-512(R, A, M)
            {
                bytes memory rs = new bytes(k.length + r.length + m.length);
                for (uint256 i = 0; i < r.length; i++) {
                    rs[i] = r[i];
                }
                for (uint256 i = 0; i < k.length; i++) {
                    rs[i + 32] = k[i];
                }
                    for (uint256 i = 0; i < m.length; i++) {
                    rs[i + 64] = m[i];
                }
                uint64[8] memory result = Sha512.hash(rs);

                uint256 h0 = uint256(result[0]) | uint256(result[1]) << 64 | uint256(result[2]) << 128 | uint256(result[3]) << 192;

                h0 =
                    ((h0 & 0xff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff) << 8) |
                    ((h0 & 0xff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00) >> 8);
                h0 =
                    ((h0 & 0xffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff) << 16) |
                    ((h0 & 0xffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000) >> 16);
                h0 =
                    ((h0 & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff) << 32) |
                    ((h0 & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000) >> 32);

                uint256 h1 = uint256(result[4]) | uint256(result[5]) << 64 | uint256(result[6]) << 128 | uint256(result[7]) << 192;

                h1 =
                    ((h1 & 0xff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff) << 8) |
                    ((h1 & 0xff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00) >> 8);
                h1 =
                    ((h1 & 0xffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff) << 16) |
                    ((h1 & 0xffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000) >> 16);
                h1 =
                    ((h1 & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff) << 32) |
                    ((h1 & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000) >> 32);
                hh = addmod(
                    h0,
                    mulmod(
                        h1,
                        0xfffffff_ffffffff_ffffffff_fffffffe_c6ef5bf4_737dcf70_d6ec3174_8d98951d,
                        0x10000000_00000000_00000000_00000000_14def9de_a2f79cd6_5812631a_5cf5d3ed
                    ),
                    0x10000000_00000000_00000000_00000000_14def9de_a2f79cd6_5812631a_5cf5d3ed
                );
            }
            // Step 2: unpack k
            k = bytes32(
                ((uint256(k) & 0xff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff) << 8) |
                    ((uint256(k) & 0xff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00) >> 8)
            );
            k = bytes32(
                ((uint256(k) & 0xffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff) << 16) |
                    ((uint256(k) & 0xffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000) >> 16)
            );
            k = bytes32(
                ((uint256(k) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff) << 32) |
                    ((uint256(k) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000) >> 32)
            );
            k = bytes32(
                ((uint256(k) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff) << 64) |
                    ((uint256(k) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff_00000000_00000000) >> 64)
            );
            k = bytes32((uint256(k) << 128) | (uint256(k) >> 128));
            uint256 ky = uint256(k) & 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff;
            uint256 kx;
            {
                uint256 ky2 = mulmod(ky, ky, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                uint256 u =
                    addmod(
                        ky2,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffec,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                uint256 v =
                    mulmod(
                        ky2,
                        0x52036cee_2b6ffe73_8cc74079_7779e898_00700a4d_4141d8ab_75eb4dca_135978a3,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    ) + 1;
                uint256 t = mulmod(u, v, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                (kx, ) = Ed25519_pow.pow22501(t);
                kx = mulmod(kx, kx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                kx = mulmod(
                    u,
                    mulmod(
                        mulmod(kx, kx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed),
                        t,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    ),
                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                );
                t = mulmod(
                    mulmod(kx, kx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed),
                    v,
                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                );
                if (t != u) {
                    if (t != 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed - u) {
                        return false;
                    }
                    kx = mulmod(
                        kx,
                        0x2b832480_4fc1df0b_2b4d0099_3dfbd7a7_2f431806_ad2fe478_c4ee1b27_4a0ea0b0,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
            }
            if ((kx & 1) != uint256(k) >> 255) {
                kx = 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed - kx;
            }
            // Verify s
            s = bytes32(
                ((uint256(s) & 0xff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff) << 8) |
                    ((uint256(s) & 0xff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00) >> 8)
            );
            s = bytes32(
                ((uint256(s) & 0xffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff) << 16) |
                    ((uint256(s) & 0xffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000) >> 16)
            );
            s = bytes32(
                ((uint256(s) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff) << 32) |
                    ((uint256(s) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000) >> 32)
            );
            s = bytes32(
                ((uint256(s) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff) << 64) |
                    ((uint256(s) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff_00000000_00000000) >> 64)
            );
            s = bytes32((uint256(s) << 128) | (uint256(s) >> 128));
            if (uint256(s) >= 0x10000000_00000000_00000000_00000000_14def9de_a2f79cd6_5812631a_5cf5d3ed) {
                return false;
            }
            uint256 vx;
            uint256 vu;
            uint256 vy;
            uint256 vv;
            // Step 3: compute multiples of k
            uint256[8][3][2] memory tables;
            {
                uint256 ks = ky + kx;
                uint256 kd = ky + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed - kx;
                uint256 k2dt =
                    mulmod(
                        mulmod(kx, ky, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed),
                        0x2406d9dc_56dffce7_198e80f2_eef3d130_00e0149a_8283b156_ebd69b94_26b2f159,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                uint256 kky = ky;
                uint256 kkx = kx;
                uint256 kku = 1;
                uint256 kkv = 1;
                {
                    uint256 xx =
                        mulmod(kkx, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy =
                        mulmod(kky, kku, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz =
                        mulmod(kku, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xx2 =
                        mulmod(xx, xx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy2 =
                        mulmod(yy, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xxyy =
                        mulmod(xx, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz2 =
                        mulmod(zz, zz, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    kkx = xxyy + xxyy;
                    kku = yy2 - xx2 + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                    kky = xx2 + yy2;
                    kkv = addmod(
                        zz2 + zz2,
                        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffda - kku,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
                {
                    uint256 xx =
                        mulmod(kkx, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy =
                        mulmod(kky, kku, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz =
                        mulmod(kku, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xx2 =
                        mulmod(xx, xx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy2 =
                        mulmod(yy, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xxyy =
                        mulmod(xx, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz2 =
                        mulmod(zz, zz, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    kkx = xxyy + xxyy;
                    kku = yy2 - xx2 + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                    kky = xx2 + yy2;
                    kkv = addmod(
                        zz2 + zz2,
                        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffda - kku,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
                {
                    uint256 xx =
                        mulmod(kkx, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy =
                        mulmod(kky, kku, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz =
                        mulmod(kku, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xx2 =
                        mulmod(xx, xx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 yy2 =
                        mulmod(yy, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 xxyy =
                        mulmod(xx, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 zz2 =
                        mulmod(zz, zz, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    kkx = xxyy + xxyy;
                    kku = yy2 - xx2 + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                    kky = xx2 + yy2;
                    kkv = addmod(
                        zz2 + zz2,
                        0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffda - kku,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
                uint256 cprod = 1;
                uint256[8][3][2] memory tables_ = tables;
                for (uint256 i = 0; ; i++) {
                    uint256 cs;
                    uint256 cd;
                    uint256 ct;
                    uint256 c2z;
                    {
                        uint256 cx =
                            mulmod(kkx, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 cy =
                            mulmod(kky, kku, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 cz =
                            mulmod(kku, kkv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        ct = mulmod(
                            kkx,
                            kky,
                            0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                        );
                        cs = cy + cx;
                        cd = cy - cx + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        c2z = cz + cz;
                    }
                    tables_[1][0][i] = cs;
                    tables_[1][1][i] = cd;
                    tables_[1][2][i] = mulmod(
                        ct,
                        0x2406d9dc_56dffce7_198e80f2_eef3d130_00e0149a_8283b156_ebd69b94_26b2f159,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                    tables_[0][0][i] = c2z;
                    tables_[0][1][i] = cprod;
                    cprod = mulmod(
                        cprod,
                        c2z,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                    if (i == 7) {
                        break;
                    }
                    uint256 ab =
                        mulmod(cs, ks, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 aa =
                        mulmod(cd, kd, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    uint256 ac =
                        mulmod(ct, k2dt, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    kkx = ab - aa + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                    kku = addmod(c2z, ac, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                    kky = ab + aa;
                    kkv = addmod(
                        c2z,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed - ac,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
                uint256 t;
                (cprod, t) = Ed25519_pow.pow22501(cprod);
                cprod = mulmod(cprod, cprod, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                cprod = mulmod(cprod, cprod, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                cprod = mulmod(cprod, cprod, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                cprod = mulmod(cprod, cprod, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                cprod = mulmod(cprod, cprod, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                cprod = mulmod(cprod, t, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                for (uint256 i = 7; ; i--) {
                    uint256 cinv =
                        mulmod(
                            cprod,
                            tables_[0][1][i],
                            0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                        );
                    tables_[1][0][i] = mulmod(
                        tables_[1][0][i],
                        cinv,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                    tables_[1][1][i] = mulmod(
                        tables_[1][1][i],
                        cinv,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                    tables_[1][2][i] = mulmod(
                        tables_[1][2][i],
                        cinv,
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                    if (i == 0) {
                        break;
                    }
                    cprod = mulmod(
                        cprod,
                        tables_[0][0][i],
                        0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                    );
                }
                tables_[0] = [
                    [
                        0x43e7ce9d_19ea5d32_9385a44c_321ea161_67c996e3_7dc6070c_97de49e3_7ac61db9,
                        0x40cff344_25d8ec30_a3bb74ba_58cd5854_fa1e3818_6ad0d31e_bc8ae251_ceb2c97e,
                        0x459bd270_46e8dd45_aea7008d_b87a5a8f_79067792_53d64523_58951859_9fdfbf4b,
                        0x69fdd1e2_8c23cc38_94d0c8ff_90e76f6d_5b6e4c2e_620136d0_4dd83c4a_51581ab9,
                        0x54dceb34_13ce5cfa_11196dfc_960b6eda_f4b380c6_d4d23784_19cc0279_ba49c5f3,
                        0x4e24184d_d71a3d77_eef3729f_7f8cf7c1_7224cf40_aa7b9548_b9942f3c_5084ceed,
                        0x5a0e5aab_20262674_ae117576_1cbf5e88_9b52a55f_d7ac5027_c228cebd_c8d2360a,
                        0x26239334_073e9b38_c6285955_6d451c3d_cc8d30e8_4b361174_f488eadd_e2cf17d9
                    ],
                    [
                        0x227e97c9_4c7c0933_d2e0c21a_3447c504_fe9ccf82_e8a05f59_ce881c82_eba0489f,
                        0x226a3e0e_cc4afec6_fd0d2884_13014a9d_bddecf06_c1a2f0bb_702ba77c_613d8209,
                        0x34d7efc8_51d45c5e_71efeb0f_235b7946_91de6228_877569b3_a8d52bf0_58b8a4a0,
                        0x3c1f5fb3_ca7166fc_e1471c9b_752b6d28_c56301ad_7b65e845_1b2c8c55_26726e12,
                        0x6102416c_f02f02ff_5be75275_f55f28db_89b2a9d2_456b860c_e22fc0e5_031f7cc5,
                        0x40adf677_f1bfdae0_57f0fd17_9c126179_18ddaa28_91a6530f_b1a4294f_a8665490,
                        0x61936f3c_41560904_6187b8ba_a978cbc9_b4789336_3ae5a3cc_7d909f36_35ae7f48,
                        0x562a9662_b6ec47f9_e979d473_c02b51e4_42336823_8c58ddb5_2f0e5c6a_180e6410
                    ],
                    [
                        0x3788bdb4_4f8632d4_2d0dbee5_eea1acc6_136cf411_e655624f_55e48902_c3bd5534,
                        0x6190cf2c_2a7b5ad7_69d594a8_2844f23b_4167fa7c_8ac30e51_aa6cfbeb_dcd4b945,
                        0x65f77870_96be9204_123a71f3_ac88a87b_e1513217_737d6a1e_2f3a13a4_3d7e3a9a,
                        0x23af32d_bfa67975_536479a7_a7ce74a0_2142147f_ac048018_7f1f1334_9cda1f2d,
                        0x64fc44b7_fc6841bd_db0ced8b_8b0fe675_9137ef87_ee966512_15fc1dbc_d25c64dc,
                        0x1434aa37_48b701d5_b69df3d7_d340c1fe_3f6b9c1e_fc617484_caadb47e_382f4475,
                        0x457a6da8_c962ef35_f2b21742_3e5844e9_d2353452_7e8ea429_0d24e3dd_f21720c6,
                        0x63b9540c_eb60ccb5_1e4d989d_956e053c_f2511837_efb79089_d2ff4028_4202c53d
                    ]
                ];
            }
            // Step 4: compute s*G - h*A
            {
                uint256 ss = uint256(s) << 3;
                uint256 hhh = hh + 0x80000000_00000000_00000000_00000000_a6f7cef5_17bce6b2_c09318d2_e7ae9f60;
                uint256 vvx = 0;
                uint256 vvu = 1;
                uint256 vvy = 1;
                uint256 vvv = 1;
                for (uint256 i = 252; ; i--) {
                    uint256 bit = 8 << i;
                    if ((ss & bit) != 0) {
                        uint256 ws;
                        uint256 wd;
                        uint256 wz;
                        uint256 wt;
                        {
                            uint256 wx =
                                mulmod(
                                    vvx,
                                    vvv,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            uint256 wy =
                                mulmod(
                                    vvy,
                                    vvu,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            ws = wy + wx;
                            wd = wy - wx + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                            wz = mulmod(
                                vvu,
                                vvv,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                            wt = mulmod(
                                vvx,
                                vvy,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        }
                        uint256 j = (ss >> i) & 7;
                        ss &= ~(7 << i);
                        uint256[8][3][2] memory tables_ = tables;
                        uint256 aa =
                            mulmod(
                                wd,
                                tables_[0][1][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ab =
                            mulmod(
                                ws,
                                tables_[0][0][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ac =
                            mulmod(
                                wt,
                                tables_[0][2][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        vvx = ab - aa + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvu = wz + ac;
                        vvy = ab + aa;
                        vvv = wz - ac + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                    }
                    if ((hhh & bit) != 0) {
                        uint256 ws;
                        uint256 wd;
                        uint256 wz;
                        uint256 wt;
                        {
                            uint256 wx =
                                mulmod(
                                    vvx,
                                    vvv,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            uint256 wy =
                                mulmod(
                                    vvy,
                                    vvu,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            ws = wy + wx;
                            wd = wy - wx + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                            wz = mulmod(
                                vvu,
                                vvv,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                            wt = mulmod(
                                vvx,
                                vvy,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        }
                        uint256 j = (hhh >> i) & 7;
                        hhh &= ~(7 << i);
                        uint256[8][3][2] memory tables_ = tables;
                        uint256 aa =
                            mulmod(
                                wd,
                                tables_[1][0][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ab =
                            mulmod(
                                ws,
                                tables_[1][1][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ac =
                            mulmod(
                                wt,
                                tables_[1][2][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        vvx = ab - aa + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvu = wz - ac + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvy = ab + aa;
                        vvv = wz + ac;
                    }
                    if (i == 0) {
                        uint256 ws;
                        uint256 wd;
                        uint256 wz;
                        uint256 wt;
                        {
                            uint256 wx =
                                mulmod(
                                    vvx,
                                    vvv,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            uint256 wy =
                                mulmod(
                                    vvy,
                                    vvu,
                                    0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                                );
                            ws = wy + wx;
                            wd = wy - wx + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                            wz = mulmod(
                                vvu,
                                vvv,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                            wt = mulmod(
                                vvx,
                                vvy,
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        }
                        uint256 j = hhh & 7;
                        uint256[8][3][2] memory tables_ = tables;
                        uint256 aa =
                            mulmod(
                                wd,
                                tables_[1][0][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ab =
                            mulmod(
                                ws,
                                tables_[1][1][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        uint256 ac =
                            mulmod(
                                wt,
                                tables_[1][2][j],
                                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                            );
                        vvx = ab - aa + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvu = wz - ac + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvy = ab + aa;
                        vvv = wz + ac;
                        break;
                    }
                    {
                        uint256 xx =
                            mulmod(vvx, vvv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 yy =
                            mulmod(vvy, vvu, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 zz =
                            mulmod(vvu, vvv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 xx2 =
                            mulmod(xx, xx, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 yy2 =
                            mulmod(yy, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 xxyy =
                            mulmod(xx, yy, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        uint256 zz2 =
                            mulmod(zz, zz, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
                        vvx = xxyy + xxyy;
                        vvu = yy2 - xx2 + 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed;
                        vvy = xx2 + yy2;
                        vvv = addmod(
                            zz2 + zz2,
                            0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffda - vvu,
                            0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
                        );
                    }
                }
                vx = vvx;
                vu = vvu;
                vy = vvy;
                vv = vvv;
            }
            // Step 5: compare the points
            (uint256 vi, uint256 vj) =
                Ed25519_pow.pow22501(mulmod(vu, vv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed));
            vi = mulmod(vi, vi, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vi = mulmod(vi, vi, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vi = mulmod(vi, vi, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vi = mulmod(vi, vi, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vi = mulmod(vi, vi, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vi = mulmod(vi, vj, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed);
            vx = mulmod(
                vx,
                mulmod(vi, vv, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed),
                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
            );
            vy = mulmod(
                vy,
                mulmod(vi, vu, 0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed),
                0x7fffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffed
            );
            bytes32 vr = bytes32(vy | (vx << 255));
            vr = bytes32(
                ((uint256(vr) & 0xff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff_00ff00ff) << 8) |
                    ((uint256(vr) & 0xff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00_ff00ff00) >> 8)
            );
            vr = bytes32(
                ((uint256(vr) & 0xffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff_0000ffff) << 16) |
                    ((uint256(vr) & 0xffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000_ffff0000) >> 16)
            );
            vr = bytes32(
                ((uint256(vr) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff) << 32) |
                    ((uint256(vr) & 0xffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000) >> 32)
            );
            vr = bytes32(
                ((uint256(vr) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff) << 64) |
                    ((uint256(vr) & 0xffffffff_ffffffff_00000000_00000000_ffffffff_ffffffff_00000000_00000000) >> 64)
            );
            vr = bytes32((uint256(vr) << 128) | (uint256(vr) >> 128));

            return vr == r;
        }
    }
}