# HashMap Benchmark Report (2018-12-08)
Results from a full 100,000 sample of various test integer key value collection, see `./benches`. The benchmark system used an Intel i5-4670K.

For each sample of integer values there are 2 tests:
- ***insert_all***: All keys from the sample are inserted into a `HashMap` of default size.
- ***find_all***: All key-values from the sample are looked up from a pre-populated `HashMap`.

## default vs `int_hash`
_random **1.23-9.61x** faster, ℕ **3.82-21.6x** faster, 32× table **1.59-4.94x** faster_
```
name                            default_hash ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               3,019                 314                     -2,705  -89.60%   x 9.61
u16_rand::find_all              88,586                13,869                 -74,717  -84.34%   x 6.39
u32_rand::find_all              2,813,612             1,257,109           -1,556,503  -55.32%   x 2.24
usize_rand::find_all            3,074,971             1,304,515           -1,770,456  -57.58%   x 2.36
u64_rand::find_all              3,089,568             1,349,341           -1,740,227  -56.33%   x 2.29

u8_rand::insert_all             13,762                5,006                   -8,756  -63.62%   x 2.75
u16_rand::insert_all            339,850               271,069                -68,781  -20.24%   x 1.25
u32_rand::insert_all            6,044,002             4,887,022           -1,156,980  -19.14%   x 1.24
usize_rand::insert_all          5,995,602             4,864,446           -1,131,156  -18.87%   x 1.23
u64_rand::insert_all            6,068,173             4,719,502           -1,348,671  -22.23%   x 1.29

u16_natural::find_all           1,339,632             82,438              -1,257,194  -93.85%  x 16.25
u32_natural::find_all           2,769,617             128,202             -2,641,415  -95.37%  x 21.60
usize_natural::find_all         3,072,822             157,579             -2,915,243  -94.87%  x 19.50
u64_natural::find_all           3,119,084             157,473             -2,961,611  -94.95%  x 19.81

u16_natural::insert_all         4,509,217             1,180,757           -3,328,460  -73.81%   x 3.82
u32_natural::insert_all         6,763,769             1,494,354           -5,269,415  -77.91%   x 4.53
usize_natural::insert_all       6,003,919             947,293             -5,056,626  -84.22%   x 6.34
u64_natural::insert_all         6,061,635             930,874             -5,130,761  -84.64%   x 6.51

u16_times_table_32::find_all    26,814                5,430                  -21,384  -79.75%   x 4.94
u32_times_table_32::find_all    2,798,724             1,378,418           -1,420,306  -50.75%   x 2.03
u64_times_table_32::find_all    3,064,368             1,645,669           -1,418,699  -46.30%   x 1.86

u16_times_table_32::insert_all  122,569               77,025                 -45,544  -37.16%   x 1.59
u32_times_table_32::insert_all  6,034,543             3,523,361           -2,511,182  -41.61%   x 1.71
u64_times_table_32::insert_all  6,064,009             3,140,083           -2,923,926  -48.22%   x 1.93
```

## `fnv` vs `int_hash`
_random **0.99-1.58x** faster, ℕ **2.11-11.08x** faster, 32× table **0.57-1.09x** faster_
```
name                            fnv_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               465                314                       -151  -32.47%   x 1.48
u16_rand::find_all              18,117             13,869                  -4,248  -23.45%   x 1.31
u32_rand::find_all              1,626,370          1,257,109             -369,261  -22.70%   x 1.29
usize_rand::find_all            2,055,344          1,304,515             -750,829  -36.53%   x 1.58
u64_rand::find_all              2,132,340          1,349,341             -782,999  -36.72%   x 1.58

u8_rand::insert_all             4,974              5,006                       32    0.64%   x 0.99
u16_rand::insert_all            282,793            271,069                -11,724   -4.15%   x 1.04
u32_rand::insert_all            5,042,772          4,887,022             -155,750   -3.09%   x 1.03
usize_rand::insert_all          5,234,611          4,864,446             -370,165   -7.07%   x 1.08
u64_rand::insert_all            5,397,107          4,719,502             -677,605  -12.55%   x 1.14

u16_natural::find_all           182,163            82,438                 -99,725  -54.74%   x 2.21
u32_natural::find_all           1,419,874          128,202             -1,291,672  -90.97%  x 11.08
usize_natural::find_all         1,406,172          157,579             -1,248,593  -88.79%   x 8.92
u64_natural::find_all           1,417,192          157,473             -1,259,719  -88.89%   x 9.00

u16_natural::insert_all         2,744,231          1,180,757           -1,563,474  -56.97%   x 2.32
u32_natural::insert_all         3,153,726          1,494,354           -1,659,372  -52.62%   x 2.11
usize_natural::insert_all       2,913,671          947,293             -1,966,378  -67.49%   x 3.08
u64_natural::insert_all         2,910,616          930,874             -1,979,742  -68.02%   x 3.13

u16_times_table_32::find_all    5,362              5,430                       68    1.27%   x 0.99
u32_times_table_32::find_all    1,261,395          1,378,418              117,023    9.28%   x 0.92
u64_times_table_32::find_all    1,367,299          1,645,669              278,370   20.36%   x 0.83

u16_times_table_32::insert_all  43,864             77,025                  33,161   75.60%   x 0.57
u32_times_table_32::insert_all  3,654,072          3,523,361             -130,711   -3.58%   x 1.04
u64_times_table_32::insert_all  3,434,717          3,140,083             -294,634   -8.58%   x 1.09
```

## `rustc_hash` vs `int_hash`
_random **0.95-1.29x** faster, ℕ **1.26-2.15x** faster, 32× table **0.94-1.28x** faster_
```
name                            rustc_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               405                  314                        -91  -22.47%   x 1.29
u16_rand::find_all              15,690               13,869                  -1,821  -11.61%   x 1.13
u32_rand::find_all              1,301,716            1,257,109              -44,607   -3.43%   x 1.04
usize_rand::find_all            1,261,491            1,304,515               43,024    3.41%   x 0.97
u64_rand::find_all              1,313,015            1,349,341               36,326    2.77%   x 0.97

u8_rand::insert_all             5,117                5,006                     -111   -2.17%   x 1.02
u16_rand::insert_all            263,578              271,069                  7,491    2.84%   x 0.97
u32_rand::insert_all            4,765,949            4,887,022              121,073    2.54%   x 0.98
usize_rand::insert_all          4,601,806            4,864,446              262,640    5.71%   x 0.95
u64_rand::insert_all            4,683,740            4,719,502               35,762    0.76%   x 0.99

u16_natural::find_all           170,213              82,438                 -87,775  -51.57%   x 2.06
u32_natural::find_all           275,143              128,202               -146,941  -53.41%   x 2.15
usize_natural::find_all         327,153              157,579               -169,574  -51.83%   x 2.08
u64_natural::find_all           306,819              157,473               -149,346  -48.68%   x 1.95

u16_natural::insert_all         1,484,507            1,180,757             -303,750  -20.46%   x 1.26
u32_natural::insert_all         2,230,220            1,494,354             -735,866  -33.00%   x 1.49
usize_natural::insert_all       1,862,551            947,293               -915,258  -49.14%   x 1.97
u64_natural::insert_all         1,869,248            930,874               -938,374  -50.20%   x 2.01

u16_times_table_32::find_all    5,120                5,430                      310    6.05%   x 0.94
u32_times_table_32::find_all    1,765,581            1,378,418             -387,163  -21.93%   x 1.28
u64_times_table_32::find_all    1,794,407            1,645,669             -148,738   -8.29%   x 1.09

u16_times_table_32::insert_all  77,386               77,025                    -361   -0.47%   x 1.00
u32_times_table_32::insert_all  3,987,172            3,523,361             -463,811  -11.63%   x 1.13
u64_times_table_32::insert_all  3,505,875            3,140,083             -365,792  -10.43%   x 1.12
```

## [Thomas Wang](https://gist.github.com/badboy/6267743) vs `int_hash`
_random **1.05-2.45x** faster, ℕ **3.33-13.09x** faster, 32× table **1.04-1.62x** faster_
```
name                            wang_mix ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               770               314                       -456  -59.22%   x 2.45
u16_rand::find_all              27,370            13,869                 -13,501  -49.33%   x 1.97
u32_rand::find_all              1,687,483         1,257,109             -430,374  -25.50%   x 1.34
usize_rand::find_all            1,733,347         1,304,515             -428,832  -24.74%   x 1.33
u64_rand::find_all              1,719,560         1,349,341             -370,219  -21.53%   x 1.27

u8_rand::insert_all             6,303             5,006                   -1,297  -20.58%   x 1.26
u16_rand::insert_all            283,334           271,069                -12,265   -4.33%   x 1.05
u32_rand::insert_all            5,176,101         4,887,022             -289,079   -5.58%   x 1.06
usize_rand::insert_all          5,171,748         4,864,446             -307,302   -5.94%   x 1.06
u64_rand::insert_all            5,176,835         4,719,502             -457,333   -8.83%   x 1.10

u16_natural::find_all           659,118           82,438                -576,680  -87.49%   x 8.00
u32_natural::find_all           1,677,795         128,202             -1,549,593  -92.36%  x 13.09
usize_natural::find_all         1,721,113         157,579             -1,563,534  -90.84%  x 10.92
u64_natural::find_all           1,721,574         157,473             -1,564,101  -90.85%  x 10.93

u16_natural::insert_all         3,929,079         1,180,757           -2,748,322  -69.95%   x 3.33
u32_natural::insert_all         5,115,708         1,494,354           -3,621,354  -70.79%   x 3.42
usize_natural::insert_all       5,083,581         947,293             -4,136,288  -81.37%   x 5.37
u64_natural::insert_all         5,115,185         930,874             -4,184,311  -81.80%   x 5.50

u16_times_table_32::find_all    7,934             5,430                   -2,504  -31.56%   x 1.46
u32_times_table_32::find_all    1,676,674         1,378,418             -298,256  -17.79%   x 1.22
u64_times_table_32::find_all    1,719,356         1,645,669              -73,687   -4.29%   x 1.04

u16_times_table_32::insert_all  87,113            77,025                 -10,088  -11.58%   x 1.13
u32_times_table_32::insert_all  5,078,900         3,523,361           -1,555,539  -30.63%   x 1.44
u64_times_table_32::insert_all  5,078,080         3,140,083           -1,937,997  -38.16%   x 1.62
```

## `seahash` vs `int_hash`
_random **1.16-5.57x** faster, ℕ **3.65-19.35x** faster, 32× table **1.33-3.11x** faster_
```
name                            seahash_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               1,749                  314                     -1,435  -82.05%   x 5.57
u16_rand::find_all              57,915                 13,869                 -44,046  -76.05%   x 4.18
u32_rand::find_all              2,492,307              1,257,109           -1,235,198  -49.56%   x 1.98
usize_rand::find_all            2,570,775              1,304,515           -1,266,260  -49.26%   x 1.97
u64_rand::find_all              2,420,253              1,349,341           -1,070,912  -44.25%   x 1.79

u8_rand::insert_all             6,627                  5,006                   -1,621  -24.46%   x 1.32
u16_rand::insert_all            321,441                271,069                -50,372  -15.67%   x 1.19
u32_rand::insert_all            5,722,811              4,887,022             -835,789  -14.60%   x 1.17
usize_rand::insert_all          5,649,676              4,864,446             -785,230  -13.90%   x 1.16
u64_rand::insert_all            5,660,883              4,719,502             -941,381  -16.63%   x 1.20

u16_natural::find_all           1,035,114              82,438                -952,676  -92.04%  x 12.56
u32_natural::find_all           2,480,523              128,202             -2,352,321  -94.83%  x 19.35
usize_natural::find_all         2,589,697              157,579             -2,432,118  -93.92%  x 16.43
u64_natural::find_all           2,414,444              157,473             -2,256,971  -93.48%  x 15.33

u16_natural::insert_all         4,304,529              1,180,757           -3,123,772  -72.57%   x 3.65
u32_natural::insert_all         5,652,620              1,494,354           -4,158,266  -73.56%   x 3.78
usize_natural::insert_all       5,623,551              947,293             -4,676,258  -83.15%   x 5.94
u64_natural::insert_all         5,675,929              930,874             -4,745,055  -83.60%   x 6.10

u16_times_table_32::find_all    16,864                 5,430                  -11,434  -67.80%   x 3.11
u32_times_table_32::find_all    2,477,012              1,378,418           -1,098,594  -44.35%   x 1.80
u64_times_table_32::find_all    2,438,979              1,645,669             -793,310  -32.53%   x 1.48

u16_times_table_32::insert_all  102,409                77,025                 -25,384  -24.79%   x 1.33
u32_times_table_32::insert_all  5,547,127              3,523,361           -2,023,766  -36.48%   x 1.57
u64_times_table_32::insert_all  5,798,074              3,140,083           -2,657,991  -45.84%   x 1.85
```

## `twox_hash` vs `int_hash`
_random **1.25-9.55x** faster, ℕ **4.05-23.95x** faster, 32× table **1.68-5.67x** faster_
```
name                            twox_crate ns/iter  int_hash ns/iter  diff ns/iter   diff %  speedup
u8_rand::find_all               2,998               314                     -2,684  -89.53%   x 9.55
u16_rand::find_all              103,272             13,869                 -89,403  -86.57%   x 7.45
u32_rand::find_all              3,053,437           1,257,109           -1,796,328  -58.83%   x 2.43
usize_rand::find_all            3,097,459           1,304,515           -1,792,944  -57.88%   x 2.37
u64_rand::find_all              3,058,458           1,349,341           -1,709,117  -55.88%   x 2.27

u8_rand::insert_all             8,730               5,006                   -3,724  -42.66%   x 1.74
u16_rand::insert_all            362,355             271,069                -91,286  -25.19%   x 1.34
u32_rand::insert_all            6,196,761           4,887,022           -1,309,739  -21.14%   x 1.27
usize_rand::insert_all          6,082,073           4,864,446           -1,217,627  -20.02%   x 1.25
u64_rand::insert_all            6,004,895           4,719,502           -1,285,393  -21.41%   x 1.27

u16_natural::find_all           1,556,816           82,438              -1,474,378  -94.70%  x 18.88
u32_natural::find_all           3,070,581           128,202             -2,942,379  -95.82%  x 23.95
usize_natural::find_all         3,056,006           157,579             -2,898,427  -94.84%  x 19.39
u64_natural::find_all           3,045,873           157,473             -2,888,400  -94.83%  x 19.34

u16_natural::insert_all         4,777,823           1,180,757           -3,597,066  -75.29%   x 4.05
u32_natural::insert_all         6,190,951           1,494,354           -4,696,597  -75.86%   x 4.14
usize_natural::insert_all       6,133,001           947,293             -5,185,708  -84.55%   x 6.47
u64_natural::insert_all         6,074,277           930,874             -5,143,403  -84.68%   x 6.53

u16_times_table_32::find_all    30,815              5,430                  -25,385  -82.38%   x 5.67
u32_times_table_32::find_all    3,058,445           1,378,418           -1,680,027  -54.93%   x 2.22
u64_times_table_32::find_all    3,058,016           1,645,669           -1,412,347  -46.19%   x 1.86

u16_times_table_32::insert_all  129,047             77,025                 -52,022  -40.31%   x 1.68
u32_times_table_32::insert_all  6,345,601           3,523,361           -2,822,240  -44.48%   x 1.80
u64_times_table_32::insert_all  5,911,372           3,140,083           -2,771,289  -46.88%   x 1.88
```
