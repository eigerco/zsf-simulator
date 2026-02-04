# Network Sustainability Mechanism simulator

This short Rust program simulates Zcash Block Subsidies, assuming that smoothing of the
issuance curve in line with the Network Sustainability Mechanism will be implemented as
specified in [ZIP 234](https://zips.z.cash/zip-0234).

## Running

### Manual setup

To run the program, you need to have Rust installed. You'll also need some extra libraries
to enable plotting, on Ubuntu do:

```
sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
```

Then, clone this repository and run:

```
cargo run
```

## Disclaimer

This fork of Eiger's "[ZSF simulator](https://github.com/eigerco/zsf-simulator)" has been
significantly modified by me, Daira-Emma Hopwood. Any errors in the interpretation of
ZIP 234 are my own.

## Output

The program will output the final block that subsidies would be paid out from
according to ZIP 234:

```
DEPLOYMENT_BLOCK_HEIGHT = 3662982 (2.23 years after 2nd halving)
Last block with non-zero subsidy is at height 50788960 in ~114.23 years after the 2nd halving.
Final block subsidy: 1 (~0.00000001 ZEC)
Final NSM balance: 0 (~0 ZEC)
```

It also gives a summary of subsidies that would be paid out during 4-year periods
after NSM deployment in comparison with the current issuance scheme. (Note that
these periods do not line up with halving periods under the current scheme.)

```
Four-year periods:
Years   0..  4 at heights  3662982.. 5342982:
  NSM subsidies:    189352008356142 (~ 1893520.084 ZEC,   1.12709529 ZEC per block)
  no-NSM subsidies: 189329609375000 (~ 1893296.094 ZEC,   1.12696196 ZEC per block)
  difference:           22398981142 (~     223.990 ZEC),         NSM/no-NSM: 1.0001
Years   4..  8 at heights  5342982.. 7022982:
  NSM subsidies:     94673941434973 (~  946739.414 ZEC,   0.56353537 ZEC per block)
  no-NSM subsidies:  94664726562500 (~  946647.266 ZEC,   0.56348052 ZEC per block)
  difference:            9214872473 (~      92.149 ZEC),         NSM/no-NSM: 1.0001
Years   8.. 12 at heights  7022982.. 8702982:
  NSM subsidies:     47335978429372 (~  473359.784 ZEC,   0.28176178 ZEC per block)
  no-NSM subsidies:  47332363281250 (~  473323.633 ZEC,   0.28174026 ZEC per block)
  difference:            3615148122 (~      36.151 ZEC),         NSM/no-NSM: 1.0001
Years  12.. 16 at heights  8702982..10382982:
  NSM subsidies:     23667493081637 (~  236674.931 ZEC,   0.14087794 ZEC per block)
  no-NSM subsidies:  23666181640625 (~  236661.816 ZEC,   0.14087013 ZEC per block)
  difference:            1311441012 (~      13.114 ZEC),         NSM/no-NSM: 1.0001
Years  16.. 20 at heights 10382982..12062982:
  NSM subsidies:     11833498478705 (~  118334.985 ZEC,   0.07043749 ZEC per block)
  no-NSM subsidies:  11833090352021 (~  118330.904 ZEC,   0.07043506 ZEC per block)
  difference:             408126684 (~       4.081 ZEC),         NSM/no-NSM: 1.0000
Years  20.. 24 at heights 12062982..13742982:
  NSM subsidies:      5916625210905 (~   59166.252 ZEC,   0.03521801 ZEC per block)
  no-NSM subsidies:   5916544804302 (~   59165.448 ZEC,   0.03521753 ZEC per block)
  difference:              80406603 (~       0.804 ZEC),         NSM/no-NSM: 1.0000
Years  24.. 28 at heights 13742982..15422982:
  NSM subsidies:      2958250593150 (~   29582.506 ZEC,   0.01760863 ZEC per block)
  no-NSM subsidies:   2958272402151 (~   29582.724 ZEC,   0.01760876 ZEC per block)
  difference:             -21809001 (~      -0.218 ZEC),         NSM/no-NSM: 1.0000
Years  28.. 32 at heights 15422982..17102982:
  NSM subsidies:      1479094290614 (~   14790.943 ZEC,   0.00880413 ZEC per block)
  no-NSM subsidies:   1479135732784 (~   14791.357 ZEC,   0.00880438 ZEC per block)
  difference:             -41442170 (~      -0.414 ZEC),         NSM/no-NSM: 1.0000
Years  32.. 36 at heights 17102982..18782982:
  NSM subsidies:       739531642870 (~    7395.316 ZEC,   0.00440197 ZEC per block)
  no-NSM subsidies:    739567026392 (~    7395.670 ZEC,   0.00440218 ZEC per block)
  difference:             -35383522 (~      -0.354 ZEC),         NSM/no-NSM: 1.0000
Years  36.. 40 at heights 18782982..20462982:
  NSM subsidies:       369758070268 (~    3697.581 ZEC,   0.00220094 ZEC per block)
  no-NSM subsidies:    369782673196 (~    3697.827 ZEC,   0.00220109 ZEC per block)
  difference:             -24602928 (~      -0.246 ZEC),         NSM/no-NSM: 0.9999
Years  40.. 44 at heights 20462982..22142982:
  NSM subsidies:       184875159527 (~    1848.752 ZEC,   0.00110045 ZEC per block)
  no-NSM subsidies:    184890496598 (~    1848.905 ZEC,   0.00110054 ZEC per block)
  difference:             -15337071 (~      -0.153 ZEC),         NSM/no-NSM: 0.9999
Years  44.. 48 at heights 22142982..23822982:
  NSM subsidies:        92435642223 (~     924.356 ZEC,   0.00055021 ZEC per block)
  no-NSM subsidies:     92444408299 (~     924.444 ZEC,   0.00055026 ZEC per block)
  difference:              -8766076 (~      -0.088 ZEC),         NSM/no-NSM: 0.9999
Years  48.. 52 at heights 23822982..25502982:
  NSM subsidies:        46216852306 (~     462.169 ZEC,   0.00027510 ZEC per block)
  no-NSM subsidies:     46221832441 (~     462.218 ZEC,   0.00027513 ZEC per block)
  difference:              -4980135 (~      -0.050 ZEC),         NSM/no-NSM: 0.9999
Years  52.. 56 at heights 25502982..27182982:
  NSM subsidies:        23107941677 (~     231.079 ZEC,   0.00013755 ZEC per block)
  no-NSM subsidies:     23110447929 (~     231.104 ZEC,   0.00013756 ZEC per block)
  difference:              -2506252 (~      -0.025 ZEC),         NSM/no-NSM: 0.9999
Years  56.. 60 at heights 27182982..28862982:
  NSM subsidies:        11553728657 (~     115.537 ZEC,   0.00006877 ZEC per block)
  no-NSM subsidies:     11554852256 (~     115.549 ZEC,   0.00006878 ZEC per block)
  difference:              -1123599 (~      -0.011 ZEC),         NSM/no-NSM: 0.9999
Years  60.. 64 at heights 28862982..30542982:
  NSM subsidies:         5776743256 (~      57.767 ZEC,   0.00003439 ZEC per block)
  no-NSM subsidies:      5777426128 (~      57.774 ZEC,   0.00003439 ZEC per block)
  difference:               -682872 (~      -0.007 ZEC),         NSM/no-NSM: 0.9999
Years  64.. 68 at heights 30542982..32222982:
  NSM subsidies:         2888311019 (~      28.883 ZEC,   0.00001719 ZEC per block)
  no-NSM subsidies:      2888713064 (~      28.887 ZEC,   0.00001719 ZEC per block)
  difference:               -402045 (~      -0.004 ZEC),         NSM/no-NSM: 0.9999
Years  68.. 72 at heights 32222982..33902982:
  NSM subsidies:         1444125052 (~      14.441 ZEC,   0.00000860 ZEC per block)
  no-NSM subsidies:      1444356532 (~      14.444 ZEC,   0.00000860 ZEC per block)
  difference:               -231480 (~      -0.002 ZEC),         NSM/no-NSM: 0.9998
Years  72.. 76 at heights 33902982..35582982:
  NSM subsidies:          722047128 (~       7.220 ZEC,   0.00000430 ZEC per block)
  no-NSM subsidies:       722178266 (~       7.222 ZEC,   0.00000430 ZEC per block)
  difference:               -131138 (~      -0.001 ZEC),         NSM/no-NSM: 0.9998
Years  76.. 80 at heights 35582982..37262982:
  NSM subsidies:          361015488 (~       3.610 ZEC,   0.00000215 ZEC per block)
  no-NSM subsidies:       361089133 (~       3.611 ZEC,   0.00000215 ZEC per block)
  difference:                -73645 (~      -0.001 ZEC),         NSM/no-NSM: 0.9998
Years  80.. 84 at heights 37262982..38942982:
  NSM subsidies:          180506560 (~       1.805 ZEC,   0.00000107 ZEC per block)
  no-NSM subsidies:       180076275 (~       1.801 ZEC,   0.00000107 ZEC per block)
  difference:                430285 (~       0.004 ZEC),         NSM/no-NSM: 1.0024
Years  84.. 88 at heights 38942982..40622982:
  NSM subsidies:           90245698 (~       0.902 ZEC,   0.00000054 ZEC per block)
  no-NSM subsidies:        89666429 (~       0.897 ZEC,   0.00000053 ZEC per block)
  difference:                579269 (~       0.006 ZEC),         NSM/no-NSM: 1.0065
Years  88.. 92 at heights 40622982..42302982:
  NSM subsidies:           45126662 (~       0.451 ZEC,   0.00000027 ZEC per block)
  no-NSM subsidies:        44364923 (~       0.444 ZEC,   0.00000026 ZEC per block)
  difference:                761739 (~       0.008 ZEC),         NSM/no-NSM: 1.0172
Years  92.. 96 at heights 42302982..43982982:
  NSM subsidies:           22552905 (~       0.226 ZEC,   0.00000013 ZEC per block)
  no-NSM subsidies:        21810753 (~       0.218 ZEC,   0.00000013 ZEC per block)
  difference:                742152 (~       0.007 ZEC),         NSM/no-NSM: 1.0340
Years  96..100 at heights 43982982..45662982:
  NSM subsidies:           11280615 (~       0.113 ZEC,   0.00000007 ZEC per block)
  no-NSM subsidies:        10437085 (~       0.104 ZEC,   0.00000006 ZEC per block)
  difference:                843530 (~       0.008 ZEC),         NSM/no-NSM: 1.0808
Years 100..104 at heights 45662982..47342982:
  NSM subsidies:            5609808 (~       0.056 ZEC,   0.00000003 ZEC per block)
  no-NSM subsidies:         4846834 (~       0.048 ZEC,   0.00000003 ZEC per block)
  difference:                762974 (~       0.008 ZEC),         NSM/no-NSM: 1.1574
Years 104..108 at heights 47342982..49022982:
  NSM subsidies:            2702325 (~       0.027 ZEC,   0.00000002 ZEC per block)
  no-NSM subsidies:         2423417 (~       0.024 ZEC,   0.00000001 ZEC per block)
  difference:                278908 (~       0.003 ZEC),         NSM/no-NSM: 1.1151
Years 108..112 at heights 49022982..50702982:
  NSM subsidies:            1680000 (~       0.017 ZEC,   0.00000001 ZEC per block)
  no-NSM subsidies:          743417 (~       0.007 ZEC,   0.00000000 ZEC per block)
  difference:                936583 (~       0.009 ZEC),         NSM/no-NSM: 2.2598
```

The program will also output PNG files with plots of the issuance curve in `plots/`:

![NSM balance simulation](plots/nsm_balance.png)
![NSM block subsidy simulation](plots/nsm_block_subsidy.png)

## About this repo

This simulator is part of our larger efforts to implement the Network Sustainability Mechanism.
They are described in a ZconV presentation by Eiger and Shielded Labs (from when it was called the
Zcash Sustainability Fund) that is available [here](https://www.youtube.com/watch?v=_QSYgvDV33k).

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low level implementations and
core components. We believe in Zcash because privacy is critical to a well functioning society.

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)
