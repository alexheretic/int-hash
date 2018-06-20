# HashMap Benchmark Report
Results from a full 100,000 sample of various test integer key value collections. See `./benches`.

For each sample of integer values there are 2 tests:
- ***insert_all***: All keys from the sample are inserted into a `HashMap` of default size.
- ***find_all***: All key-values from the sample are looked up from a pre-populated `HashMap`.

## default vs `int_hash`
_ℕ **2.57-8.38x** faster, random **1.17-3.61x** faster, 32× table **1.52-3.09x** faster_
```
name                            default_hash ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        9,419,267             3,666,087           -5,753,180  -61.08%   x 2.57
u64_natural::insert_all         7,655,443             2,167,465           -5,487,978  -71.69%   x 3.53
usize_natural::insert_all       7,748,178             2,126,062           -5,622,116  -72.56%   x 3.64
u32_natural::insert_all         7,496,285             2,521,114           -4,975,171  -66.37%   x 2.97
u16_natural::insert_all         5,558,629             1,905,489           -3,653,140  -65.72%   x 2.92
u128_natural::find_all          4,157,188             559,842             -3,597,346  -86.53%   x 7.43
u64_natural::find_all           3,519,071             422,216             -3,096,855  -88.00%   x 8.33
usize_natural::find_all         3,542,723             422,871             -3,119,852  -88.06%   x 8.38
u32_natural::find_all           2,967,511             368,664             -2,598,847  -87.58%   x 8.05
u16_natural::find_all           1,452,387             258,888             -1,193,499  -82.17%   x 5.61

u128_rand::insert_all           9,427,925             7,920,764           -1,507,161  -15.99%   x 1.19
u64_rand::insert_all            7,656,231             6,341,322           -1,314,909  -17.17%   x 1.21
usize_rand::insert_all          7,731,195             6,490,067           -1,241,128  -16.05%   x 1.19
u32_rand::insert_all            7,476,879             6,375,432           -1,101,447  -14.73%   x 1.17
u16_rand::insert_all            382,677               324,580                -58,097  -15.18%   x 1.18
u8_rand::insert_all             15,766                6,198                   -9,568  -60.69%   x 2.54
u128_rand::find_all             4,184,843             2,076,310           -2,108,533  -50.38%   x 2.02
u64_rand::find_all              3,527,884             1,949,207           -1,578,677  -44.75%   x 1.81
usize_rand::find_all            3,525,940             1,914,162           -1,611,778  -45.71%   x 1.84
u32_rand::find_all              2,971,218             1,831,239           -1,139,979  -38.37%   x 1.62
u16_rand::find_all              97,125                29,991                 -67,134  -69.12%   x 3.24
u8_rand::find_all               3,445                 954                     -2,491  -72.31%   x 3.61

u64_times_table_32::insert_all  7,664,132             4,936,084           -2,728,048  -35.60%   x 1.55
u32_times_table_32::insert_all  7,495,207             4,598,230           -2,896,977  -38.65%   x 1.63
u16_times_table_32::insert_all  138,752               91,407                 -47,345  -34.12%   x 1.52
u64_times_table_32::find_all    3,514,995             1,905,971           -1,609,024  -45.78%   x 1.84
u32_times_table_32::find_all    2,973,862             1,697,730           -1,276,132  -42.91%   x 1.75
u16_times_table_32::find_all    29,256                9,472                  -19,784  -67.62%   x 3.09
```


## `fnv` vs `int_hash`
_ℕ **1.36-5.27x** faster, random **0.99-1.83x** faster, 32× table **0.60-1.12x** faster_
```
name                            fnv_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        7,100,767          3,666,087           -3,434,680  -48.37%   x 1.94
u64_natural::insert_all         4,236,083          2,167,465           -2,068,618  -48.83%   x 1.95
usize_natural::insert_all       4,205,092          2,126,062           -2,079,030  -49.44%   x 1.98
u32_natural::insert_all         4,288,739          2,521,114           -1,767,625  -41.22%   x 1.70
u16_natural::insert_all         3,646,492          1,905,489           -1,741,003  -47.74%   x 1.91
u128_natural::find_all          2,951,928          559,842             -2,392,086  -81.03%   x 5.27
u64_natural::find_all           1,610,153          422,216             -1,187,937  -73.78%   x 3.81
usize_natural::find_all         1,709,800          422,871             -1,286,929  -75.27%   x 4.04
u32_natural::find_all           1,712,490          368,664             -1,343,826  -78.47%   x 4.65
u16_natural::find_all           353,262            258,888                -94,374  -26.72%   x 1.36

u128_rand::insert_all           9,441,726          7,920,764           -1,520,962  -16.11%   x 1.19
u64_rand::insert_all            7,049,711          6,341,322             -708,389  -10.05%   x 1.11
usize_rand::insert_all          6,780,380          6,490,067             -290,313   -4.28%   x 1.04
u32_rand::insert_all            6,344,375          6,375,432               31,057    0.49%   x 1.00
u16_rand::insert_all            322,542            324,580                  2,038    0.63%   x 0.99
u8_rand::insert_all             6,153              6,198                       45    0.73%   x 0.99
u128_rand::find_all             3,791,003          2,076,310           -1,714,693  -45.23%   x 1.83
u64_rand::find_all              2,426,270          1,949,207             -477,063  -19.66%   x 1.24
usize_rand::find_all            2,395,299          1,914,162             -481,137  -20.09%   x 1.25
u32_rand::find_all              1,989,625          1,831,239             -158,386   -7.96%   x 1.09
u16_rand::find_all              34,988             29,991                  -4,997  -14.28%   x 1.17
u8_rand::find_all               1,153              954                       -199  -17.26%   x 1.21

u64_times_table_32::insert_all  4,801,353          4,936,084              134,731    2.81%   x 0.97
u32_times_table_32::insert_all  4,867,927          4,598,230             -269,697   -5.54%   x 1.06
u16_times_table_32::insert_all  55,035             91,407                  36,372   66.09%   x 0.60
u64_times_table_32::find_all    1,630,910          1,905,971              275,061   16.87%   x 0.86
u32_times_table_32::find_all    1,587,556          1,697,730              110,174    6.94%   x 0.94
u16_times_table_32::find_all    10,632             9,472                   -1,160  -10.91%   x 1.12
```

## `rustc_hash` vs `int_hash`
_ℕ **1.16-2.34x** faster, random **0.94-1.10x** faster, 32× table **0.98-1.17x** faster_
```
name                            rustc_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        5,264,292            3,666,087           -1,598,205  -30.36%   x 1.44
u64_natural::insert_all         2,961,574            2,167,465             -794,109  -26.81%   x 1.37
usize_natural::insert_all       2,992,092            2,126,062             -866,030  -28.94%   x 1.41
u32_natural::insert_all         3,240,480            2,521,114             -719,366  -22.20%   x 1.29
u16_natural::insert_all         2,207,857            1,905,489             -302,368  -13.70%   x 1.16
u128_natural::find_all          1,312,536            559,842               -752,694  -57.35%   x 2.34
u64_natural::find_all           550,087              422,216               -127,871  -23.25%   x 1.30
usize_natural::find_all         530,627              422,871               -107,756  -20.31%   x 1.25
u32_natural::find_all           459,968              368,664                -91,304  -19.85%   x 1.25
u16_natural::find_all           330,374              258,888                -71,486  -21.64%   x 1.28

u128_rand::insert_all           7,926,443            7,920,764               -5,679   -0.07%   x 1.00
u64_rand::insert_all            6,268,822            6,341,322               72,500    1.16%   x 0.99
usize_rand::insert_all          6,130,916            6,490,067              359,151    5.86%   x 0.94
u32_rand::insert_all            6,114,297            6,375,432              261,135    4.27%   x 0.96
u16_rand::insert_all            303,774              324,580                 20,806    6.85%   x 0.94
u8_rand::insert_all             6,241                6,198                      -43   -0.69%   x 1.01
u128_rand::find_all             2,277,356            2,076,310             -201,046   -8.83%   x 1.10
u64_rand::find_all              1,923,126            1,949,207               26,081    1.36%   x 0.99
usize_rand::find_all            1,860,523            1,914,162               53,639    2.88%   x 0.97
u32_rand::find_all              1,801,016            1,831,239               30,223    1.68%   x 0.98
u16_rand::find_all              31,855               29,991                  -1,864   -5.85%   x 1.06
u8_rand::find_all               1,021                954                        -67   -6.56%   x 1.07

u64_times_table_32::insert_all  4,965,685            4,936,084              -29,601   -0.60%   x 1.01
u32_times_table_32::insert_all  4,891,667            4,598,230             -293,437   -6.00%   x 1.06
u16_times_table_32::insert_all  89,659               91,407                   1,748    1.95%   x 0.98
u64_times_table_32::find_all    2,203,289            1,905,971             -297,318  -13.49%   x 1.16
u32_times_table_32::find_all    1,992,569            1,697,730             -294,839  -14.80%   x 1.17
u16_times_table_32::find_all    10,302               9,472                     -830   -8.06%   x 1.09
```

## Thomas Wang (see https://gist.github.com/badboy/6267743) vs `int_hash`
_ℕ **2.23-5.50x** faster, random **1.00-1.43x** faster, 32× table **1.07-1.42x** faster_
```
name                            wang_mix ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        8,193,466         3,666,087           -4,527,379  -55.26%   x 2.23
u64_natural::insert_all         6,619,744         2,167,465           -4,452,279  -67.26%   x 3.05
usize_natural::insert_all       6,628,400         2,126,062           -4,502,338  -67.92%   x 3.12
u32_natural::insert_all         6,562,860         2,521,114           -4,041,746  -61.59%   x 2.60
u16_natural::insert_all         5,016,280         1,905,489           -3,110,791  -62.01%   x 2.63
u128_natural::find_all          2,338,982         559,842             -1,779,140  -76.06%   x 4.18
u64_natural::find_all           2,058,018         422,216             -1,635,802  -79.48%   x 4.87
usize_natural::find_all         2,135,752         422,871             -1,712,881  -80.20%   x 5.05
u32_natural::find_all           2,027,216         368,664             -1,658,552  -81.81%   x 5.50
u16_natural::find_all           842,024           258,888               -583,136  -69.25%   x 3.25

u128_rand::insert_all           8,264,773         7,920,764             -344,009   -4.16%   x 1.04
u64_rand::insert_all            6,672,117         6,341,322             -330,795   -4.96%   x 1.05
usize_rand::insert_all          6,696,120         6,490,067             -206,053   -3.08%   x 1.03
u32_rand::insert_all            6,625,881         6,375,432             -250,449   -3.78%   x 1.04
u16_rand::insert_all            324,581           324,580                     -1   -0.00%   x 1.00
u8_rand::insert_all             7,425             6,198                   -1,227  -16.53%   x 1.20
u128_rand::find_all             2,328,925         2,076,310             -252,615  -10.85%   x 1.12
u64_rand::find_all              2,052,410         1,949,207             -103,203   -5.03%   x 1.05
usize_rand::find_all            2,145,243         1,914,162             -231,081  -10.77%   x 1.12
u32_rand::find_all              2,037,734         1,831,239             -206,495  -10.13%   x 1.11
u16_rand::find_all              43,012            29,991                 -13,021  -30.27%   x 1.43
u8_rand::find_all               1,292             954                       -338  -26.16%   x 1.35

u64_times_table_32::insert_all  6,573,920         4,936,084           -1,637,836  -24.91%   x 1.33
u32_times_table_32::insert_all  6,514,195         4,598,230           -1,915,965  -29.41%   x 1.42
u16_times_table_32::insert_all  108,345           91,407                 -16,938  -15.63%   x 1.19
u64_times_table_32::find_all    2,047,634         1,905,971             -141,663   -6.92%   x 1.07
u32_times_table_32::find_all    2,023,473         1,697,730             -325,743  -16.10%   x 1.19
u16_times_table_32::find_all    12,646            9,472                   -3,174  -25.10%   x 1.34
```

## `seahash` vs `int_hash`
_ℕ **2.72-9.70x** faster, random **1.07-2.83x** faster, 32× table **1.31-2.10x** faster_
```
name                            seahash_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        11,137,128             3,666,087           -7,471,041  -67.08%   x 3.04
u64_natural::insert_all         7,116,342              2,167,465           -4,948,877  -69.54%   x 3.28
usize_natural::insert_all       7,191,083              2,126,062           -5,065,021  -70.43%   x 3.38
u32_natural::insert_all         7,008,973              2,521,114           -4,487,859  -64.03%   x 2.78
u16_natural::insert_all         5,176,372              1,905,489           -3,270,883  -63.19%   x 2.72
u128_natural::find_all          5,428,210              559,842             -4,868,368  -89.69%   x 9.70
u64_natural::find_all           2,714,967              422,216             -2,292,751  -84.45%   x 6.43
usize_natural::find_all         2,758,889              422,871             -2,336,018  -84.67%   x 6.52
u32_natural::find_all           2,689,484              368,664             -2,320,820  -86.29%   x 7.30
u16_natural::find_all           1,177,277              258,888               -918,389  -78.01%   x 4.55

u128_rand::insert_all           10,999,250             7,920,764           -3,078,486  -27.99%   x 1.39
u64_rand::insert_all            7,102,610              6,341,322             -761,288  -10.72%   x 1.12
usize_rand::insert_all          7,235,882              6,490,067             -745,815  -10.31%   x 1.11
u32_rand::insert_all            7,086,609              6,375,432             -711,177  -10.04%   x 1.11
u16_rand::insert_all            348,792                324,580                -24,212   -6.94%   x 1.07
u8_rand::insert_all             7,751                  6,198                   -1,553  -20.04%   x 1.25
u128_rand::find_all             5,866,014              2,076,310           -3,789,704  -64.60%   x 2.83
u64_rand::find_all              2,720,173              1,949,207             -770,966  -28.34%   x 1.40
usize_rand::find_all            2,737,408              1,914,162             -823,246  -30.07%   x 1.43
u32_rand::find_all              2,698,920              1,831,239             -867,681  -32.15%   x 1.47
u16_rand::find_all              66,787                 29,991                 -36,796  -55.09%   x 2.23
u8_rand::find_all               2,078                  954                     -1,124  -54.09%   x 2.18

u64_times_table_32::insert_all  7,207,157              4,936,084           -2,271,073  -31.51%   x 1.46
u32_times_table_32::insert_all  6,879,415              4,598,230           -2,281,185  -33.16%   x 1.50
u16_times_table_32::insert_all  119,530                91,407                 -28,123  -23.53%   x 1.31
u64_times_table_32::find_all    2,739,972              1,905,971             -834,001  -30.44%   x 1.44
u32_times_table_32::find_all    2,690,811              1,697,730             -993,081  -36.91%   x 1.58
u16_times_table_32::find_all    19,926                 9,472                  -10,454  -52.46%   x 2.10
```

## `twox_hash` vs `int_hash`
_ℕ **2.81-9.19x** faster, random **1.17-3.91x** faster, 32× table **1.52-3.59x** faster_
```
name                            twox_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u128_natural::insert_all        10,292,140          3,666,087           -6,626,053  -64.38%   x 2.81
u64_natural::insert_all         7,732,500           2,167,465           -5,565,035  -71.97%   x 3.57
usize_natural::insert_all       7,735,446           2,126,062           -5,609,384  -72.52%   x 3.64
u32_natural::insert_all         7,466,322           2,521,114           -4,945,208  -66.23%   x 2.96
u16_natural::insert_all         5,685,675           1,905,489           -3,780,186  -66.49%   x 2.98
u128_natural::find_all          5,125,067           559,842             -4,565,225  -89.08%   x 9.15
u64_natural::find_all           3,297,794           422,216             -2,875,578  -87.20%   x 7.81
usize_natural::find_all         3,292,033           422,871             -2,869,162  -87.15%   x 7.78
u32_natural::find_all           3,388,255           368,664             -3,019,591  -89.12%   x 9.19
u16_natural::find_all           1,675,797           258,888             -1,416,909  -84.55%   x 6.47

u128_rand::insert_all           10,367,506          7,920,764           -2,446,742  -23.60%   x 1.31
u64_rand::insert_all            7,659,986           6,341,322           -1,318,664  -17.21%   x 1.21
usize_rand::insert_all          7,677,055           6,490,067           -1,186,988  -15.46%   x 1.18
u32_rand::insert_all            7,467,646           6,375,432           -1,092,214  -14.63%   x 1.17
u16_rand::insert_all            396,252             324,580                -71,672  -18.09%   x 1.22
u8_rand::insert_all             10,395              6,198                   -4,197  -40.38%   x 1.68
u128_rand::find_all             5,113,492           2,076,310           -3,037,182  -59.40%   x 2.46
u64_rand::find_all              3,281,844           1,949,207           -1,332,637  -40.61%   x 1.68
usize_rand::find_all            3,288,494           1,914,162           -1,374,332  -41.79%   x 1.72
u32_rand::find_all              3,360,082           1,831,239           -1,528,843  -45.50%   x 1.83
u16_rand::find_all              111,750             29,991                 -81,759  -73.16%   x 3.73
u8_rand::find_all               3,732               954                     -2,778  -74.44%   x 3.91

u64_times_table_32::insert_all  7,526,265           4,936,084           -2,590,181  -34.42%   x 1.52
u32_times_table_32::insert_all  7,613,790           4,598,230           -3,015,560  -39.61%   x 1.66
u16_times_table_32::insert_all  145,637             91,407                 -54,230  -37.24%   x 1.59
u64_times_table_32::find_all    3,285,227           1,905,971           -1,379,256  -41.98%   x 1.72
u32_times_table_32::find_all    3,373,475           1,697,730           -1,675,745  -49.67%   x 1.99
u16_times_table_32::find_all    33,995              9,472                  -24,523  -72.14%   x 3.59
```
