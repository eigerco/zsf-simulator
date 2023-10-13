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
Block 47917869 (~113.88 years): Subsidy: 1 (~0 ZEC), ZSF: 0 (~0 ZEC)
```

As well as a summary of subsidies paid out during 4 year periods in between halvings in comparison with the current
issuance scheme:

```
Halving  1 at block  1680000:
  ZSF subsidies:    262523884819889 (~2625238 ZEC)
  legacy subsidies: 262500000000000 (~2625000 ZEC) (1.56250000 ZEC per block)
  difference:           23884819889 (~    238 ZEC), ZSF/legacy: 1.0001
Halving  2 at block  3360000:
  ZSF subsidies:    131259082555273 (~1312590 ZEC)
  legacy subsidies: 131250000000000 (~1312500 ZEC) (0.78125000 ZEC per block)
  difference:            9082555273 (~     90 ZEC), ZSF/legacy: 1.0001
Halving  3 at block  5040000:
  ZSF subsidies:     65628165536456 (~ 656281 ZEC)
  legacy subsidies:  65625000000000 (~ 656250 ZEC) (0.39062500 ZEC per block)
  difference:            3165536456 (~     31 ZEC), ZSF/legacy: 1.0000
Halving  4 at block  6720000:
  ZSF subsidies:     32813394913130 (~ 328133 ZEC)
  legacy subsidies:  32812500000000 (~ 328125 ZEC) (0.19531250 ZEC per block)
  difference:             894913130 (~      8 ZEC), ZSF/legacy: 1.0000
Halving  5 at block  8400000:
  ZSF subsidies:     16406353535358 (~ 164063 ZEC)
  legacy subsidies:  16406250000000 (~ 164062 ZEC) (0.09765625 ZEC per block)
  difference:             103535358 (~      1 ZEC), ZSF/legacy: 1.0000
Halving  6 at block 10080000:
  ZSF subsidies:      8203004810267 (~  82030 ZEC)
  legacy subsidies:   8203124160000 (~  82031 ZEC) (0.04882812 ZEC per block)
  difference:            -119349733 (~     -1 ZEC), ZSF/legacy: 1.0000
Halving  7 at block 11760000:
  ZSF subsidies:      4101416429306 (~  41014 ZEC)
  legacy subsidies:   4101562080000 (~  41015 ZEC) (0.02441406 ZEC per block)
  difference:            -145650694 (~     -1 ZEC), ZSF/legacy: 1.0000
Halving  8 at block 13440000:
  ZSF subsidies:      2050665226986 (~  20506 ZEC)
  legacy subsidies:   2050781040000 (~  20507 ZEC) (0.01220703 ZEC per block)
  difference:            -115813014 (~     -1 ZEC), ZSF/legacy: 0.9999
Halving  9 at block 15120000:
  ZSF subsidies:      1025311119937 (~  10253 ZEC)
  legacy subsidies:   1025389680000 (~  10253 ZEC) (0.00610351 ZEC per block)
  difference:             -78560063 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 10 at block 16800000:
  ZSF subsidies:       512644813724 (~   5126 ZEC)
  legacy subsidies:    512694000000 (~   5126 ZEC) (0.00305175 ZEC per block)
  difference:             -49186276 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 11 at block 18480000:
  ZSF subsidies:       256317033772 (~   2563 ZEC)
  legacy subsidies:    256346160000 (~   2563 ZEC) (0.00152587 ZEC per block)
  difference:             -29126228 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 12 at block 20160000:
  ZSF subsidies:       128155830300 (~   1281 ZEC)
  legacy subsidies:    128172240000 (~   1281 ZEC) (0.00076293 ZEC per block)
  difference:             -16409700 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 13 at block 21840000:
  ZSF subsidies:        64076572065 (~    640 ZEC)
  legacy subsidies:     64085280000 (~    640 ZEC) (0.00038146 ZEC per block)
  difference:              -8707935 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 14 at block 23520000:
  ZSF subsidies:        32037614417 (~    320 ZEC)
  legacy subsidies:     32042640000 (~    320 ZEC) (0.00019073 ZEC per block)
  difference:              -5025583 (~      0 ZEC), ZSF/legacy: 0.9998
Halving 15 at block 25200000:
  ZSF subsidies:        16018471375 (~    160 ZEC)
  legacy subsidies:     16020480000 (~    160 ZEC) (0.00009536 ZEC per block)
  difference:              -2008625 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 16 at block 26880000:
  ZSF subsidies:         8009067846 (~     80 ZEC)
  legacy subsidies:      8010240000 (~     80 ZEC) (0.00004768 ZEC per block)
  difference:              -1172154 (~      0 ZEC), ZSF/legacy: 0.9999
Halving 17 at block 28560000:
  ZSF subsidies:         4004449844 (~     40 ZEC)
  legacy subsidies:      4005120000 (~     40 ZEC) (0.00002384 ZEC per block)
  difference:               -670156 (~      0 ZEC), ZSF/legacy: 0.9998
Halving 18 at block 30240000:
  ZSF subsidies:         2002182886 (~     20 ZEC)
  legacy subsidies:      2002560000 (~     20 ZEC) (0.00001192 ZEC per block)
  difference:               -377114 (~      0 ZEC), ZSF/legacy: 0.9998
Halving 19 at block 31920000:
  ZSF subsidies:         1001070275 (~     10 ZEC)
  legacy subsidies:      1001280000 (~     10 ZEC) (0.00000596 ZEC per block)
  difference:               -209725 (~      0 ZEC), ZSF/legacy: 0.9998
Halving 20 at block 33600000:
  ZSF subsidies:          500525701 (~      5 ZEC)
  legacy subsidies:       500640000 (~      5 ZEC) (0.00000298 ZEC per block)
  difference:               -114299 (~      0 ZEC), ZSF/legacy: 0.9998
Halving 21 at block 35280000:
  ZSF subsidies:          250255480 (~      2 ZEC)
  legacy subsidies:       250320000 (~      2 ZEC) (0.00000149 ZEC per block)
  difference:                -64520 (~      0 ZEC), ZSF/legacy: 0.9997
Halving 22 at block 36960000:
  ZSF subsidies:          125126575 (~      1 ZEC)
  legacy subsidies:       124320000 (~      1 ZEC) (0.00000074 ZEC per block)
  difference:                806575 (~      0 ZEC), ZSF/legacy: 1.0065
Halving 23 at block 38640000:
  ZSF subsidies:           62556140 (~      0 ZEC)
  legacy subsidies:        62160000 (~      0 ZEC) (0.00000037 ZEC per block)
  difference:                396140 (~      0 ZEC), ZSF/legacy: 1.0064
Halving 24 at block 40320000:
  ZSF subsidies:           31271589 (~      0 ZEC)
  legacy subsidies:        30240000 (~      0 ZEC) (0.00000018 ZEC per block)
  difference:               1031589 (~      0 ZEC), ZSF/legacy: 1.0341
Halving 25 at block 42000000:
  ZSF subsidies:           15660304 (~      0 ZEC)
  legacy subsidies:        15120000 (~      0 ZEC) (0.00000009 ZEC per block)
  difference:                540304 (~      0 ZEC), ZSF/legacy: 1.0357
Halving 26 at block 43680000:
  ZSF subsidies:            7766952 (~      0 ZEC)
  legacy subsidies:         6720000 (~      0 ZEC) (0.00000004 ZEC per block)
  difference:               1046952 (~      0 ZEC), ZSF/legacy: 1.1558
Halving 27 at block 45360000:
  ZSF subsidies:            3962388 (~      0 ZEC)
  legacy subsidies:         3360000 (~      0 ZEC) (0.00000002 ZEC per block)
  difference:                602388 (~      0 ZEC), ZSF/legacy: 1.1793
Halving 28 at block 47040000:
  ZSF subsidies:            1814216 (~      0 ZEC)
  legacy subsidies:         1680000 (~      0 ZEC) (0.00000001 ZEC per block)
  difference:                134216 (~      0 ZEC), ZSF/legacy: 1.0799

```

The program will also output PNG files with plots of the issuance curve in `plots/`:

![ZSF balance simulation](plots/zsf_balance.png)
![ZSF block subsidy simulation](plots/zsf_block_subsidy.png)