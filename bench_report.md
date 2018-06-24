# HashMap Benchmark Report
Results from a full 100,000 sample of various test integer key value collection, see `./benches`. The benchmark system used an Intel i5-4670K.

For each sample of integer values there are 2 tests:
- ***insert_all***: All keys from the sample are inserted into a `HashMap` of default size.
- ***find_all***: All key-values from the sample are looked up from a pre-populated `HashMap`.

## default vs `int_hash`
_ℕ **2.53-9.06x** faster, random **1.18-3.90x** faster, 32× table **1.49-3.13x** faster_
```
name                            default_hash ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        9,430,402             3,726,791           -5,703,611  -60.48%   x 2.53
u64_natural::insert_all         7,658,142             2,135,107           -5,523,035  -72.12%   x 3.59
usize_natural::insert_all       7,677,566             2,131,158           -5,546,408  -72.24%   x 3.60
u32_natural::insert_all         7,478,978             2,520,760           -4,958,218  -66.30%   x 2.97
u16_natural::insert_all         5,534,763             1,913,495           -3,621,268  -65.43%   x 2.89
u128_natural::find_all          4,145,854             503,631             -3,642,223  -87.85%   x 8.23
u64_natural::find_all           3,596,242             396,861             -3,199,381  -88.96%   x 9.06
usize_natural::find_all         3,584,711             398,027             -3,186,684  -88.90%   x 9.01
u32_natural::find_all           2,972,257             342,627             -2,629,630  -88.47%   x 8.67
u16_natural::find_all           1,441,834             276,190             -1,165,644  -80.84%   x 5.22

u128_rand::insert_all           9,417,089             7,963,523           -1,453,566  -15.44%   x 1.18
u64_rand::insert_all            7,667,847             6,310,732           -1,357,115  -17.70%   x 1.22
usize_rand::insert_all          7,681,926             6,455,917           -1,226,009  -15.96%   x 1.19
u32_rand::insert_all            7,470,839             6,315,266           -1,155,573  -15.47%   x 1.18
u16_rand::insert_all            386,024               315,440                -70,584  -18.28%   x 1.22
u8_rand::insert_all             15,748                6,076                   -9,672  -61.42%   x 2.59
u128_rand::find_all             4,125,308             2,065,209           -2,060,099  -49.94%   x 2.00
u64_rand::find_all              3,582,193             1,940,426           -1,641,767  -45.83%   x 1.85
usize_rand::find_all            3,560,783             1,864,784           -1,695,999  -47.63%   x 1.91
u32_rand::find_all              2,969,717             1,753,468           -1,216,249  -40.96%   x 1.69
u16_rand::find_all              97,320                30,497                 -66,823  -68.66%   x 3.19
u8_rand::find_all               3,444                 882                     -2,562  -74.39%   x 3.90

u64_times_table_32::insert_all  7,677,063             4,862,680           -2,814,383  -36.66%   x 1.58
u32_times_table_32::insert_all  7,484,472             4,366,920           -3,117,552  -41.65%   x 1.71
u16_times_table_32::insert_all  140,074               93,945                 -46,129  -32.93%   x 1.49
u64_times_table_32::find_all    3,573,829             1,936,193           -1,637,636  -45.82%   x 1.85
u32_times_table_32::find_all    2,965,513             1,625,043           -1,340,470  -45.20%   x 1.82
u16_times_table_32::find_all    29,571                9,458                  -20,113  -68.02%   x 3.13
```

## `fnv` vs `int_hash`
_ℕ **1.31-5.84x** faster, random **1.00-1.84x** faster, 32× table **0.59-1.14x** faster_
```
name                            fnv_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        7,096,628          3,726,791           -3,369,837  -47.49%   x 1.90
u64_natural::insert_all         4,225,578          2,135,107           -2,090,471  -49.47%   x 1.98
usize_natural::insert_all       4,209,341          2,131,158           -2,078,183  -49.37%   x 1.98
u32_natural::insert_all         4,275,368          2,520,760           -1,754,608  -41.04%   x 1.70
u16_natural::insert_all         3,631,043          1,913,495           -1,717,548  -47.30%   x 1.90
u128_natural::find_all          2,939,183          503,631             -2,435,552  -82.86%   x 5.84
u64_natural::find_all           1,658,005          396,861             -1,261,144  -76.06%   x 4.18
usize_natural::find_all         1,618,836          398,027             -1,220,809  -75.41%   x 4.07
u32_natural::find_all           1,607,451          342,627             -1,264,824  -78.69%   x 4.69
u16_natural::find_all           361,278            276,190                -85,088  -23.55%   x 1.31

u128_rand::insert_all           9,458,147          7,963,523           -1,494,624  -15.80%   x 1.19
u64_rand::insert_all            6,987,447          6,310,732             -676,715   -9.68%   x 1.11
usize_rand::insert_all          6,856,428          6,455,917             -400,511   -5.84%   x 1.06
u32_rand::insert_all            6,293,380          6,315,266               21,886    0.35%   x 1.00
u16_rand::insert_all            323,246            315,440                 -7,806   -2.41%   x 1.02
u8_rand::insert_all             6,220              6,076                     -144   -2.32%   x 1.02
u128_rand::find_all             3,803,030          2,065,209           -1,737,821  -45.70%   x 1.84
u64_rand::find_all              2,504,745          1,940,426             -564,319  -22.53%   x 1.29
usize_rand::find_all            2,424,788          1,864,784             -560,004  -23.09%   x 1.30
u32_rand::find_all              1,923,486          1,753,468             -170,018   -8.84%   x 1.10
u16_rand::find_all              35,830             30,497                  -5,333  -14.88%   x 1.17
u8_rand::find_all               1,023              882                       -141  -13.78%   x 1.16

u64_times_table_32::insert_all  4,789,588          4,862,680               73,092    1.53%   x 0.98
u32_times_table_32::insert_all  4,854,573          4,366,920             -487,653  -10.05%   x 1.11
u16_times_table_32::insert_all  55,893             93,945                  38,052   68.08%   x 0.59
u64_times_table_32::find_all    1,656,430          1,936,193              279,763   16.89%   x 0.86
u32_times_table_32::find_all    1,520,743          1,625,043              104,300    6.86%   x 0.94
u16_times_table_32::find_all    10,781             9,458                   -1,323  -12.27%   x 1.14
```

## `rustc_hash` vs `int_hash`
_ℕ **1.14-2.48x** faster, random **0.95-1.07x** faster, 32× table **0.97-1.13x** faster_
```
name                            rustc_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        5,295,947            3,726,791           -1,569,156  -29.63%   x 1.42
u64_natural::insert_all         2,939,389            2,135,107             -804,282  -27.36%   x 1.38
usize_natural::insert_all       2,943,134            2,131,158             -811,976  -27.59%   x 1.38
u32_natural::insert_all         3,250,113            2,520,760             -729,353  -22.44%   x 1.29
u16_natural::insert_all         2,185,980            1,913,495             -272,485  -12.47%   x 1.14
u128_natural::find_all          1,250,862            503,631               -747,231  -59.74%   x 2.48
u64_natural::find_all           512,569              396,861               -115,708  -22.57%   x 1.29
usize_natural::find_all         531,162              398,027               -133,135  -25.06%   x 1.33
u32_natural::find_all           455,601              342,627               -112,974  -24.80%   x 1.33
u16_natural::find_all           339,095              276,190                -62,905  -18.55%   x 1.23

u128_rand::insert_all           7,929,368            7,963,523               34,155    0.43%   x 1.00
u64_rand::insert_all            6,238,452            6,310,732               72,280    1.16%   x 0.99
usize_rand::insert_all          6,216,101            6,455,917              239,816    3.86%   x 0.96
u32_rand::insert_all            6,099,500            6,315,266              215,766    3.54%   x 0.97
u16_rand::insert_all            310,424              315,440                  5,016    1.62%   x 0.98
u8_rand::insert_all             6,275                6,076                     -199   -3.17%   x 1.03
u128_rand::find_all             2,165,399            2,065,209             -100,190   -4.63%   x 1.05
u64_rand::find_all              1,897,912            1,940,426               42,514    2.24%   x 0.98
usize_rand::find_all            1,776,407            1,864,784               88,377    4.98%   x 0.95
u32_rand::find_all              1,737,856            1,753,468               15,612    0.90%   x 0.99
u16_rand::find_all              32,639               30,497                  -2,142   -6.56%   x 1.07
u8_rand::find_all               895                  882                        -13   -1.45%   x 1.01

u64_times_table_32::insert_all  4,706,111            4,862,680              156,569    3.33%   x 0.97
u32_times_table_32::insert_all  4,665,906            4,366,920             -298,986   -6.41%   x 1.07
u16_times_table_32::insert_all  94,243               93,945                    -298   -0.32%   x 1.00
u64_times_table_32::find_all    2,082,625            1,936,193             -146,432   -7.03%   x 1.08
u32_times_table_32::find_all    1,828,796            1,625,043             -203,753  -11.14%   x 1.13
u16_times_table_32::find_all    10,539               9,458                   -1,081  -10.26%   x 1.11
```

## [Thomas Wang](https://gist.github.com/badboy/6267743) vs `int_hash`
_ℕ **2.21-5.82x** faster, random **1.04-1.46x** faster, 32× table **1.04-1.48x** faster_
```
name                            wang_mix ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        8,223,081         3,726,791           -4,496,290  -54.68%   x 2.21
u64_natural::insert_all         6,627,508         2,135,107           -4,492,401  -67.78%   x 3.10
usize_natural::insert_all       6,624,942         2,131,158           -4,493,784  -67.83%   x 3.11
u32_natural::insert_all         6,530,064         2,520,760           -4,009,304  -61.40%   x 2.59
u16_natural::insert_all         4,950,439         1,913,495           -3,036,944  -61.35%   x 2.59
u128_natural::find_all          2,352,708         503,631             -1,849,077  -78.59%   x 4.67
u64_natural::find_all           2,025,607         396,861             -1,628,746  -80.41%   x 5.10
usize_natural::find_all         2,071,958         398,027             -1,673,931  -80.79%   x 5.21
u32_natural::find_all           1,992,639         342,627             -1,650,012  -82.81%   x 5.82
u16_natural::find_all           865,885           276,190               -589,695  -68.10%   x 3.14

u128_rand::insert_all           8,281,195         7,963,523             -317,672   -3.84%   x 1.04
u64_rand::insert_all            6,671,253         6,310,732             -360,521   -5.40%   x 1.06
usize_rand::insert_all          6,698,918         6,455,917             -243,001   -3.63%   x 1.04
u32_rand::insert_all            6,598,060         6,315,266             -282,794   -4.29%   x 1.04
u16_rand::insert_all            330,916           315,440                -15,476   -4.68%   x 1.05
u8_rand::insert_all             7,492             6,076                   -1,416  -18.90%   x 1.23
u128_rand::find_all             2,338,992         2,065,209             -273,783  -11.71%   x 1.13
u64_rand::find_all              2,024,201         1,940,426              -83,775   -4.14%   x 1.04
usize_rand::find_all            2,085,011         1,864,784             -220,227  -10.56%   x 1.12
u32_rand::find_all              2,002,983         1,753,468             -249,515  -12.46%   x 1.14
u16_rand::find_all              44,479            30,497                 -13,982  -31.44%   x 1.46
u8_rand::find_all               1,292             882                       -410  -31.73%   x 1.46

u64_times_table_32::insert_all  6,563,801         4,862,680           -1,701,121  -25.92%   x 1.35
u32_times_table_32::insert_all  6,484,104         4,366,920           -2,117,184  -32.65%   x 1.48
u16_times_table_32::insert_all  109,195           93,945                 -15,250  -13.97%   x 1.16
u64_times_table_32::find_all    2,020,953         1,936,193              -84,760   -4.19%   x 1.04
u32_times_table_32::find_all    1,990,492         1,625,043             -365,449  -18.36%   x 1.22
u16_times_table_32::find_all    12,919            9,458                   -3,461  -26.79%   x 1.37

```

## `seahash` vs `int_hash`
_ℕ **2.71-10.67x** faster, random **1.11-2.61x** faster, 32× table **1.29-2.14x** faster_
```
name                            seahash_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        11,359,016             3,726,791           -7,632,225  -67.19%   x 3.05
u64_natural::insert_all         7,034,806              2,135,107           -4,899,699  -69.65%   x 3.29
usize_natural::insert_all       7,261,798              2,131,158           -5,130,640  -70.65%   x 3.41
u32_natural::insert_all         6,968,926              2,520,760           -4,448,166  -63.83%   x 2.76
u16_natural::insert_all         5,193,001              1,913,495           -3,279,506  -63.15%   x 2.71
u128_natural::find_all          5,371,472              503,631             -4,867,841  -90.62%  x 10.67
u64_natural::find_all           2,831,999              396,861             -2,435,138  -85.99%   x 7.14
usize_natural::find_all         2,744,145              398,027             -2,346,118  -85.50%   x 6.89
u32_natural::find_all           2,741,229              342,627             -2,398,602  -87.50%   x 8.00
u16_natural::find_all           1,179,988              276,190               -903,798  -76.59%   x 4.27

u128_rand::insert_all           11,260,861             7,963,523           -3,297,338  -29.28%   x 1.41
u64_rand::insert_all            7,011,184              6,310,732             -700,452   -9.99%   x 1.11
usize_rand::insert_all          7,302,565              6,455,917             -846,648  -11.59%   x 1.13
u32_rand::insert_all            7,049,107              6,315,266             -733,841  -10.41%   x 1.12
u16_rand::insert_all            355,698                315,440                -40,258  -11.32%   x 1.13
u8_rand::insert_all             7,760                  6,076                   -1,684  -21.70%   x 1.28
u128_rand::find_all             5,393,819              2,065,209           -3,328,610  -61.71%   x 2.61
u64_rand::find_all              2,834,543              1,940,426             -894,117  -31.54%   x 1.46
usize_rand::find_all            2,726,268              1,864,784             -861,484  -31.60%   x 1.46
u32_rand::find_all              2,760,185              1,753,468           -1,006,717  -36.47%   x 1.57
u16_rand::find_all              67,794                 30,497                 -37,297  -55.02%   x 2.22
u8_rand::find_all               2,079                  882                     -1,197  -57.58%   x 2.36

u64_times_table_32::insert_all  7,124,570              4,862,680           -2,261,890  -31.75%   x 1.47
u32_times_table_32::insert_all  6,849,191              4,366,920           -2,482,271  -36.24%   x 1.57
u16_times_table_32::insert_all  121,184                93,945                 -27,239  -22.48%   x 1.29
u64_times_table_32::find_all    2,849,781              1,936,193             -913,588  -32.06%   x 1.47
u32_times_table_32::find_all    2,753,743              1,625,043           -1,128,700  -40.99%   x 1.69
u16_times_table_32::find_all    20,282                 9,458                  -10,824  -53.37%   x 2.14
```

## `twox_hash` vs `int_hash`
_ℕ **2.93-9.85x** faster, random **1.20-4.17x** faster, 32× table **1.55-3.64x** faster_
```
name                            twox_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        10,917,497          3,726,791           -7,190,706  -65.86%   x 2.93
u64_natural::insert_all         7,776,458           2,135,107           -5,641,351  -72.54%   x 3.64
usize_natural::insert_all       7,816,477           2,131,158           -5,685,319  -72.74%   x 3.67
u32_natural::insert_all         7,446,445           2,520,760           -4,925,685  -66.15%   x 2.95
u16_natural::insert_all         5,656,950           1,913,495           -3,743,455  -66.17%   x 2.96
u128_natural::find_all          4,778,330           503,631             -4,274,699  -89.46%   x 9.49
u64_natural::find_all           3,434,316           396,861             -3,037,455  -88.44%   x 8.65
usize_natural::find_all         3,342,633           398,027             -2,944,606  -88.09%   x 8.40
u32_natural::find_all           3,374,483           342,627             -3,031,856  -89.85%   x 9.85
u16_natural::find_all           1,682,861           276,190             -1,406,671  -83.59%   x 6.09

u128_rand::insert_all           10,977,046          7,963,523           -3,013,523  -27.45%   x 1.38
u64_rand::insert_all            7,696,870           6,310,732           -1,386,138  -18.01%   x 1.22
usize_rand::insert_all          7,757,104           6,455,917           -1,301,187  -16.77%   x 1.20
u32_rand::insert_all            7,547,967           6,315,266           -1,232,701  -16.33%   x 1.20
u16_rand::insert_all            397,421             315,440                -81,981  -20.63%   x 1.26
u8_rand::insert_all             10,527              6,076                   -4,451  -42.28%   x 1.73
u128_rand::find_all             4,729,428           2,065,209           -2,664,219  -56.33%   x 2.29
u64_rand::find_all              3,413,292           1,940,426           -1,472,866  -43.15%   x 1.76
usize_rand::find_all            3,336,895           1,864,784           -1,472,111  -44.12%   x 1.79
u32_rand::find_all              3,367,959           1,753,468           -1,614,491  -47.94%   x 1.92
u16_rand::find_all              112,784             30,497                 -82,287  -72.96%   x 3.70
u8_rand::find_all               3,676               882                     -2,794  -76.01%   x 4.17

u64_times_table_32::insert_all  7,571,333           4,862,680           -2,708,653  -35.78%   x 1.56
u32_times_table_32::insert_all  7,698,344           4,366,920           -3,331,424  -43.27%   x 1.76
u16_times_table_32::insert_all  145,995             93,945                 -52,050  -35.65%   x 1.55
u64_times_table_32::find_all    3,412,272           1,936,193           -1,476,079  -43.26%   x 1.76
u32_times_table_32::find_all    3,379,171           1,625,043           -1,754,128  -51.91%   x 2.08
u16_times_table_32::find_all    34,468              9,458                  -25,010  -72.56%   x 3.64
```
