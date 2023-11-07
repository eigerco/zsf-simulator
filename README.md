# Zcash Sustainaiblity Fund simulator

This short Rust program simulates Zcash Block Rewards (aka Block Subsidies) assuming Zcash Sustainability Fund backed smoothing out of the issuance curve will be implemented (https://github.com/zcash/zips/pull/706).

## Running

### Devenv
You can use [devenv.sh](https://devenv.sh/) to setup a development environment. Otherwise follow the instructions below.

### Manual setup

To run the program, you need to have Rust installed. You'll also need some extra libraries to enable plotting, on Ubuntu do:

```
sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
```

 Then, clone this repository and run:

```
cargo run
```

## Output

The program will output the final block that subsidies will be paid out from:

```
Last block is 47917869 in ~113.88 years
Final block subsidy: 1 (~0 ZEC)
Final ZSF balance: 0 (~0 ZEC)
```

As well as a summary of subsidies paid out during 4 year periods in between halvings in comparison with the current
issuance scheme:

```
Halvings:
Halving  1 at block  1680000:
  ZSF subsidies:    262523884819889 (~ 2625238.848 ZEC,        1.563 ZEC per block)
  legacy subsidies: 262500000000000 (~ 2625000.000 ZEC,        1.562 ZEC per block)
  difference:           23884819889 (~         238 ZEC),         ZSF/legacy: 1.0001
Halving  2 at block  3360000:
  ZSF subsidies:    131259082555273 (~ 1312590.826 ZEC,        0.781 ZEC per block)
  legacy subsidies: 131250000000000 (~ 1312500.000 ZEC,        0.781 ZEC per block)
  difference:            9082555273 (~          90 ZEC),         ZSF/legacy: 1.0001
Halving  3 at block  5040000:
  ZSF subsidies:     65628165536456 (~  656281.655 ZEC,        0.391 ZEC per block)
  legacy subsidies:  65625000000000 (~  656250.000 ZEC,        0.391 ZEC per block)
  difference:            3165536456 (~          31 ZEC),         ZSF/legacy: 1.0000
Halving  4 at block  6720000:
  ZSF subsidies:     32813394913130 (~  328133.949 ZEC,        0.195 ZEC per block)
  legacy subsidies:  32812500000000 (~  328125.000 ZEC,        0.195 ZEC per block)
  difference:             894913130 (~           8 ZEC),         ZSF/legacy: 1.0000
Halving  5 at block  8400000:
  ZSF subsidies:     16406353535358 (~  164063.535 ZEC,        0.098 ZEC per block)
  legacy subsidies:  16406250000000 (~  164062.500 ZEC,        0.098 ZEC per block)
  difference:             103535358 (~           1 ZEC),         ZSF/legacy: 1.0000
Halving  6 at block 10080000:
  ZSF subsidies:      8203004810267 (~   82030.048 ZEC,        0.049 ZEC per block)
  legacy subsidies:   8203124160000 (~   82031.242 ZEC,        0.049 ZEC per block)
  difference:            -119349733 (~          -1 ZEC),         ZSF/legacy: 1.0000
Halving  7 at block 11760000:
  ZSF subsidies:      4101416429306 (~   41014.164 ZEC,        0.024 ZEC per block)
  legacy subsidies:   4101562080000 (~   41015.621 ZEC,        0.024 ZEC per block)
  difference:            -145650694 (~          -1 ZEC),         ZSF/legacy: 1.0000
Halving  8 at block 13440000:
  ZSF subsidies:      2050665226986 (~   20506.652 ZEC,        0.012 ZEC per block)
  legacy subsidies:   2050781040000 (~   20507.810 ZEC,        0.012 ZEC per block)
  difference:            -115813014 (~          -1 ZEC),         ZSF/legacy: 0.9999
Halving  9 at block 15120000:
  ZSF subsidies:      1025311119937 (~   10253.111 ZEC,        0.006 ZEC per block)
  legacy subsidies:   1025389680000 (~   10253.897 ZEC,        0.006 ZEC per block)
  difference:             -78560063 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 10 at block 16800000:
  ZSF subsidies:       512644813724 (~    5126.448 ZEC,        0.003 ZEC per block)
  legacy subsidies:    512694000000 (~    5126.940 ZEC,        0.003 ZEC per block)
  difference:             -49186276 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 11 at block 18480000:
  ZSF subsidies:       256317033772 (~    2563.170 ZEC,        0.002 ZEC per block)
  legacy subsidies:    256346160000 (~    2563.462 ZEC,        0.002 ZEC per block)
  difference:             -29126228 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 12 at block 20160000:
  ZSF subsidies:       128155830300 (~    1281.558 ZEC,        0.001 ZEC per block)
  legacy subsidies:    128172240000 (~    1281.722 ZEC,        0.001 ZEC per block)
  difference:             -16409700 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 13 at block 21840000:
  ZSF subsidies:        64076572065 (~     640.766 ZEC,        0.000 ZEC per block)
  legacy subsidies:     64085280000 (~     640.853 ZEC,        0.000 ZEC per block)
  difference:              -8707935 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 14 at block 23520000:
  ZSF subsidies:        32037614417 (~     320.376 ZEC,        0.000 ZEC per block)
  legacy subsidies:     32042640000 (~     320.426 ZEC,        0.000 ZEC per block)
  difference:              -5025583 (~           0 ZEC),         ZSF/legacy: 0.9998
Halving 15 at block 25200000:
  ZSF subsidies:        16018471375 (~     160.185 ZEC,        0.000 ZEC per block)
  legacy subsidies:     16020480000 (~     160.205 ZEC,        0.000 ZEC per block)
  difference:              -2008625 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 16 at block 26880000:
  ZSF subsidies:         8009067846 (~      80.091 ZEC,        0.000 ZEC per block)
  legacy subsidies:      8010240000 (~      80.102 ZEC,        0.000 ZEC per block)
  difference:              -1172154 (~           0 ZEC),         ZSF/legacy: 0.9999
Halving 17 at block 28560000:
  ZSF subsidies:         4004449844 (~      40.044 ZEC,        0.000 ZEC per block)
  legacy subsidies:      4005120000 (~      40.051 ZEC,        0.000 ZEC per block)
  difference:               -670156 (~           0 ZEC),         ZSF/legacy: 0.9998
Halving 18 at block 30240000:
  ZSF subsidies:         2002182886 (~      20.022 ZEC,        0.000 ZEC per block)
  legacy subsidies:      2002560000 (~      20.026 ZEC,        0.000 ZEC per block)
  difference:               -377114 (~           0 ZEC),         ZSF/legacy: 0.9998
Halving 19 at block 31920000:
  ZSF subsidies:         1001070275 (~      10.011 ZEC,        0.000 ZEC per block)
  legacy subsidies:      1001280000 (~      10.013 ZEC,        0.000 ZEC per block)
  difference:               -209725 (~           0 ZEC),         ZSF/legacy: 0.9998
Halving 20 at block 33600000:
  ZSF subsidies:          500525701 (~       5.005 ZEC,        0.000 ZEC per block)
  legacy subsidies:       500640000 (~       5.006 ZEC,        0.000 ZEC per block)
  difference:               -114299 (~           0 ZEC),         ZSF/legacy: 0.9998
Halving 21 at block 35280000:
  ZSF subsidies:          250255480 (~       2.503 ZEC,        0.000 ZEC per block)
  legacy subsidies:       250320000 (~       2.503 ZEC,        0.000 ZEC per block)
  difference:                -64520 (~           0 ZEC),         ZSF/legacy: 0.9997
Halving 22 at block 36960000:
  ZSF subsidies:          125126575 (~       1.251 ZEC,        0.000 ZEC per block)
  legacy subsidies:       124320000 (~       1.243 ZEC,        0.000 ZEC per block)
  difference:                806575 (~           0 ZEC),         ZSF/legacy: 1.0065
Halving 23 at block 38640000:
  ZSF subsidies:           62556140 (~       0.626 ZEC,        0.000 ZEC per block)
  legacy subsidies:        62160000 (~       0.622 ZEC,        0.000 ZEC per block)
  difference:                396140 (~           0 ZEC),         ZSF/legacy: 1.0064
Halving 24 at block 40320000:
  ZSF subsidies:           31271589 (~       0.313 ZEC,        0.000 ZEC per block)
  legacy subsidies:        30240000 (~       0.302 ZEC,        0.000 ZEC per block)
  difference:               1031589 (~           0 ZEC),         ZSF/legacy: 1.0341
Halving 25 at block 42000000:
  ZSF subsidies:           15660304 (~       0.157 ZEC,        0.000 ZEC per block)
  legacy subsidies:        15120000 (~       0.151 ZEC,        0.000 ZEC per block)
  difference:                540304 (~           0 ZEC),         ZSF/legacy: 1.0357
Halving 26 at block 43680000:
  ZSF subsidies:            7766952 (~       0.078 ZEC,        0.000 ZEC per block)
  legacy subsidies:         6720000 (~       0.067 ZEC,        0.000 ZEC per block)
  difference:               1046952 (~           0 ZEC),         ZSF/legacy: 1.1558
Halving 27 at block 45360000:
  ZSF subsidies:            3962388 (~       0.040 ZEC,        0.000 ZEC per block)
  legacy subsidies:         3360000 (~       0.034 ZEC,        0.000 ZEC per block)
  difference:                602388 (~           0 ZEC),         ZSF/legacy: 1.1793
Halving 28 at block 47040000:
  ZSF subsidies:            1814216 (~       0.018 ZEC,        0.000 ZEC per block)
  legacy subsidies:         1680000 (~       0.017 ZEC,        0.000 ZEC per block)
  difference:                134216 (~           0 ZEC),         ZSF/legacy: 1.0799
```

The program will also output PNG files with plots of the issuance curve in `plots/`:

![ZSF balance simulation](plots/zsf_balance.png)
![ZSF block subsidy simulation](plots/zsf_block_subsidy.png)