pub trait PrimalityTest {
    fn is_prime(&self, n: u64) -> bool;
}

pub mod primality_test_algorithms {
    #[derive(Debug, PartialEq, Eq)]
    pub enum DivisionUpperBound {
        Whole,
        Half,
        Square,
    }

    pub struct NaiveTrialDivision {
        pub increment: u64,
        pub upper_bound: DivisionUpperBound,
    }

    impl super::PrimalityTest for NaiveTrialDivision {
        fn is_prime(&self, n: u64) -> bool {
            if n == 2 || n == 3 {
                return true;
            }

            if n <= 1 || n % 2 == 0 {
                return false;
            }

            let mut i = 3;
            let upper_bound = match self.upper_bound {
                DivisionUpperBound::Whole => n,
                DivisionUpperBound::Half => n / 2,
                DivisionUpperBound::Square => (n as f64).sqrt() as u64 + 1,
            };

            while i < upper_bound && n % i != 0 {
                i += self.increment;
            }

            i >= upper_bound
        }
    }

    pub struct SixKOneDivision {
        pub upper_bound: DivisionUpperBound,
    }

    impl super::PrimalityTest for SixKOneDivision {
        fn is_prime(&self, n: u64) -> bool {
            if n == 2 || n == 3 || n == 5 || n == 7 {
                return true;
            }

            if n <= 1 || n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
                return false;
            }

            let upper_bound = match self.upper_bound {
                DivisionUpperBound::Whole => n,
                DivisionUpperBound::Half => n / 2,
                DivisionUpperBound::Square => (n as f64).sqrt() as u64,
            };
            let mut k1 = 5;
            let mut k2 = 7;

            while (k1 <= upper_bound || k2 <= upper_bound)
                && n % k1 != 0
                && n % k2 != 0
            {
                k1 += 6;
                k2 += 6;
            }

            if k1 < upper_bound && k2 < upper_bound {
                return false;
            }

            if self.upper_bound == DivisionUpperBound::Square
                && (n % k1 == 0 || n % k2 == 0)
            {
                return false;
            }

            n % k1 == 0
                || n % k2 == 0
                || (k1 >= upper_bound && k2 >= upper_bound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::primality_test_algorithms::*;
    use super::*;

    const FIRST_1000_PRIMES: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
        71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139,
        149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
        227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
        307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383,
        389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
        467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569,
        571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647,
        653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839,
        853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
        947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
        1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091, 1093, 1097, 1103,
        1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201,
        1213, 1217, 1223, 1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289,
        1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373, 1381,
        1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471,
        1481, 1483, 1487, 1489, 1493, 1499, 1511, 1523, 1531, 1543, 1549, 1553,
        1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621,
        1627, 1637, 1657, 1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723,
        1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811, 1823,
        1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913,
        1931, 1933, 1949, 1951, 1973, 1979, 1987, 1993, 1997, 1999, 2003, 2011,
        2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099,
        2111, 2113, 2129, 2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207,
        2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287, 2293,
        2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381,
        2383, 2389, 2393, 2399, 2411, 2417, 2423, 2437, 2441, 2447, 2459, 2467,
        2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591,
        2593, 2609, 2617, 2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683,
        2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731, 2741, 2749,
        2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843,
        2851, 2857, 2861, 2879, 2887, 2897, 2903, 2909, 2917, 2927, 2939, 2953,
        2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049,
        3061, 3067, 3079, 3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169,
        3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257, 3259,
        3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359,
        3361, 3371, 3373, 3389, 3391, 3407, 3413, 3433, 3449, 3457, 3461, 3463,
        3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539, 3541, 3547,
        3557, 3559, 3571, 3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637,
        3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727, 3733,
        3739, 3761, 3767, 3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847,
        3851, 3853, 3863, 3877, 3881, 3889, 3907, 3911, 3917, 3919, 3923, 3929,
        3931, 3943, 3947, 3967, 3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027,
        4049, 4051, 4057, 4073, 4079, 4091, 4093, 4099, 4111, 4127, 4129, 4133,
        4139, 4153, 4157, 4159, 4177, 4201, 4211, 4217, 4219, 4229, 4231, 4241,
        4243, 4253, 4259, 4261, 4271, 4273, 4283, 4289, 4297, 4327, 4337, 4339,
        4349, 4357, 4363, 4373, 4391, 4397, 4409, 4421, 4423, 4441, 4447, 4451,
        4457, 4463, 4481, 4483, 4493, 4507, 4513, 4517, 4519, 4523, 4547, 4549,
        4561, 4567, 4583, 4591, 4597, 4603, 4621, 4637, 4639, 4643, 4649, 4651,
        4657, 4663, 4673, 4679, 4691, 4703, 4721, 4723, 4729, 4733, 4751, 4759,
        4783, 4787, 4789, 4793, 4799, 4801, 4813, 4817, 4831, 4861, 4871, 4877,
        4889, 4903, 4909, 4919, 4931, 4933, 4937, 4943, 4951, 4957, 4967, 4969,
        4973, 4987, 4993, 4999, 5003, 5009, 5011, 5021, 5023, 5039, 5051, 5059,
        5077, 5081, 5087, 5099, 5101, 5107, 5113, 5119, 5147, 5153, 5167, 5171,
        5179, 5189, 5197, 5209, 5227, 5231, 5233, 5237, 5261, 5273, 5279, 5281,
        5297, 5303, 5309, 5323, 5333, 5347, 5351, 5381, 5387, 5393, 5399, 5407,
        5413, 5417, 5419, 5431, 5437, 5441, 5443, 5449, 5471, 5477, 5479, 5483,
        5501, 5503, 5507, 5519, 5521, 5527, 5531, 5557, 5563, 5569, 5573, 5581,
        5591, 5623, 5639, 5641, 5647, 5651, 5653, 5657, 5659, 5669, 5683, 5689,
        5693, 5701, 5711, 5717, 5737, 5741, 5743, 5749, 5779, 5783, 5791, 5801,
        5807, 5813, 5821, 5827, 5839, 5843, 5849, 5851, 5857, 5861, 5867, 5869,
        5879, 5881, 5897, 5903, 5923, 5927, 5939, 5953, 5981, 5987, 6007, 6011,
        6029, 6037, 6043, 6047, 6053, 6067, 6073, 6079, 6089, 6091, 6101, 6113,
        6121, 6131, 6133, 6143, 6151, 6163, 6173, 6197, 6199, 6203, 6211, 6217,
        6221, 6229, 6247, 6257, 6263, 6269, 6271, 6277, 6287, 6299, 6301, 6311,
        6317, 6323, 6329, 6337, 6343, 6353, 6359, 6361, 6367, 6373, 6379, 6389,
        6397, 6421, 6427, 6449, 6451, 6469, 6473, 6481, 6491, 6521, 6529, 6547,
        6551, 6553, 6563, 6569, 6571, 6577, 6581, 6599, 6607, 6619, 6637, 6653,
        6659, 6661, 6673, 6679, 6689, 6691, 6701, 6703, 6709, 6719, 6733, 6737,
        6761, 6763, 6779, 6781, 6791, 6793, 6803, 6823, 6827, 6829, 6833, 6841,
        6857, 6863, 6869, 6871, 6883, 6899, 6907, 6911, 6917, 6947, 6949, 6959,
        6961, 6967, 6971, 6977, 6983, 6991, 6997, 7001, 7013, 7019, 7027, 7039,
        7043, 7057, 7069, 7079, 7103, 7109, 7121, 7127, 7129, 7151, 7159, 7177,
        7187, 7193, 7207, 7211, 7213, 7219, 7229, 7237, 7243, 7247, 7253, 7283,
        7297, 7307, 7309, 7321, 7331, 7333, 7349, 7351, 7369, 7393, 7411, 7417,
        7433, 7451, 7457, 7459, 7477, 7481, 7487, 7489, 7499, 7507, 7517, 7523,
        7529, 7537, 7541, 7547, 7549, 7559, 7561, 7573, 7577, 7583, 7589, 7591,
        7603, 7607, 7621, 7639, 7643, 7649, 7669, 7673, 7681, 7687, 7691, 7699,
        7703, 7717, 7723, 7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823,
        7829, 7841, 7853, 7867, 7873, 7877, 7879, 7883, 7901, 7907, 7919,
    ];

    const FIRST_1000_COMPOSITES: &[u64] = &[
        4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28, 30,
        32, 33, 34, 35, 36, 38, 39, 40, 42, 44, 45, 46, 48, 49, 50, 51, 52, 54,
        55, 56, 57, 58, 60, 62, 63, 64, 65, 66, 68, 69, 70, 72, 74, 75, 76, 77,
        78, 80, 81, 82, 84, 85, 86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 98, 99,
        100, 102, 104, 105, 106, 108, 110, 111, 112, 114, 115, 116, 117, 118,
        119, 120, 121, 122, 123, 124, 125, 126, 128, 129, 130, 132, 133, 134,
        135, 136, 138, 140, 141, 142, 143, 144, 145, 146, 147, 148, 150, 152,
        153, 154, 155, 156, 158, 159, 160, 161, 162, 164, 165, 166, 168, 169,
        170, 171, 172, 174, 175, 176, 177, 178, 180, 182, 183, 184, 185, 186,
        187, 188, 189, 190, 192, 194, 195, 196, 198, 200, 201, 202, 203, 204,
        205, 206, 207, 208, 209, 210, 212, 213, 214, 215, 216, 217, 218, 219,
        220, 221, 222, 224, 225, 226, 228, 230, 231, 232, 234, 235, 236, 237,
        238, 240, 242, 243, 244, 245, 246, 247, 248, 249, 250, 252, 253, 254,
        255, 256, 258, 259, 260, 261, 262, 264, 265, 266, 267, 268, 270, 272,
        273, 274, 275, 276, 278, 279, 280, 282, 284, 285, 286, 287, 288, 289,
        290, 291, 292, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304,
        305, 306, 308, 309, 310, 312, 314, 315, 316, 318, 319, 320, 321, 322,
        323, 324, 325, 326, 327, 328, 329, 330, 332, 333, 334, 335, 336, 338,
        339, 340, 341, 342, 343, 344, 345, 346, 348, 350, 351, 352, 354, 355,
        356, 357, 358, 360, 361, 362, 363, 364, 365, 366, 368, 369, 370, 371,
        372, 374, 375, 376, 377, 378, 380, 381, 382, 384, 385, 386, 387, 388,
        390, 391, 392, 393, 394, 395, 396, 398, 399, 400, 402, 403, 404, 405,
        406, 407, 408, 410, 411, 412, 413, 414, 415, 416, 417, 418, 420, 422,
        423, 424, 425, 426, 427, 428, 429, 430, 432, 434, 435, 436, 437, 438,
        440, 441, 442, 444, 445, 446, 447, 448, 450, 451, 452, 453, 454, 455,
        456, 458, 459, 460, 462, 464, 465, 466, 468, 469, 470, 471, 472, 473,
        474, 475, 476, 477, 478, 480, 481, 482, 483, 484, 485, 486, 488, 489,
        490, 492, 493, 494, 495, 496, 497, 498, 500, 501, 502, 504, 505, 506,
        507, 508, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519, 520, 522,
        524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537,
        538, 539, 540, 542, 543, 544, 545, 546, 548, 549, 550, 551, 552, 553,
        554, 555, 556, 558, 559, 560, 561, 562, 564, 565, 566, 567, 568, 570,
        572, 573, 574, 575, 576, 578, 579, 580, 581, 582, 583, 584, 585, 586,
        588, 589, 590, 591, 592, 594, 595, 596, 597, 598, 600, 602, 603, 604,
        605, 606, 608, 609, 610, 611, 612, 614, 615, 616, 618, 620, 621, 622,
        623, 624, 625, 626, 627, 628, 629, 630, 632, 633, 634, 635, 636, 637,
        638, 639, 640, 642, 644, 645, 646, 648, 649, 650, 651, 652, 654, 655,
        656, 657, 658, 660, 662, 663, 664, 665, 666, 667, 668, 669, 670, 671,
        672, 674, 675, 676, 678, 679, 680, 681, 682, 684, 685, 686, 687, 688,
        689, 690, 692, 693, 694, 695, 696, 697, 698, 699, 700, 702, 703, 704,
        705, 706, 707, 708, 710, 711, 712, 713, 714, 715, 716, 717, 718, 720,
        721, 722, 723, 724, 725, 726, 728, 729, 730, 731, 732, 734, 735, 736,
        737, 738, 740, 741, 742, 744, 745, 746, 747, 748, 749, 750, 752, 753,
        754, 755, 756, 758, 759, 760, 762, 763, 764, 765, 766, 767, 768, 770,
        771, 772, 774, 775, 776, 777, 778, 779, 780, 781, 782, 783, 784, 785,
        786, 788, 789, 790, 791, 792, 793, 794, 795, 796, 798, 799, 800, 801,
        802, 803, 804, 805, 806, 807, 808, 810, 812, 813, 814, 815, 816, 817,
        818, 819, 820, 822, 824, 825, 826, 828, 830, 831, 832, 833, 834, 835,
        836, 837, 838, 840, 841, 842, 843, 844, 845, 846, 847, 848, 849, 850,
        851, 852, 854, 855, 856, 858, 860, 861, 862, 864, 865, 866, 867, 868,
        869, 870, 871, 872, 873, 874, 875, 876, 878, 879, 880, 882, 884, 885,
        886, 888, 889, 890, 891, 892, 893, 894, 895, 896, 897, 898, 899, 900,
        901, 902, 903, 904, 905, 906, 908, 909, 910, 912, 913, 914, 915, 916,
        917, 918, 920, 921, 922, 923, 924, 925, 926, 927, 928, 930, 931, 932,
        933, 934, 935, 936, 938, 939, 940, 942, 943, 944, 945, 946, 948, 949,
        950, 951, 952, 954, 955, 956, 957, 958, 959, 960, 961, 962, 963, 964,
        965, 966, 968, 969, 970, 972, 973, 974, 975, 976, 978, 979, 980, 981,
        982, 984, 985, 986, 987, 988, 989, 990, 992, 993, 994, 995, 996, 998,
        999, 1000, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1010, 1011,
        1012, 1014, 1015, 1016, 1017, 1018, 1020, 1022, 1023, 1024, 1025, 1026,
        1027, 1028, 1029, 1030, 1032, 1034, 1035, 1036, 1037, 1038, 1040, 1041,
        1042, 1043, 1044, 1045, 1046, 1047, 1048, 1050, 1052, 1053, 1054, 1055,
        1056, 1057, 1058, 1059, 1060, 1062, 1064, 1065, 1066, 1067, 1068, 1070,
        1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082,
        1083, 1084, 1085, 1086, 1088, 1089, 1090, 1092, 1094, 1095, 1096, 1098,
        1099, 1100, 1101, 1102, 1104, 1105, 1106, 1107, 1108, 1110, 1111, 1112,
        1113, 1114, 1115, 1116, 1118, 1119, 1120, 1121, 1122, 1124, 1125, 1126,
        1127, 1128, 1130, 1131, 1132, 1133, 1134, 1135, 1136, 1137, 1138, 1139,
        1140, 1141, 1142, 1143, 1144, 1145, 1146, 1147, 1148, 1149, 1150, 1152,
        1154, 1155, 1156, 1157, 1158, 1159, 1160, 1161, 1162, 1164, 1165, 1166,
        1167, 1168, 1169, 1170, 1172, 1173, 1174, 1175, 1176, 1177, 1178, 1179,
        1180, 1182, 1183, 1184, 1185, 1186, 1188, 1189, 1190, 1191, 1192, 1194,
        1195, 1196, 1197,
    ];

    #[test]
    fn test_whole_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_whole_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Half,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Half,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Square,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Square,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(ntd.is_prime(*p), true);
        }

        assert_eq!(ntd.is_prime(0), false);
        assert_eq!(ntd.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_whole_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(skod.is_prime(*p), true);
        }

        assert_eq!(skod.is_prime(0), false);
        assert_eq!(skod.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(skod.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Half,
        };

        for p in FIRST_1000_PRIMES {
            assert!(
                skod.is_prime(*p) == true,
                "is_prime({}): {}",
                *p,
                skod.is_prime(*p)
            );
        }

        assert_eq!(skod.is_prime(0), false);
        assert_eq!(skod.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert_eq!(skod.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Square,
        };

        for p in FIRST_1000_PRIMES {
            assert_eq!(skod.is_prime(*p), true);
        }

        assert_eq!(skod.is_prime(0), false);
        assert_eq!(skod.is_prime(1), false);
        for p in FIRST_1000_COMPOSITES {
            assert!(
                skod.is_prime(*p) == false,
                "is_prime({}): {}",
                *p,
                skod.is_prime(*p)
            );
        }
    }
}
