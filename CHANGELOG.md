# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.87.0] - 2023-01-29

### Added
* `apply`: add decimal separator --replacement option to thousands operation. This fully rounds out `thousands` formatting, as it will allow formatting numbers to support "euro-style" formats (e.g. 1.234.567,89 instead of 1,234,567.89) https://github.com/jqnatividad/qsv/pull/749
* `apply`: add round operation; also refactored thousands operation to use more appropriate `--formatstr` option instead of `--comparand` option to specify "format" of thousands separator policy https://github.com/jqnatividad/qsv/pull/751
* `applydp`: add round operation  https://github.com/jqnatividad/qsv/pull/752

### Changed
* changed MSRV policy to track latest Rust version in Homebrew, instead of latest Rust stable
* removed excess trailing whitespace in `apply` & `applydp` usage text
* moved `round_num` function from `stats.rs` to `util.rs` so it can be used in round operation in `apply` and `applydp`
* cargo update bump dependencies, notably tokio from 1.24.2 to 1.25.0
* pin Rust nightly to 2023-01-28

### Fixed
* `apply`: corrected thousands operation usage text - `hexfour` not `hex_four` https://github.com/jqnatividad/qsv/commit/6545aa2b3ce470b5f6c039c998e9f6fc21a6ad84


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.86.0...0.87.0


## [0.86.0] - 2023-01-29

### Added
* `apply`: added `thousands` operation which adds thousands separators to numeric values.
Specify the separator policy with --comparand (default: comma). The valid policies are:
comma, dot, space, underscore, hexfour (place a space every four hex digits) and
indiancomma (place a comma every two digits, except the last three digits). https://github.com/jqnatividad/qsv/pull/748
* `searchset`: added `--unmatched-output` option. This was done to allow Datapusher+ to screen for PIIs more efficiently. Writing PII candidate records in one CSV file, and the "clean" records in another CSV in just one pass.  https://github.com/jqnatividad/qsv/pull/742


### Changed
* `fetch` & `fetchpost`: expanded usage text info on HTTP2 Adaptive Flow Control support
* `fetchpost`: added more detail about `--compress` option
* `stats`: added more tests
* updated prebuilt zip archive READMEs https://github.com/jqnatividad/qsv/commit/072973efd7947a93773b2783d098eeace17d963d
* Bump redis from 0.22.2 to 0.22.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/741
* Bump ahash from 0.8.2 to 0.8.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/743
* Bump jql from 5.1.4 to 5.1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/747
* applied select clippy recommendations
* cargo update bump several indirect dependencies
* pin Rust nightly to 2023-01-27


### Fixed
* `stats`: fixed antimodes null display. Use the literal `NULL` instead of just "" when listing NULL as an antimode. https://github.com/jqnatividad/qsv/pull/745
* `tojsonl`: fixed invalid escaping of JSON values https://github.com/jqnatividad/qsv/pull/746

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.85.0...0.86.0

## [0.85.0] - 2023-01-22

### Added
* Update csvs_convert by @kindly in https://github.com/jqnatividad/qsv/pull/736
* `sniff`: added `--delimiter` option https://github.com/jqnatividad/qsv/pull/732
* `fetchpost`: add `--compress` option in https://github.com/jqnatividad/qsv/pull/737
* `searchset`: several tweaks for PII screening requirement of Datapusher+. `--flag` option now shows regex labels instead of just row number; new `--flag-matches-only` option sends only matching rows to output when used with `--flag`; `--json` option returns rows_with_matches, total_matches and rowcount as json to stderr. https://github.com/jqnatividad/qsv/pull/738

### Changed
* `luau`: minor tweaks to increase code readability https://github.com/jqnatividad/qsv/commit/31d01c8b9eb1fe85262e9bf5fd237ae4493d562c
* `stats`: now normalize after rounding. Normalizing strips trailing zeroes and converts -0.0 to 0.0. https://github.com/jqnatividad/qsv/commit/f838272b4deb79d25ca5704cf3c89652c0b9a3bb
* `safenames`: mention CKAN-specific options https://github.com/jqnatividad/qsv/commit/f371ac25ba0c27e48b7b9b14a37dc47913cf0095
* `fetch` & `fetchpost`: document decompression priority https://github.com/jqnatividad/qsv/commit/43ce13c4bf7eb23dc5d051d522d6d52d3cc255aa
* Bump actix-governor from 0.3.2 to 0.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/728
* Bump sysinfo from 0.27.6 to 0.27.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/730
* Bump serial_test from 0.10.0 to 1.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/729
* Bump pyo3 from 0.17.3 to 0.18.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/731
* Bump reqwest from 0.11.13 to 0.11.14 by @dependabot in https://github.com/jqnatividad/qsv/pull/734
* cargo update bump for other dependencies
* pin Rust nightly to 2023-01-21

### Fixed
* `sniff`: now checks that `--sample` size is greater than zero https://github.com/jqnatividad/qsv/commit/cd4c390ce4322d7076866be27025d67800bc60e2

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.84.0...0.85.0

## [0.84.0] - 2023-01-14

### Added
* `headers`: added `--trim` option to trim quote and spaces from headers https://github.com/jqnatividad/qsv/pull/726


### Changed
* `input`: `--trim-headers` option also removes excess quotes https://github.com/jqnatividad/qsv/pull/727
* `safenames`: trim quotes and spaces from headers https://github.com/jqnatividad/qsv/commit/0260833bc8b36ea6e6ccb9e79687c76470a8a6b0
* cargo update bump dependencies
* pin Rust nightly to 2022-01-13


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.83.0...0.84.0

## [0.83.0] - 2023-01-13

### Added
* `stats`: add sparsity to "streaming" statistics https://github.com/jqnatividad/qsv/pull/719
* `schema`: also infer enum constraints for integer fields. Not only good for validation, this is also required by `tojsonl` for smarter boolean inferencing https://github.com/jqnatividad/qsv/pull/721

### Changed
* `stats`: change `--typesonly` so it will not automatically `--infer-dates`. Let the user decide. https://github.com/jqnatividad/qsv/pull/718
* `stats`: if median is already known, use it to calculate Median Absolute Deviation https://github.com/jqnatividad/qsv/commit/08ed08da4651a96bf05372b34b670063fbcec14f
* `tojsonl`: smarter boolean inferencing. It will infer a column as boolean if it only has a domain of two values,
and the first character of the values are one of the following case-insensitive "truthy/falsy"
combinations: t/f; t/null; 1/0; 1/null; y/n & y/null are treated as true/false. https://github.com/jqnatividad/qsv/pull/722 and https://github.com/jqnatividad/qsv/pull/723
* `safenames`: process `--reserved` option before `--prefix` option. https://github.com/jqnatividad/qsv/commit/b333549199726a3e92b95fb1d501fbdbbeede34a
* `strum` and `strum-macros` are no longer optional dependencies as we use it with all the binary variants now https://github.com/jqnatividad/qsv/commit/bea6e00fc400e8fafa2938832f8654d97c45fe34
* Bump qsv-stats from 0.6.0 to 0.7.0
* Bump sysinfo from 0.27.3 to 0.27.6
* Bump hashbrown from 0.13.1 to 0.13.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/720
* Bump actions/setup-python from 4.4.0 to 4.5.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/724
* change MSRV from 1.66.0 to 1.66.1

* cargo update bump indirect dependencies
* pin Rust nightly to 2023-01-12

### Fixed
* `safenames`: fixed `--prefix` option. When checking for invalid underscore prefix, it was checking for hyphen, not underscore, causing a problem with Datapusher+ https://github.com/jqnatividad/qsv/commit/4fbbfd3a479b6678fa9d4c823fd00b592b326c7a


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.82.0...0.83.0


## [0.82.0] - 2023-01-09

### Added
* `diff`: Find the difference between two CSVs ludicrously fast! by @janriemer in https://github.com/jqnatividad/qsv/pull/711
* `stats`: added [Median Absolute Deviation](https://en.wikipedia.org/wiki/Median_absolute_deviation) (MAD) https://github.com/jqnatividad/qsv/pull/715
* added Testing section to README https://github.com/jqnatividad/qsv/commit/517d69b496aaa9535a2b23b05e44a5999d8ef994

### Changed
* `validate`: schema less validation error improvements https://github.com/jqnatividad/qsv/pull/703
* `stats`: faster date inferencing https://github.com/jqnatividad/qsv/pull/706
* `stats`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/15e6284c20cccf4a6b74498336d31b0d7ba03285 https://github.com/jqnatividad/qsv/commit/3f0ed2b314765a546e28b534d5e82bff892592c3
* `stats`: refactored modes compilation https://github.com/jqnatividad/qsv/commit/6e448b041a2c78b3ce1cc89aadaff4a8d1081472
* `stats`: simplify if condition https://github.com/jqnatividad/qsv/commit/ae7cc85afe1dc4c3f87cbefe3b14dc93b28d94e9
* `luau`: show luau version when invoking --version https://github.com/jqnatividad/qsv/commit/f7f9c4297fb3dea685b5d0f631932b6b2ca4a99a
* `excel`: add "sheet" suffix to end msg for readability https://github.com/jqnatividad/qsv/commit/ae3a8e31784a24c8492de76c5074e477cc474063
* cache `util::count_rows` result, so if a CSV without an index is queried, it caches the result and future calls to count_rows in the same session will be instantaneous https://github.com/jqnatividad/qsv/commit/e805dedf5674cfbc56d9948791419ac6fd51f2fd
* Bump console from 0.15.3 to 0.15.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/704
* Bump cached from 0.41.0 to 0.42.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/709
* Bump mlua from 0.8.6 to 0.8.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/712
* Bump qsv-stats from 0.5.2 to 0.6.0 with the new MAD statistic support
* cargo update bump dependencies - notably mimalloc from 0.1.32 to 0.1.34, luau0-src from 0.4.1_luau553 to 0.5.0_luau555, csvs_convert from 0.7.9 to 0.7.11 and regex from 1.7.0 to 1.7.1
* pin Rust nightly to 2023-01-08

### Fixed
* `tojsonl`: fix escaping of unicode string. Replace hand-rolled escape fn with built-in escape_default fn https://github.com/jqnatividad/qsv/pull/707. Fixes https://github.com/jqnatividad/qsv/issues/705
* `tojsonl`: more robust boolean inferencing https://github.com/jqnatividad/qsv/pull/710. Fixes https://github.com/jqnatividad/qsv/issues/708


## New Contributors
* @janriemer made their first contribution in https://github.com/jqnatividad/qsv/pull/711

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.81.0...0.82.0

## [0.81.0] - 2023-01-02

### Added
* `stats`: added range statistic https://github.com/jqnatividad/qsv/pull/691
* `stats`: added additional mode stats. For mode, added mode_count and mode_occurrences. Added "antimode" (opposite of mode - least frequently occurring non-zero value), antimode_count and antimode_occurrences. https://github.com/jqnatividad/qsv/pull/694
* qsv-dateparser now recognizes unix timestamp values with fractional seconds to nanosecond precision as dates. `stats`, `sniff`, `apply datefmt` and `schema`, which all use qsv-dateparser, now infer unix timestamps as dates - https://github.com/jqnatividad/qsv/commit/a29ff8ea255d5aed9992556a0a23ab76117c8340 https://github.com/jqnatividad/qsv/pull/702
> USAGE NOTE: As timestamps can be float or integer, and data type inferencing will guess dates last, preprocess timestamp columns with apply datefmt first to more date-like, non-timestamp formats, so they are recognized as dates by other qsv commands.

### Changed
* `apply`: document numtocurrency --comparand & --replacement behavior https://github.com/jqnatividad/qsv/commit/cc88fe921d8cdf7eedcb0008e16ebb5c46744f33
* `index`: explicitly flush buffers after creating index https://github.com/jqnatividad/qsv/commit/ee5d790af1cde73dfc57b028bf52fa88e83cdaa4
* `sample`: no longer requires an index to do percentage sampling https://github.com/jqnatividad/qsv/commit/45d4657713ebe2ae8388ce55f4cb1a733e727024
* `slice`: removed unneeded utf8 check https://github.com/jqnatividad/qsv/commit/5a199f4442bd025cec31309bee44ac71bacbdfaa
* `schema`: expand usage text regarding `--strict-dates` https://github.com/jqnatividad/qsv/commit/3d22829f3cf0441961e854555cd0c333bcb3ffb1 
* `stats`: date stats refactor. Date stats are returned in rfc3339 format. Dates are converted to timestamps with millisecond precision while calculating date stats. https://github.com/jqnatividad/qsv/pull/690 https://github.com/jqnatividad/qsv/commit/e7c297795ff5e82cf1dc242090be11ecced6da9a
* filter out variance/stddev in tests as float precision issues are causing flaky CI tests  https://github.com/jqnatividad/qsv/pull/696
* Bump qsv-dateparser from 0.4.4 to 0.6.0
* Bump qsv-stats from 0.4.6 to 0.5.2
* Bump qsv-sniffer from 0.5.0 to 0.6.0
* Bump serde from 1.0.151 to 1.0.152 by @dependabot in https://github.com/jqnatividad/qsv/pull/692
* Bump csvs_convert from 0.7.7 to 0.7.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/693
* Bump once_cell from 0.16.0 to 0.17.0 https://github.com/jqnatividad/qsv/commit/d3ac2556c74e2ddd66dcee00e5e836d284b662a7
* Bump self-update from 0.32.0 to 0.34.0 https://github.com/jqnatividad/qsv/commit/5f95933f01e2e0c592b52d7424b6a832aafd3591
* Bump cpc from 1.8 to 1.9; set csvs_convert dependency to minor version https://github.com/jqnatividad/qsv/commit/ee9164810559f5496dfafba0e789b9cd84000a17
* applied select clippy recommendations
* deeplink to Cookbook from Table of Contents
* pin Rust nightly to 2023-01-01
* implementation comments on `stats`, `sample`, `sort` & Python distribution

### Fixed
* `stats`: prevent premature rounding, and make sum statistic use the same rounding method https://github.com/jqnatividad/qsv/commit/879214a1f3032f140f0207fe8807e1bb641110d7 https://github.com/jqnatividad/qsv/commit/1a1362031de8973b623598748bea4bc5fc6e08d3
* fix autoindex so we return the index path properly https://github.com/jqnatividad/qsv/commit/d3ce6a3918683d66bf0f3246c7d6e8518eead392
* `fetch` & `fetchpost`: corrected typo https://github.com/jqnatividad/qsv/commit/684036bbc237d5b80ea060f9ee8b8d46c1a2ad88


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.80.0...0.81.0

## [0.80.0] - 2022-12-23

### Added
* new `to` command. Converts CSVs "to" PostgreSQL, SQLite, XLSX, Parquet and Data Package by @kindly in https://github.com/jqnatividad/qsv/pull/656
* `apply`: add numtocurrency operation https://github.com/jqnatividad/qsv/pull/670
* `sort`: add --ignore-case option https://github.com/jqnatividad/qsv/pull/673
* `stats`: now computes descriptive statistics for dates as well https://github.com/jqnatividad/qsv/pull/684
* added --updatenow option, resolves https://github.com/jqnatividad/qsv/issues/661 https://github.com/jqnatividad/qsv/pull/662
* replace footnotes in Available Commands list with emojis :smile:


### Changed
* `apply` & `applydp`: expose --batch size option https://github.com/jqnatividad/qsv/pull/679
* `validate`: add last valid row to validation error https://github.com/jqnatividad/qsv/commit/7680011a2fcc459aa621414122ecaa869e98ae83
* `input`: add last valid row to error message https://github.com/jqnatividad/qsv/commit/492e51f85ab5a0637c201d7020d7ac2fdb72be96
* upgrade to csvs-convert 0.7.5 by @kindly in https://github.com/jqnatividad/qsv/pull/668
* Bump serial_test from 0.9.0 to 0.10.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/671
* Bump csvs_convert from 0.7.5 to 0.7.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/674
* Bump num_cpus from 1.14.0 to 1.15.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/678
* Bump robinraju/release-downloader from 1.6 to 1.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/677
* Bump actions/stale from 6 to 7 by @dependabot in https://github.com/jqnatividad/qsv/pull/676
* Bump actions/setup-python from 4.3.1 to 4.4.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/683
* added concurrency check to CI tests so that redundant CI test are canceled when new ones are launched
* instead of saying "descriptive statistics", use more understandable "summary statistics"
* changed publishing workflows to enable `to` feature for applicable target platforms
* cargo update bump dependencies, notably qsv-stats from 0.4.5 to 0.4.6 and qsv_currency from 0.5.0 to 0.6.0
* pin Rust nightly to 2022-12-22

### Fixed
* `stats`: fix leading zero handling https://github.com/jqnatividad/qsv/pull/667
* `apply`: fix currencytonum bug https://github.com/jqnatividad/qsv/pull/669


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.79.0...0.80.0


## [0.79.0] - 2022-12-16
### Added
* `safenames`: add --reserved option, allowing user to specify additional "unsafe" names https://github.com/jqnatividad/qsv/pull/657
* `safenames`: add --prefix option https://github.com/jqnatividad/qsv/pull/658
* `fetch` & `fetchpost`: added simple retry backoff multiplier - https://github.com/jqnatividad/qsv/commit/e343398ddd9c804237e73bbc652cc9e51c657b78

### Changed
* `excel`: refactored --metadata processing; added more debug messages; minor perf tweaks https://github.com/jqnatividad/qsv/commit/f137bab42f81518acd3ef825cd223b9970d70b02
* set MSRV to Rust 1.6.6
* cargo update bump several dependencies, notably qsv-dateparser
* pin Rust nightly to 2022-12-15

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.2...0.79.0


## [0.78.2] - 2022-12-13

### Changed
* cargo update bump paste 1.0.9 to 1.0.10
* pin Rust nightly to 2022-12-12

### Removed
* `excel`: remove --safenames option. If you need safenames, use the `safenames` command https://github.com/jqnatividad/qsv/commit/e5da73bcc64ef3a8c66c611fd6247fa331117544


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.1...0.78.2

## [0.78.1] - 2022-12-12

### Changed
* `qsvdp`: `apply` now available in qsvdp as`applydp` - removing the geocode and calconv subcommands, and removing all operations that require third-party crates EXCEPT dynfmt and datefmt which is needed for Datapusher+ https://github.com/jqnatividad/qsv/pull/652
* `excel`: fine-tune --metadata processing https://github.com/jqnatividad/qsv/commit/09530d4f65b06060d24b7ed3948aeab25b2aa7c8
* bump serde from 1.0.149 to 1.0.150
* `qsvdp` in now included in CI tests


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.78.0...0.78.1

## [0.78.0] - 2022-12-11

### Added
* `stats`: added leading zero handling when inferring types (e.g. zipcodes like "07094" are strings not integers) https://github.com/jqnatividad/qsv/pull/648
* `stats`: added --typesonly option, which infers only data types with date inferencing enabled for all columns  https://github.com/jqnatividad/qsv/pull/650
* `stats`: added underflow handing to sum statistic https://github.com/jqnatividad/qsv/commit/1b5e5451f929ad1c7dc5fb7f17b2a3261809ab05
* `excel`: expanded --metadata functionality, with the option to return workbook metadata as JSON as well https://github.com/jqnatividad/qsv/pull/651
* added platform-specific README for prebuilt zip archives https://github.com/jqnatividad/qsv/commit/15e247e523dbc22a50ebff1b15d7d0c4eb668bd5

### Changed
* `safenames`: improved usage text
* `stats`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/88be38b542fc61470a7b0331e7be3a3cad62a7bb and https://github.com/jqnatividad/qsv/commit/8aa58c5ad733116d246e171bcea622c1378b8e48
* `join`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/92d41910077148f769ccf2c8a283be2c30d68bbf
* `exclude`: minor performance tweaks https://github.com/jqnatividad/qsv/commit/f3cc0ac29c5f3e6cec5a08d3aac3371d32b5eb0f
* `sniff`: minor performance tweak https://github.com/jqnatividad/qsv/commit/d2a4676fcb5189fc9232538e68854cfcf4ef808b
* `sortcheck`: minor performance tweak https://github.com/jqnatividad/qsv/commit/83c22ae5a623a8b0740f7024aac9448ee809eabd
* switch GitHub Actions to use ubuntu-20.04 so as not to link to too new glibc libraries, preventing older distros from running the linux-gnu prebuilts.
* switch GitHub Actions to use macos-12 to minimize flaky CI tests
* expanded `qsvdp` description in README
* Bump actions/setup-python from 4.3.0 to 4.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/645
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-12-10


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.77.0...0.78.0

## [0.77.0] - 2022-12-08

### Added
* `safenames`: added Verbose JSON options https://github.com/jqnatividad/qsv/pull/644

### Changed
* `py` & `luau`: improved usage text
* opt-in self-update in https://github.com/jqnatividad/qsv/pull/640 and https://github.com/jqnatividad/qsv/pull/641
* Create README in prebuilt zip archive with platform specific notes https://github.com/jqnatividad/qsv/pull/642
* Simplify python map_datetime test so it works on older Python versions https://github.com/jqnatividad/qsv/commit/e85e4e7bf9bf379f8478b066a9f6dea21afbf0e8
* include date.lua in qsv package so `cargo install` works https://github.com/jqnatividad/qsv/commit/11a0ff8edc5405afd9cc6637de026bf2138a7df0
* Bump data-encoding from 2.3.2 to 2.3.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/638
* cargo update bump several dependencies
* pin Rust nightly to 2022-12-07

### Fixed:
* `safenames`: fixed calculation of unsafe headers as it was dupe-counting some unsafe headers - https://github.com/jqnatividad/qsv/pull/644


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.3...0.77.0

## [0.76.3] - 2022-12-05

### Changed
* cargo update bump serde from 1.0.148 to 1.0.149
* simplify python datetime test so it runs on Python 3.6 and above

### Fixed
* reverted `not_luau_compatible` introduced in 0.76.2 and 0.76.3. Adjusted Github Action publish workflow instead to properly build `luau` in qsvdp when the platform supports it.

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.2...0.76.3

## [0.76.2] - 2022-12-04

### Fixed
* tweak `not_luau_compatible` feature so we can more easily disable `luau` feature when cross-compiling for some platforms where we cannot properly build luau.

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.1...0.76.2

## [0.76.1] - 2022-12-04

### Fixed
* added `not_luau_compatible` feature so we can more easily disable `luau` feature when cross-compiling for some platforms where we cannot properly build luau.

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.76.0...0.76.1

## [0.76.0] - 2022-12-04

### Added
* `qsvdp`: add `luau` in anticipation of Datapusher+ optional preprocessing https://github.com/jqnatividad/qsv/pull/634
* `luau`: added ability to load libraries using "require"; preload LuaDate library https://github.com/jqnatividad/qsv/pull/633
* `luau`: added more extensive debug logging support, adding _idx to debug log messages; trace log level support showing global vars and record values when an error occurs https://github.com/jqnatividad/qsv/pull/636 and https://github.com/jqnatividad/qsv/pull/637

### Changed
* `py` and `luau`: when errors encountered, return non-zero exit code, along with error count to stderr https://github.com/jqnatividad/qsv/pull/631
* `safenames` and `excel`: Unsafe empty column/header names are replaced with "\_blank" instead of "\_" https://github.com/jqnatividad/qsv/pull/632
* `frequency`: replace foreach iterator with regular for; remove unneeded assert https://github.com/jqnatividad/qsv/commit/74eb321defbf294675872a7dd891e8a7aedd31f1
* bumped qsv-stats from 0.4.1 to 0.4.5 - fixing sum rounding and variance precision errors. 
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-12-03

### Fixed
* `stats`: fix sum rounding and variance precision errors https://github.com/jqnatividad/qsv/pull/635

NOTE: Not published on crates.io due to problems creating prebuilt binaries

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.75.0...0.76.0

## [0.75.0] - 2022-12-01

### Added:
* `py`: added python datetime module by default in https://github.com/jqnatividad/qsv/pull/629
* `qsvdp` ([Datapusher+](https://github.com/dathere/datapusher-plus) optimized binary variant): added self-update. However, unlike `qsv` and `qsvlite` binary variants, `qsvdp` will not automatically prompt for a self-update, and will only inform the user if there is a new release. The user will need to invoke the `--update` option explicitly. https://github.com/jqnatividad/qsv/pull/622

### Changed:
* `stats`: Speedup type checking by @kindly in https://github.com/jqnatividad/qsv/pull/625
* `validate`: Added a useful note about validate output by @aborruso in https://github.com/jqnatividad/qsv/pull/624
* `luau`: Now precompiles all scripts, including the `--prologue` & `--epilogue` scripts, into bytecode https://github.com/jqnatividad/qsv/commit/e97c2caf81316bcf655875a9bee4c78dac5a8b70
* `frequency`: remove unsafe from_utf8_unchecked https://github.com/jqnatividad/qsv/commit/16642e8ee3364309c1a774142976f6207ba5c594
* More robust autoindexing in https://github.com/jqnatividad/qsv/pull/623
* minor clippy performance tweaks to [rust-csv fork](https://github.com/jqnatividad/rust-csv/tree/perf-tweaks)
* Bump serde from 1.0.147 to 1.0.148 by @dependabot in https://github.com/jqnatividad/qsv/pull/620
* cargo update bump several indirect dependencies
* improved README; use :sparkle: to indicate commands behind a feature flag
* pin Rust nightly to 2022-11-30

## New Contributors
* @aborruso made their first contribution in https://github.com/jqnatividad/qsv/pull/624
* @kindly made their first contribution in https://github.com/jqnatividad/qsv/pull/625

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.74.0...0.75.0

## [0.74.0] - 2022-11-27

### Added:
* `safenames`: added --verify and --verbose modes in https://github.com/jqnatividad/qsv/pull/610 and https://github.com/jqnatividad/qsv/pull/615

### Changed:
* `excel`: align --safenames option to `safenames` command in https://github.com/jqnatividad/qsv/pull/611 and https://github.com/jqnatividad/qsv/pull/616
* `luau`: Now precompiles main script to bytecode; now allow loading luau script from file for main, prologue and epilogue scripts in https://github.com/jqnatividad/qsv/pull/619
* `sniff`: increase default sample size from 100 to 1000 in https://github.com/jqnatividad/qsv/commit/40d52cf0c67e39d645a1c76a26ae234999317b0b
* `validate`: applied various optimizations in https://github.com/jqnatividad/qsv/commit/bfed127f28c4ccf6e9a18a5998588396594831d2 and https://github.com/jqnatividad/qsv/commit/06c109a0335326f57d903211334b4f2fb1ab7ccc
* updated Github Actions workflows to reflect removal of luajit feature
* Bump sysinfo from 0.26.7 to 0.26.8 by @dependabot in https://github.com/jqnatividad/qsv/pull/614
* Bump rust_decimal from 1.26.1 to 1.27.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/617
* cargo bump update several indirect dependencies
* applied various clippy recommendations
* pin Rust nightly to 2022-11-25

### Removed:
* `luajit`: removed as its been deprecated by optimized `luau` command which now support precompiling to bytecode, largely obviating the main feature of LuaJIT - Just-in-Time compilation in https://github.com/jqnatividad/qsv/pull/619

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.2...0.74.0

## [0.73.2] - 2022-11-22

### Changed:
* Link to tests as examples from usage text in https://github.com/jqnatividad/qsv/pull/608
* Bump serde_json from 1.0.88 to 1.0.89 by @dependabot in https://github.com/jqnatividad/qsv/pull/607
* cargo update bump to get latest crossbeam crates to replace yanked crates https://github.com/jqnatividad/qsv/commit/5108a87b0f5e2d5a7cfef3f60f4cd6b3659bce7d 

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.1...0.73.2

## [0.73.1] - 2022-11-21
### Changed:
* rename `safename` command to `safenames` for consistency
* cargo update bump indirect dependencies

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.73.0...0.73.1

## [0.73.0] - 2022-11-21
### Added
* `safenames`: new command to modify header names to db-safe names in https://github.com/jqnatividad/qsv/pull/606
* `apply`: added `censor-count` operation in https://github.com/jqnatividad/qsv/pull/599
* `apply`: added `escape` operation in https://github.com/jqnatividad/qsv/pull/600
* `excel`: added `--safe-names` option in https://github.com/jqnatividad/qsv/pull/598

### Changed
* `apply`: refactored to use enums instead of strings for operations in https://github.com/jqnatividad/qsv/pull/601
* `fetch` & `fetchpost`: --http-header -H shortcut in https://github.com/jqnatividad/qsv/pull/596
* `excel`: smarter date parsing for XLSX files; rename --safe-column-names to --safe-names in https://github.com/jqnatividad/qsv/pull/603
* Smarter safe names in https://github.com/jqnatividad/qsv/pull/605
* Bump uuid from 1.2.1 to 1.2.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/594
* Bump mimalloc from 0.1.31 to 0.1.32 by @dependabot in https://github.com/jqnatividad/qsv/pull/595
* Bump censor from 0.2.0 to 0.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/597
* Bump Swatinem/rust-cache from 1 to 2 by @dependabot in https://github.com/jqnatividad/qsv/pull/602
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-11-19

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.72.0...0.73.0

## [0.72.0] - 2022-11-14

### Added
* `apply`: added --keep-zero-time option in https://github.com/jqnatividad/qsv/pull/590
* `lua` and `luajit`: added  --prologue & --epilogue options in https://github.com/jqnatividad/qsv/pull/592
* `luau` & `luajit`: switched from Lua to Luau; added special vars _idx and _rowcount in https://github.com/jqnatividad/qsv/pull/593
* `luau` & `luajit`: return exitcode 1 if interpretation error is encountered https://github.com/jqnatividad/qsv/commit/655041b86c86c3ce0024d1e20599c98dfab28658

### Changed
* `schema` & `validate`: expand description/usage text in https://github.com/jqnatividad/qsv/commit/60dfebc9f401045467417b2065481b657ff82c92
* `validate`: return exitcode 0 if CSV is valid; exitcode 1 otherwise in https://github.com/jqnatividad/qsv/pull/591
* Bump hashbrown from 0.12.3 to 0.13.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/586
* cargo bump update indirect dependencies, notably chrono from 0.4.22 to 0.4.23
* Shortened command descriptions for `luau` & `luajit` and added salient notes to new interpreter section
* adjust GitHub Actions workflows to use `luau` feature
* pin Rust nightly to 2022-11-14


**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.71.1...0.72.0

## [0.71.1] - 2022-11-09

### Changed
* `python` feature is no longer enabled in the prebuilt binaries to avoid distribution issues and qsv panicking if the exact python version it was statically linked against
is not available. If you require the `python` feature, you'll have to install/build for source.

### Fixed
* whirlwind tour: `join`'s `--no-case` option has been replaced by `--ignore-case` by @alperyilmaz in https://github.com/jqnatividad/qsv/pull/585

## New Contributors
* @alperyilmaz made their first contribution in https://github.com/jqnatividad/qsv/pull/585

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.71.0...0.71.1

## [0.71.0] - 2022-11-08
### Added
* `apply`: new `encode` and  `decode` operations in https://github.com/jqnatividad/qsv/pull/569
* `apply`: add ability to show confidence to whatlang language detection. in https://github.com/jqnatividad/qsv/pull/579
* `count`: add --width option in https://github.com/jqnatividad/qsv/pull/582
* `fetch` & `fetchpost`: Added --user_agent option by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/565 and https://github.com/jqnatividad/qsv/commit/f59bd8591079f22df3c40e5f036c5e2ff83e77f8
* Documented Homebrew installer :rocket: created by @FnControlOption

### Changed
* `apply`: refactor operations validation in https://github.com/jqnatividad/qsv/pull/564 and https://github.com/jqnatividad/qsv/commit/f83ec6f7e7fa7bed9bcc2b5e55516a61e5154b52
* `sortcheck`: expand usage text and use fail_clierror macro https://github.com/jqnatividad/qsv/commit/8513b53eaac594d20106b3f77f73f3d1b63e227d
* `stats`: minor refactoring https://github.com/jqnatividad/qsv/commit/38795134e3ed66bf0816eeee2a68aa9b557c4908
* `tojsonl`: it does "smart" conversion of CSV to JSONL https://github.com/jqnatividad/qsv/commit/af98290bf1803ae5ab3e01df5f20f5b007912e02
* `validate`: also show --progressbar when doing schemaless validation https://github.com/jqnatividad/qsv/commit/aae550aa0b1042e205689ae40d19c0532e7ae584
* only show enabled commands in command list in https://github.com/jqnatividad/qsv/pull/583
* Updated the benchmark script by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/581
* Switch back to using num_cpus for detecting parallelism https://github.com/jqnatividad/qsv/commit/b7dbed88f7d931e03a835ca4a929328c2c4a34b6
* qsv now links against Python 3.11 for the `py` command in https://github.com/jqnatividad/qsv/pull/576
* Bump robinraju/release-downloader from 1.5 to 1.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/568
* Bump newline-converter from 0.2.0 to 0.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/567
* Bump sysinfo from 0.26.5 to 0.26.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/566 and https://github.com/jqnatividad/qsv/pull/572
* Bump ahash from 0.8.0 to 0.8.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/575
* Bump flexi_logger from 0.24.0 to 0.24.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/574
* Bump pyo3 from 0.17.2 to 0.17.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/573
* Bump jql from 5.1.1 to 5.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/577
* Bump num_cpus from 1.13.1 to 1.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/580
* Bump mimalloc from 0.1.30 to 0.1.31
* Bump indicatif from 0.17.1 to 0.17.2
* cargo update bump several indirect dependencies
* updated rustfmt.toml with comment and string formatting options
* bump MSRV to 1.65.0
* pin Rust Nightly to 2022-11-07

**Full Changelog**: https://github.com/jqnatividad/qsv/compare/0.70.0...0.71.0

## [0.70.0] - 2022-10-24

### Added
* `apply`: additional operations - `squeeze0`, `strip_prefix` and `strip_suffix` https://github.com/jqnatividad/qsv/pull/518 & https://github.com/jqnatividad/qsv/pull/519
* `apply`: add `calcconv` subcommand, which parses & evaluate math expressions, with support for units & conversions. https://github.com/jqnatividad/qsv/pull/560

### Changed
* `search` & `searchset`: make match count optional https://github.com/jqnatividad/qsv/pull/526
* `jsonl`: remove panic and do proper error handling; add  --ignore-errors option https://github.com/jqnatividad/qsv/pull/531
* `py`: py command does not do aggregations (reduce) operations https://github.com/jqnatividad/qsv/pull/548
* `lua` & `luajit` can do aggregations across CSV rows and `py` cannot https://github.com/jqnatividad/qsv/pull/549
* `py`: add more complex f-string formatting example https://github.com/jqnatividad/qsv/pull/556
* Standardize ignore case option https://github.com/jqnatividad/qsv/pull/535
* Use rustfmt nightly to take advantage of advanced features like StdExternalCrate https://github.com/jqnatividad/qsv/pull/514 & https://github.com/jqnatividad/qsv/pull/517
* Update benchmark-basic.sh by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/542
* Use fail macros more consistently https://github.com/jqnatividad/qsv/pull/545
* Use Redis `ahash` feature for performance
* Added wix file for future Windows Installer by @minhajuddin2510 in https://github.com/jqnatividad/qsv/pull/546
* Bump console from 0.15.1 to 0.15.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/512
* Bump pyo3 from 0.17.1 to 0.17.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/522
* Bump jql from 5.0.2 to 5.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/521
* Bump titlecase from 2.2.0 to 2.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/534
* Bump itoa from 1.0.3 to 1.0.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/533
* Bump sysinfo from 0.26.4 to 0.26.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/554
* Bump mlua from 0.8.3 to 0.8.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/540
* Bump uuid from 1.1.2 to 1.2.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/539
* Bump flexi_logger from 0.23.3 to 0.24.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/538
* Bump serde_json from 1.0.85 to 1.0.86 by @dependabot in https://github.com/jqnatividad/qsv/pull/537
* Bump actions/setup-python from 4.2.0 to 4.3.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/541
* Bump filetime from 0.2.17 to 0.2.18 by @dependabot in https://github.com/jqnatividad/qsv/pull/559
* Bump redis from 0.21.6 to 0.22.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/555
* Bump cached from 0.39.0 to 0.40.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/561
* Bump whatlang from 0.16.1 to 0.16.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/562
* cargo update bump several indirect dependencies
* Pin Rust nightly to 2022-10-22


### Fixed
* `excel`: xls float temporary workaround for #516 that was ultimately fixed in PR 558 https://github.com/jqnatividad/qsv/pull/520
* `tojsonl`: escape newlines and double quotes. Fixes #552 https://github.com/jqnatividad/qsv/pull/553
* `tojsonl`: better error handling; when checking stdin for utf8, make sure its not empty. Fixes #536 https://github.com/jqnatividad/qsv/pull/536

### Removed
* `excel`: removed xls float workaround now that calamine crate has been fixed. Fixes #516 removing need for PR 520 workaround. https://github.com/jqnatividad/qsv/pull/558
* removed obsolete Rust Nightly workflow https://github.com/jqnatividad/qsv/commit/2a99318242040300130c323dc3e7df504a6e3b2e


## New Contributors
* @minhajuddin2510 made their first contribution in https://github.com/jqnatividad/qsv/pull/542

## [0.69.0] - 2022-09-28

### Added
* `luajit`: new command using LuaJIT, which is much faster than Lua https://github.com/jqnatividad/qsv/pull/500

### Changed
* `python`: tweaks. Expanded usage text. Only show python version when logging is on.  https://github.com/jqnatividad/qsv/pull/507
* `fetch` & `fetchpost`: apply clippy recommendation https://github.com/jqnatividad/qsv/commit/dd7220bce2811d9e8248c379af5d5c38da3b02d5
* `excel`: use `winfo!` macro https://github.com/jqnatividad/qsv/commit/7211ff214a58394d68c8c7484e8ef4505d75b482
* Removed anyhow dependency https://github.com/jqnatividad/qsv/pull/508
* Bump actions/stale from 5 to 6 by @dependabot in https://github.com/jqnatividad/qsv/pull/505
* Bump sysinfo from 0.26.3 to 0.26.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/510
* Cargo update bump several indirect dependencies
* include Python 3.10 shared libraries when publishing for select platforms
* bump MSRV to Rust 1.64.0
* Pin Rust nightly to 2022-09-26

### Fixed
* `python`: corrected erroneous --helper example. Included hashhelper.py example.
* `extsort`: fixed --help bug (https://github.com/jqnatividad/qsv/issues/506)

## [0.68.0] - 2022-09-16
### Changed
* Simplify python support. For prebuilt binaries, Python 3.10 is now required and the python 3.10 shared libraries are bundled for select platforms.
If you require an earlier version of Python (3.6 and up), you'll have to install/compile from source. https://github.com/jqnatividad/qsv/pull/492
* Smarter self update. --update can still be explicitly invoked even when self-update feature has been disabled. Further, if you compiled qsv from source,
self-update will only notify you of new releases, instead of proceeding with self-update. https://github.com/jqnatividad/qsv/pull/490 and https://github.com/jqnatividad/qsv/pull/493
* `lua`: switch from Lua 5.4 to LuaJIT 2.1, primarily for performance https://github.com/jqnatividad/qsv/pull/495
* `lua`: when filtering using floats, "0.0" is false
* `join`: removed unneeded utf8 check
* `search`: simplify regex_unicode check
* `fetch` & `fetchpost`: optimize imports; remove unneeded utf8 check
* Bump anyhow from 1.0.64 to 1.0.65 by @dependabot in https://github.com/jqnatividad/qsv/pull/498
* Bump self_update from 0.31.0 to 0.32.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/499
* add additional copyright holder to MIT License
* Improved publishing workflow for prebuilt binaries
* cargo update bumped several dependencies
* pin Rust nightly to 2022-09-14

### Fixed
* fix typos by @kianmeng in https://github.com/jqnatividad/qsv/pull/491
* `python`: better error handling. When mapping/filtering, python expression errors no longer cause a panic, but instead fail to map/filter as expected (when mapping, "\<ERROR\>" is returned, when filtering, the filter is not applied), and continue processing. Also, other errors are properly propagated instead of panicking. https://github.com/jqnatividad/qsv/pull/496
* `lua`: better error handling. When mapping/filtering, Lua errors no longer cause a panic, but instead fail to map/filter as expected (when mapping, "\<ERROR\>" is returned, when filtering, the filter is not applied), and continue processing. https://github.com/jqnatividad/qsv/pull/497

## [0.67.0] - 2022-09-09
### Added
* added `self_update` feature, so users can build qsv without self-update engine https://github.com/jqnatividad/qsv/pull/483 and https://github.com/jqnatividad/qsv/pull/484

### Changed
* `search` & `searchset`: --quick option returns first match row to stderr https://github.com/jqnatividad/qsv/pull/475
* `python`: make --batch size configurable https://github.com/jqnatividad/qsv/pull/485
* `stats`: added more implementation comments; standardize string creation
* `replace`: add conditional compilation to eliminate dead_code warning
* `lua`: when filtering, non-zero integers are true
* refactored `workdir.rs` test helpers
* refactored `util:init_logger()` to log command-line arguments
* Bump url from 2.3.0 to 2.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/489
* Bump anyhow from 1.0.63 to 1.0.64 by @dependabot in https://github.com/jqnatividad/qsv/pull/478
* Bump sysinfo from 0.26.1 to 0.26.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/477
* Bump robinraju/release-downloader from 1.4 to 1.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/481
* cargo update bump indirect dependencies
* pin Rust nightly to 2022-09-07

## [0.66.0] - 2022-09-01

### Added
* `apply`: added Multi-column subcommands by @udsamani in https://github.com/jqnatividad/qsv/pull/462
* `stats`:  added --round option https://github.com/jqnatividad/qsv/pull/474
* created `fail_format!` macro for more concise error handling in https://github.com/jqnatividad/qsv/pull/471

### Changed
* Move command usage text to beginning of cmd source code, so we don't need to move around deeplinks to usage texts from README https://github.com/jqnatividad/qsv/pull/467
* Optimize conditional compilation of various qsv binary variants, removing dead code https://github.com/jqnatividad/qsv/pull/473
* `fetch` & `fetchpost`: removed initial burst of requests, making the commands "friendlier" to rate-limited APIs
* `search`, `searchset` & `replace`: minor performance optimizations
* created dedicated rustfmt GitHub action workflow to ensure code is always rust formatted. Previously, rustfmt check was in Linux workflow.
* applied some clippy recommendations
* Bump actix-governor from 0.3.1 to 0.3.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/461
* cargo update bumped several dependencies
* pin Rust nightly to 2022-08-31
* set RUSTFLAGS to emit=asm when compiling pre-built binaries for performance
  see http://likebike.com/posts/How_To_Write_Fast_Rust_Code.html#emit-asm

### Fixed
* `extsort` code was being compiled for qsvdp even if it was not enabled
* bump sysinfo from 0.25.2 to 0.26.0, fixing segfault on Apple Silicon
* fixed qsvnp on Windows so it doesn't look for python shared libraries even if python is not enabled
* fixed CliError::Other so it returns bad exitcode (exitcode 1) instead of incorrect_usage (exit code 2)

## New Contributors
* @udsamani made their first contribution in https://github.com/jqnatividad/qsv/pull/462

## [0.65.0] - 2022-08-28

### Added
* Major refactoring of main variants - removing redundant code and moving them to a new module - clitypes.rs. Added custom exit codes. 
  Removed need to have --exitcode option in several commands as qsv now returns exit codes for ALL commands in a standard way. https://github.com/jqnatividad/qsv/pull/460
* Major refactoring of CI test helpers in workdir.rs

### Changed
* `py`: use python interning to amortize allocs https://github.com/jqnatividad/qsv/pull/457
* `search` & `searchset`: return num of matches to stderr; add --quick option; remove --exitcode option https://github.com/jqnatividad/qsv/pull/458
* `extsort`: improved error handling
* `fetch` & `fetchpost`: better --report option handling https://github.com/jqnatividad/qsv/pull/451
* `lua`: faster number to string conversion using itoa and ryu
* `replace`: removed --exitcode option
* `sortcheck`: --json options now always cause full scan of CSV
* `stats`: expanded usage text, explicitly listing stats that require loading the entire CSV into memory. Mentioned data type inferences are guaranteed.
* cargo update bumped several dependencies
* pin Rust nightly to 2022-08-27

### Fixed
* `py`: batched python processing refactor. Instead of using one GILpool for one session, `py` now processes in batches of 30,000 rows, releasing memory after each batch.  This resulted in memory consumption levelling out, instead of increasing to gigabytes of memory with very large files. As an added bonus, this made the `py` command ~30% faster in testing. :smile:  https://github.com/jqnatividad/qsv/pull/456

## [0.64.0] - 2022-08-23
### Added
* added `sortcheck` command https://github.com/jqnatividad/qsv/pull/445
* `replace`: added --exitcode and --progressbar options 

### Changed
* `apply`: improved usage text
* `excel`: replace --list-sheets option with expanded --metadata option https://github.com/jqnatividad/qsv/pull/448
* `sortcheck` improvements https://github.com/jqnatividad/qsv/pull/447
* `extsort`: improved error handling
* progressbar messages are now logged
* bump pyo3 from 0.16 to 0.17
* bump reqwest & redis "patches" further upstream
* cargo update bump several indirect dependencies
* pin Rust nightly to 2022-08-22

### Fixed
* `extsort`: fixed sysinfo segfault on Apple Silicon by pinning sysinfo to 0.25.2 https://github.com/jqnatividad/qsv/pull/446
* `tojsonl`: fixed panic with stdin input

## [0.63.2] - 2022-08-18
### Added
* `fetchpost`: added formdata to report https://github.com/jqnatividad/qsv/pull/434
* `search` & `searchset`: added Custom exit codes; --exitcode option https://github.com/jqnatividad/qsv/pull/439
* `search` & `searchset`: added --progressbar option
* progressbars are now optional by default; added QSV_PROGRESSBAR env var to override setting
* `search`, `searchset` & `replace`: added mem-limit options for regex-powered commands https://github.com/jqnatividad/qsv/pull/440
### Changed
* Bump jql from 4.0.7 to 5.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/436
* progressbars are now off by default, and are disabled with stdin input https://github.com/jqnatividad/qsv/pull/438
* `lua` & `py`: improved error-handling when loading script files
* `stats`: changed to using AtomicBool instead of OnceCell, use with_capacity in hot compute loop to minize allocs - hyperfine shows 18% perf increase with these changes
* self-update now gives a proper error message when GitHub is rate-limiting updates
* cargo update bump several dependencies
* document MSRV policy
* pin Rust Nightly to 2022-08-16

### Fixed
* fixed stdin input causing an error when progressbars are enabled https://github.com/jqnatividad/qsv/pull/438

## [0.62.0] - 2022-08-12
### Added
* `fetchpost`: new command that uses HTTP POST, as opposed to `fetch` - which uses HTTP GET ([difference between HTTP GET & POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) https://github.com/jqnatividad/qsv/pull/431
* Added `qsvnp` binary variant to prebuilt binaries - qsv with all the features EXCEPT python

### Changed
* `fetch`: refactor report parameter processing https://github.com/jqnatividad/qsv/pull/426
* Bump serde from 1.0.142 to 1.0.143 by @dependabot in https://github.com/jqnatividad/qsv/pull/423
* Bump ahash from 0.7.6 to 0.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/425
* Bump serial_test from 0.8.0 to 0.9.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/428
* Bump anyhow from 1.0.60 to 1.0.61 by @dependabot in https://github.com/jqnatividad/qsv/pull/427
* Bump sysinfo from 0.25.1 to 0.25.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/429
* Bump actix-governor from 0.3.0 to 0.3.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/430
* cargo update bump various indirect dependencies
* pin Rust nightly to 2022-08-11
* change MSRV to 1.63

### Fixed
* `excel`: fixed empty sheet handling https://github.com/jqnatividad/qsv/pull/422

## [0.61.4] - 2022-08-07
### Changed
* `py`: qsv uses the present working directory to find python shared library
* `py`: show python version info on startup
* publish qsvnp - another binary variant with all features except python
* bumped once_cell from 1.12 to 1.13
* use reqwest upstream with MSRV from 1.49 to 1.56; lazy_static to once_cell
* update calamine fork with chrono time feature disabled
* BetterTOML reformat cargo.toml
* pin Rust nightly to 2022-08-06

### Fixed
* `excel`: remove unneeded checkutf8 for writer

## [0.61.2] - 2022-08-04
### Changed
* `fetch`: Reformatted report so response is the last column; do not allow --timeout to be zero; progressbar refresh set at 5 times/sec; show name of generated report at the end. https://github.com/jqnatividad/qsv/pull/404
* `fetch`: report improvements. Remove `qsv_fetch_` column prefix in short report; change progressbar format to default characters https://github.com/jqnatividad/qsv/pull/406
* `excel`: make --sheet case-insensitive; better error-handling  https://github.com/jqnatividad/qsv/pull/416
* `py`: add detected python version to --version option
* Only do input utf8-encoding check for commands that need it. https://github.com/jqnatividad/qsv/pull/419
* Bump cached from 0.37.0 to 0.38.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/407
* Bump anyhow from 1.0.58 to 1.0.59 by @dependabot in https://github.com/jqnatividad/qsv/pull/408
* Bump serde from 1.0.140 to 1.0.141 by @dependabot in https://github.com/jqnatividad/qsv/pull/409
* Bump ryu from 1.0.10 to 1.0.11 by @dependabot in https://github.com/jqnatividad/qsv/pull/414
* Bump anyhow from 1.0.59 to 1.0.60 by @dependabot in https://github.com/jqnatividad/qsv/pull/413
* Bump mlua from 0.8.2 to 0.8.3 by @dependabot in https://github.com/jqnatividad/qsv/pull/412
* Bump actions/setup-python from 4.1.0 to 4.2.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/411
* Bump flexi_logger from 0.22.5 to 0.22.6 by @dependabot in https://github.com/jqnatividad/qsv/pull/417
* Bump indicatif from 0.16.2 to 0.17.0
* Bump chrono from 0.4.19 to 0.4.20
* Bump qsv-dateparser from 0.4.2 to 0.4.3
* pin Rust nightly to 2022-08-03

### Fixed
* fixed double progressbars https://github.com/jqnatividad/qsv/pull/405
* fix utf8 encoding check to resolve [#410](https://github.com/jqnatividad/qsv/issues/410) https://github.com/jqnatividad/qsv/pull/418

## [0.61.1] - 2022-07-30
### Added
* `fetch`: add elapsed time, retries to reports; add --max-retries option https://github.com/jqnatividad/qsv/pull/395

### Changed
* `lua`: better error messages https://github.com/jqnatividad/qsv/pull/399
* `python`: better error messages https://github.com/jqnatividad/qsv/pull/400
* `fetch`: improved error handling https://github.com/jqnatividad/qsv/pull/402
* `stats`: improve performance by using `unwrap_unchecked` in hot compute loop
* Bump indicatif from 0.16.2 to 0.17.0 https://github.com/jqnatividad/qsv/pull/403
* Bump mlua from 0.8.1 to 0.8.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/394
* Bump console from 0.15.0 to 0.15.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/398
* Bump grex from 1.3 to 1.4
* Cargo update bump various dependencies
* pin Rust nightly to 2022-07-29

### Fixed
* `excel`:  fixed --sheet option bounds checking https://github.com/jqnatividad/qsv/pull/401

## [0.60.0] - 2022-07-24
### Added
* `fetch`: add redis --flushdb option https://github.com/jqnatividad/qsv/pull/387
* `fetch`: add --report & --cache-error options. --report creates a separate report file, detailing the URL used,
the response, the HTTP status code, and if its a cache hit.
--cache-error is used to also cache errors - i.e. identical fetches will return the cached error. Otherwise, fetch will
request the URL again. https://github.com/jqnatividad/qsv/pull/393

### Changed
* `fetch`: fast defaults. Now tries to go as fast as possible, leveraging dynamic throttling (using RateLimit and Rety-After headers) 
but aborting after 100 errors. Also added a separate error progress bar. https://github.com/jqnatividad/qsv/pull/388
* Smarter `tojsonl`. Now scans CSV file and infers data types and uses the appropriate JSON data type https://github.com/jqnatividad/qsv/pull/389
* `tojsonl` is also multithreaded https://github.com/jqnatividad/qsv/pull/392
* `stats`: use unwrap_unchecked for even more performance https://github.com/jqnatividad/qsv/pull/390
* `fetch`: refactor dynamic throttling https://github.com/jqnatividad/qsv/pull/391
* Bump sysinfo from 0.24.6 to 0.24.7 by @dependabot in https://github.com/jqnatividad/qsv/pull/384
* cargo bump update several dependencies
* pin Rust nightly to 2022-07-23

### Fixed
* `fetch`: fix --http-header parsing bug https://github.com/jqnatividad/qsv/pull/386

## [0.59.0] - 2022-07-18
### Added
* added `tojsonl` command - CSV to JSONL https://github.com/jqnatividad/qsv/pull/380
* `excel`: additional --date-whitelist modes https://github.com/jqnatividad/qsv/pull/368
* `fetch`: added Redis connection pooling https://github.com/jqnatividad/qsv/pull/373

### Changed
* `python`: remove unneeded python3.dll generation https://github.com/jqnatividad/qsv/pull/379
* `stats`: minor performance tweaks
* `fetch`: minor performance tweaks - larger/faster in-mem cache
* Bump cached from 0.34.1 to 0.37.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/367 and https://github.com/jqnatividad/qsv/pull/381
* Bump regex from 1.5.6 to 1.6.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/369
* Bump reverse_geocoder from 3.0.0 to 3.0.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/377
* Bump actions/setup-python from 4.0.0 to 4.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/376
* Bump serde from 1.0.138 to 1.0.139 by @dependabot in https://github.com/jqnatividad/qsv/pull/374
* cargo update bump several dependencies
* larger logfiles (from 1mb to 10mb) before rotating
* apply select clippy recommendations
* pin Rust nightly to 2022-07-13

### Fixed
* Use option_env! macro to trap errors https://github.com/jqnatividad/qsv/pull/378

## [0.58.2] - 2022-07-02
### Changed
* Pin Rust nightly to 2022-07-02

### Fixed
* fixed redis dev-dependency which mistakenly added a non-existent ahash feature. This prevented publishing of qsv 0.58.1 to crates.io.

## [0.58.1] - 2022-07-02
### Changed
* Universal clippy handling. Added allow clippy hint section in main for clippy lints we allow/ignore, and added exceptions as needed throughout the codebase.
This means clippy, even in pedantic/nursery/perf mode will have no warnings. https://github.com/jqnatividad/qsv/pull/365
* reqwest deflate compression support https://github.com/jqnatividad/qsv/pull/366
* `fetch`: expanded --http-header explanation/example
* `fetch`: refactored --timeout processing https://github.com/jqnatividad/qsv/commit/3454ed068f0f243473a0f66520f90f55ece4bf49
* `fetch`: prioritized ACCEPT-ENCODING to prioritize brotli first, gzip second, and deflate last for compression https://github.com/jqnatividad/qsv/commit/c540d22b630df424a8516bb07af9bbf80150d67b
* updated patched crates, particularly our rust-csv fork with more clippy recommendations applied
* cargo update bump actix-http from 3.2.0 to 3.2.1

### Fixed
* `excel`: fixed docopt usage text which prevents --help from working
* `extsort`: better parsing/error-handling, instead of generic panic when no input/output is specified. This also allows --help to be displayed.

## [0.58.0] - 2022-07-02
### Added
* `excel`: add --list-sheets option https://github.com/jqnatividad/qsv/pull/364
* `fetch`: added 0 option to --rate-limit to go as fast as possible.  
**CAUTION:** Only use this with APIs that have [RateLimit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html) headers so qsv can automatically down-throttle as required. Otherwise, the fetch job will look like a Denial-of-Service attack. https://github.com/jqnatividad/qsv/commit/e4ece60aea3720b872119ca7a8ad3666dad033e7
* `fetch`: added --max-errors option. Maximum number of errors before aborting

### Changed
* progress bars now display per_sec throughput while running jobs, not just at the end of a job
* `fetch`: for long-running fetch jobs, the progress bar will update at least every three seconds, so it doesn't look like the job is frozen/stuck.
* `fetch`: added additional verbiage to usage text on how to pass multiple key-value pairs to the HTTP header
* `fetch`: made RateLimit jitters (required to avoid [thundering herd](https://en.wikipedia.org/wiki/Thundering_herd_problem) issues as per the [RateLimit spec](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html#resource-exhaustion-and-clock-skew)) shorter, as they were too long.
* pin Rust nightly to 2022-07-01
* applied various clippy recommendations
* bumped serde from 1.0.137 to 1.0.138
* added stale warning to benchmarks. The benchmarks have not been updated since qsv 0.20.0.
* cargo update bumped several other dependencies

### Fixed
* remove unneeded sleep pause before fetch ratelimit test

## [0.57.1] - 2022-06-31
### Changed
* `fetch`: higher default settings which makes fetch much faster

## [0.57.0] - 2022-06-30
### Added
* `excel`: date support https://github.com/jqnatividad/qsv/pull/357
* added hardware survey reminiscent of [Steam's Hardware Survey](https://store.steampowered.com/hwsurvey). Only sent when checking for updates with no personally identifiable information. https://github.com/jqnatividad/qsv/pull/358
* `fetch`: ensure URLs are properly encoded https://github.com/jqnatividad/qsv/pull/359

### Changed
* Bump jql from 4.0.4 to 4.0.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/356
* cargo bump update several dependencies
* change MSRV to Rust 1.62.0
* pin Rust Nightly to 2022-06-29

### Fixed
* `fetch`: is single-threaded again. It turns out it was more complicated than I hoped. Will revisit making it multi-threaded once I sort out the sync issues.

## [0.56.0] - 2022-06-20
### Added
* `fetch` is now multithreaded! 🚀🚀🚀 - with threadsafe memoized caching, dynamic throttling & http2 adaptive flow control https://github.com/jqnatividad/qsv/pull/354

### Changed
* `fetch`: do more expensive ops behind cache https://github.com/jqnatividad/qsv/pull/355
* applied BetterTOML formatting to Cargo.toml
* `exclude`, `flatten` & `join`: applied clippy recommendation for borrow_deref_ref https://github.com/jqnatividad/qsv/commit/bf1ac90185947a6d923613f17c4af616631dc149
* `utils`: minor cleanup of version fn https://github.com/jqnatividad/qsv/commit/217702b51785f51d6924608a5122c405ff384fef
* `validate`: perf tweak - use collect_into_vec to reduce allocations
* `apply`: perf tweak - use collect_into_vec to reduce allocations
* removed `thiserror` dependency
* pin Rust Nightly to 2022-06-19
* Bump robinraju/release-downloader from 1.3 to 1.4 by @dependabot in https://github.com/jqnatividad/qsv/pull/351
* Bump crossbeam-channel from 0.5.4 to 0.5.5 by @dependabot in https://github.com/jqnatividad/qsv/pull/352
* Bump redis patch
* cargo update bump several other dependencies

### Fixed
* `fetch`: better error handling https://github.com/jqnatividad/qsv/pull/353


###
## [0.55.5] - 2022-06-16
### Changed
* `fetch`: performance tweaks https://github.com/jqnatividad/qsv/pull/350
* Bump titlecase from 1.1.0 to 2.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/349
* Bump sysinfo from 0.24.3 to 0.24.4

### Fixed
* `fetch`: convert non-persistent cache from an Unbound cache to a Sized LRU cache, 
so we don't run out of memory if the file being processed is very large and cache hits are low.
https://github.com/jqnatividad/qsv/commit/4349fc9389a32c0d9544be824d1f42b1af65974d

## [0.55.4] - 2022-06-15
### Changed
* `fetch`: preemptively throttle down before we hit the ratelimit quota

## [0.55.3] - 2022-06-15
### Added
* `fetch`: add "dynamic throttling". If response header has [rate-limit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html) or [retry-after](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After) fields, fetch will dynamically throttle itself as needed. https://github.com/jqnatividad/qsv/pull/348

### Changed
* cargo update bump dependencies
* Pin Rust nightly to 2022-06-14

## [0.55.2] - 2022-06-14
### Changed
* `fetch`: more robust/consistent error handling https://github.com/jqnatividad/qsv/pull/347
* removed reqwest 0.11.10 patch and used reqwest 0.11.11
* Pin Rust nightly to 2022-06-13

## [0.55.1] - 2022-06-13
### Changed
* Pin Rust nightly to 2022-06-12

### Fixed
* `fetch`: fix invalid jsonl response https://github.com/jqnatividad/qsv/pull/346

## [0.55.0] - 2022-06-12
### Added
* `apply`: now multithreaded with rayon (up to 10x 🚀🚀🚀 faster!) https://github.com/jqnatividad/qsv/pull/342

### Changed
* `apply`: refactor hot loop to use enums instead of nested if https://github.com/jqnatividad/qsv/pull/343
* `sniff`: more idiomatic vec loop https://github.com/jqnatividad/qsv/commit/2a70134bf45f9485bcbb27579f92f89abb7b6bb1
* `validate`: optimizations (up to 20% 🚀 faster) https://github.com/jqnatividad/qsv/commit/0f0be0aba0a6d0cd10f5c96fd17ffd726d3231d1
* `excel`: optimize trimming https://github.com/jqnatividad/qsv/commit/780206a575d40cf759abd295aa91da640e5febed
* various whirlwind tour improvements (more timings, flows/reads better, removed non-sequiturs)
* improved progress bar prep (unstyled progress bar is not momentarily displayed, standardized across cmds)
* bumped reqwest patch to latest upstream https://github.com/jqnatividad/qsv/commit/cb0eb1717f07d8481211e289e6762d9b994fac18
* Bump actions/setup-python from 3.1.2 to 4.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/339
* Bump mlua from 0.7.4 to 0.8.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/340

### Fixed
* fixed error-handling in util::count_rows()  https://github.com/jqnatividad/qsv/pull/341
* do not panic when index is stale https://github.com/jqnatividad/qsv/commit/36dbd79591e3ae1e9c271ec3c0272599cc8695de
* `fetch`: fixed docopt arg processing so --help text is displayed properly https://github.com/jqnatividad/qsv/commit/0cbf7017ebc7f28fa67951133e3bac7d2c7a1368
* `excel`: more robust error handling https://github.com/jqnatividad/qsv/commit/413c693320653d085b5cca48ca32b0d371ccd240

## [0.54.0] - 2022-06-08

### Added
* `stats`: added [outer fences](https://www.statisticshowto.com/upper-and-lower-fences/) to help identify extreme and mild outliers  https://github.com/jqnatividad/qsv/pull/337

### Changed
* `stats`: change skewness algorithm to use [quantile-based measures](https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures)
* whirlwind tour: added more stats about stats command; updated stats output with the additional columns
* pin nightly to 2022-06-07
* cargo update bump several dependencies

### Fixed
* fixed stats quartile tests, as the results were being prematurely truncated, causing in false negative test results

## [0.53.0] - 2022-06-05

### Changed
* `stats`: changed `--dates-whitelist` option to use "all" instead of "\<null\>"; better usage text; more perf tweaks; more tests https://github.com/jqnatividad/qsv/pull/334
* `stats`: mem alloc tweaks & date-inferencing optimization https://github.com/jqnatividad/qsv/pull/333
* `apply`: improved usage text about --formatstr https://github.com/jqnatividad/qsv/commit/2f18565caec6c6e900f776c5f6f3e1adf4c9b6e1
* `sample`: added note about why we don't need crypto secure random number generators https://github.com/jqnatividad/qsv/commit/3384d1a9630bc1033ff67db5dcbf48c067e97728
* `excel` & `slice`: avoid panic by replacing `abs` with `unsigned_abs` https://github.com/jqnatividad/qsv/commit/7e2b14a5de67e70ee0b26ea0eff83462dbc77a0a
* turn on once_cell `parking_lot` feature for storage efficiency/performance https://github.com/jqnatividad/qsv/commit/849548cde8bc9c2d96ddf464f2578faf63d6e9cf
* applied various cargo +nightly clippy optimizations
* pin nightly build to Rust Nightly 2022-06-04
* made various optimizations to our csv fork https://github.com/BurntSushi/rust-csv/compare/master...jqnatividad:perf-tweaks
* cargo bump updated several dependencies

## [0.52.2] - 2022-06-01
### Added
* added `QSV_PREFER_DMY` environment variable. https://github.com/jqnatividad/qsv/pull/331

### Changed
* reorganized Environment Variables section in README https://github.com/jqnatividad/qsv/commit/f25bbf0361fcb7b960d45590ca35b2e676a4497d
* logging: longer END snippet to make it easier to match START/END pairs
* added Boston 311 sample data to tests
* Bump uuid from 1.1.0 to 1.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/332
* cargo update bumped packed_simd_2 from 0.3.7 to 0.3.8

### Fixed
* Instead of panicking, do proper error-handling on IO errors when checking utf-8 encoding https://github.com/jqnatividad/qsv/pull/331/commits/37b4482aae77563995f13a15f73ca8849df6a27d

## [0.52.1] - 2022-05-31
### Added
* added qsv GitHub social media image which

### Changed
* `stats`: added sum integer overflow handling. If sum overflows, instead of panicking, the value 'OVERFLOW' is returned
* upgraded to faster qsv_dateparser 0.4.2, which parses the slash_dmy/slash_mdy date formats earlier in the parse tree, which has more prevalent usage.
* nightly builds are now bundled into the main distribution zip archive.
* renamed qsv_rust_version_info.txt to qsv_nightly_rust_version.info.txt in the distribution zip archive to make it clearer that it only pertains to nightly builds
* cargo bump update several dependencies

### Removed
* nightly distribution zip archives have been removed, now that the nightly builds are in the main zip archive.

### Fixed
* `stats`: prefer_dmy date-parsing preference was not used when computing date min/max
* `stats`: prefer_dmy setting was not initialized properly the first time its called
* nightly build self-update now works properly, now that they are bundled into the main distribution zip archive

## [0.52.0] - 2022-05-29
### Added
* `apply`: DATEFMT subcommand now has a `--prefer-dmy` option. https://github.com/jqnatividad/qsv/pull/328
* `stats` and `schema`: add `--prefer-dmy` option. https://github.com/jqnatividad/qsv/pull/329
* `sniff`: can now sniff Date and Datetime data types.  https://github.com/jqnatividad/qsv/pull/330
* `sniff`: added to `qsvdp` - [DataPusher+](https://github.com/dathere/datapusher-plus)-optimized qsv binary
* added DevSkim security linter Github Action to CI

### Changed
* applied various clippy pedantic and nursery recommendations
* cargo bump updated several dependencies, notably [qsv-dateparser](https://docs.rs/qsv-dateparser/0.4.1/qsv_dateparser/) with its new DMY format parsing capability and
  [qsv-sniffer](https://github.com/jqnatividad/qsv-sniffer) with its Date and Datetime data type detection

### Fixed
* Closed all cargo-audit findings(https://github.com/jqnatividad/qsv/issues/167), as the latest `qsv-dateparser` eliminated qsv's `chrono` dependency.
* Properly create `qsv_rust_version_info.txt` in nightly builds
* Fixed multithreading link in Features Flag section

## [0.51.0] - 2022-05-27
### Added
* `sniff`: sniff field names as well in addition to field data types in https://github.com/jqnatividad/qsv/pull/317
* `sniff`: intelligent sampling. In addition to specifying the number of first n rows to sample, when `--sample`
is between 0 and 1 exclusive, its treated as a percentage of the CSV to sample (e.g. 0.20 is 20 percent).
If its zero, the entire file is sampled. https://github.com/jqnatividad/qsv/pull/318
* `schema`: add --stdout option in https://github.com/jqnatividad/qsv/pull/321
* `stats`: smart date inferencing with field-name date whitelist. Also did some minor tweaks for a little more performance in https://github.com/jqnatividad/qsv/pull/327
* `rename`: added to `qsvdp` - [DataPusher+](https://github.com/dathere/datapusher-plus)-optimized qsv binary 

### Changed
* Switch to qsv_sniffer fork of csv_sniffer. [qsv_sniffer](https://github.com/jqnatividad/qsv-sniffer) has several optimizations (field name sniffing, utf-8 encoding detection, 
SIMD speedups, [etc.](https://github.com/jqnatividad/qsv-sniffer/releases/tag/0.4.0)) that enabled the added `sniff` features above. https://github.com/jqnatividad/qsv/pull/320
* Bump uuid from 1.0.0 to 1.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/323
* Improved Performance Tuning section with more details about UTF-8 encoding, and Nightly builds
* Updated list of commands that use an index
* cargo update bump dependencies, notably jql 4.0.3 to 4.0.4, and cookie_store from 0.16.0 to 0.16.1

### Fixed
* pinned Rust Nightly to 2022-05-23. Later Rust Nightly releases "broke" packed-simd dependency
which prevented us from building qsv's nightly build. (see https://github.com/apache/arrow-rs/issues/1734)

## [0.50.1] - 2022-05-23
### Changed
* disable simd acceleration feature on our csv-sniffer fork so we can publish on crates.io

## [0.50.0] - 2022-05-23
### Added
* `input`:  added `--auto-skip` CSV preamble option in https://github.com/jqnatividad/qsv/pull/313
* `sniff`: support non-utf8 files; flexible detection now works; rename --len to --sample in https://github.com/jqnatividad/qsv/pull/315
* `sniff`: added `is_utf8` property in https://github.com/jqnatividad/qsv/pull/316
* added RFC4180 section to README

### Changed
* `validate`: improve RFC4180 validation messages in https://github.com/jqnatividad/qsv/pull/309
* `stats`: nullcount is a "streaming" statistic and is now on by default in https://github.com/jqnatividad/qsv/pull/311
* `schema`: refactored stdin processing 
* Made logging more consistent in https://github.com/jqnatividad/qsv/pull/314
* bumped MSRV to Rust 1.61.0
* use a qsv-optimized fork of csv-sniffer (https://github.com/jqnatividad/csv-sniffer/tree/non-utf8-qsv), that fixes flexible detection,
  reads non-utf8 encoded files, reports if a file is utf8-encoded, and uses SIMD/CPU features to accelerate performance.
* applied select pedantic clippy recommendations
* bumped several dependencies, notably regex from 1.5.5 to 1.5.6

### Fixed
* `py`: enabled `abi3` feature properly, so qsv now works with higher versions of python over v3.8

## [0.49.0] - 2022-05-17
### Added
* `validate`: add `--json` & `--pretty-json` options for RFC4180 check in https://github.com/jqnatividad/qsv/pull/303
* `qsvdp`: add `validate` command in https://github.com/jqnatividad/qsv/pull/306
* added rust nightly version info to nightly builds

### Changed
* apply select clippy::pedantic recommendations in https://github.com/jqnatividad/qsv/pull/305
* Bump actions/checkout from 2 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/300
* `sniff` and `validate` json errors are now JSONAPI compliant
* cargo update bump several dependencies

### Removed
* removed unused debian package publishing workflow 

### Fixed
* `sniff`: preamble and rowcount fixes in https://github.com/jqnatividad/qsv/pull/301
* `schema`: fixed stdin bug in https://github.com/jqnatividad/qsv/pull/304

## [0.48.1] - 2022-05-16
### Fixed:
* Fixed conditional compilation directives that caused qsvdp build to fail.

## [0.48.0] - 2022-05-15
### Added
* `dedup`: add `--sorted` option in https://github.com/jqnatividad/qsv/pull/286
* `sniff`: add `--json` and `--pretty-json` options in https://github.com/jqnatividad/qsv/pull/297
* added rust version info to nightly build zip files so users can see which Rust nightly version was used to build the nightly binaries

### Changed:
* `stats`: added more `--infer-dates` tests
* number of processors used now logged when logging is on
* `python`: nightly build optimization in https://github.com/jqnatividad/qsv/pull/296
* moved Performance Tuning to its own markdown file, and included it in the TOC
* bumped several dependencies, notably `rayon`, `jsonschema` and `pyo3`
* moved FAQ from Wiki to Discussions
* added clone count badge

### Fixed:
* `python`: should now work with python 3.8, 3.9.or 3.10

## [0.47.0] - 2022-05-12
### Added
* `dedup` and `sort` are now multithreaded with rayon in https://github.com/jqnatividad/qsv/pull/283
* add `--jobs` option to `schema` and `validate` in https://github.com/jqnatividad/qsv/pull/284

### Changed
* `--jobs` and `QSV_MAX_JOBS` settings also now work with rayon
* cargo update bump several dependencies
* upgrade `calamine` fork patch that enables `excel` command
* removed `target-cpu=native` in nightly builds so they are more portable

### Fixed
* fixed `publish-nightly` workflow bugs so nightly builds are built properly
* corrected several build instructions errors in README
* fixed `workdir:output_stderr()` helper so it also returns std_err message
* fixed `Rust Beta` workflow so we can also manually test against Rust Beta

## [0.46.1] - 2022-05-08
### Changed
* `extsort`: increased performance. Use 10% of total memory or if total mem is not detectable, 100 mb for in-mem sorting. Increased R/W buffer size to 1mb [e2f013f](https://github.com/jqnatividad/qsv/commit/e2f013f267ce0add457a3a64bc16b9924c142a05)
* `searchset`: more idiomatic rust [fa1f340](https://github.com/jqnatividad/qsv/commit/fa1f340c3084cea548008ec204ec12bc67c60ad7)
* added "Nightly Release Builds" section in README Performance Tuning
* cargo update bump several dependencies

### Fixed
* `excel`: fixed off by +1 row count (we were counting the header as well); added column count to final message and removed useless human-readable option. [c99df2533b5c112d90c6e04068227b7f873459c2](https://github.com/jqnatividad/qsv/commit/c99df2533b5c112d90c6e04068227b7f873459c2)
* fixed various bugs in Publish Nightly GitHub Action that automatically built nightly binaries

## [0.46.0] - 2022-05-07
### Added
* Added release nightly binaries, optimized for size and speed
   * uses Rust nightly
   * also compiles stdlib, so build-time optimizations also apply, instead of using pre-built stdlib
   * set `panic=abort` - removing panic-handling, formatting and backtrace code from binaries
   * set `RUSTFLAGS= -C target-cpu=native` to enable use of additional CPU-level features
   * enables unstable/nightly features on `regex` and `rand` crates
* Added testing on nightly to CI

### Changed
* `dedup`: reduced memory footprint by half by writing directly to disk, rather than storing in working mem, before writing
* `excel`: show sheet name in message along with row count; let docopt take care of validating mandatory arguments
* More whirlwind tour improvements - how timings were collected, footnotes, etc.
* Bump github/codeql-action from 1 to 2 by @dependabot in https://github.com/jqnatividad/qsv/pull/277
* Bump log from 0.4.16 to 0.4.17 by @dependabot in https://github.com/jqnatividad/qsv/pull/278
* Bump whatlang from 0.15 to 0.16
* Make file extension processing case-insensitive in https://github.com/jqnatividad/qsv/pull/280
* Added Caching section to Performance Tuning
* Added UTF-8 section to Performance Tuning

### Removed
* removed unneeded header file for wcp.csv used in Whirlwind Tour, now that we have a well-formed wcp.csv

## [0.45.2] - 2022-05-01
### Added
* added `headers` command to qsvdp binary

### Changed
* cargo update bump semver from 1.0.7 to 1.0.8

## [0.45.1] - 2022-05-01
### Added
* added rust-clippy GH action workflow
* added security policy

### Changed:
* `extsort`: use util::njobs to process --jobs option
* various improvements on Whirlwind tour to help users follow along
* `extsort`: add link to "External Sorting" wikipedia article
* `extsort`: made <input> and <output> mandatory docopt arguments 
* `sort`: mention `extsort` in usage text
* added markdownlint.json config to suppress noisy markdown lints in VSC
* reformatted README to apply some markdown lints
* bump whatlang from 0.14 to 0.15
* bump qsv-stats from 0.3.6 to 0.3.7 for some minor perf improvements

## [0.45.0] - 2022-04-30
### Added
* Added `extsort` command - sort arbitrarily large text files\CSVs using a multi-threaded external sort algorithm.

### Changed
* Updated whirlwind tour with simple `stats` step
* `py`: Automatically create python3.dll import libraries on Windows targets
* Updated build instructions to include `full` feature
* `index`: mention QSV_AUTOINDEX env var in usage text
* Corrected minor typos
* Bump jql from 4.0.1 to 4.0.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/276
* cargo update bump several dependencies - notably mimalloc

## [0.44.0] - 2022-04-27
### Added
* Created new binary - qsvdp - binary optimized for [DataPusher+](https://github.com/dathere/datapusher-plus) in https://github.com/jqnatividad/qsv/pull/273
  qsvdp only has DataPusher+ relevant commands, with the self-update engine removed. This results in a binary that's
  3x smaller than qsvlite, and 6x smaller than qsv will all features enabled.

### Changed
* `dedup`: send dupe count to stderr in https://github.com/jqnatividad/qsv/pull/272
* `dedup`: improve usage text
* cargo update bump several crates

### Fixed
* `count`: corrected usage text typo

## [0.43.0] - 2022-04-26
### Added
* `input` can now effectively transcode non-utf-8 encoded files to utf-8 in https://github.com/jqnatividad/qsv/pull/271

### Changed
* `table`: made it flexible - i.e. each row can have varying number of columns
* `excel`: remove unneeded closure

## [0.42.2] - 2022-04-25
### Changed
* use our grex fork, as the upstream fork has an unpublished version number that prevents us from publishing on crates.io.

## [0.42.1] - 2022-04-25
### Changed
* use `[patch.crates-io]` to use crate forks, rather than using the git directive in the dependencies section.
This has the added benefit of making the dependency tree smaller, as other crates that depend on the patched crates also
use the patches. This should also result in smaller binaries.

## [0.42.0] - 2022-04-24
### Added
* `input` refactor. Added trimming and epilog skiplines option. https://github.com/jqnatividad/qsv/pull/270
* `sniff`: added note about sniff limitations
* also publish x86_64-unknown-linux-musl binary

### Changed
* Bump anyhow from 1.0.56 to 1.0.57 by @dependabot in https://github.com/jqnatividad/qsv/pull/268
* Bump jsonschema from 0.15.2 to 0.16.0
* use optimized fork of rust-csv, with non-allocating, in-place trimming and various perf tweaks
* use optimized fork of docopt.rs, with various perf & memory allocation tweaks
* use reqwest fork with unreleased changes that remove unneeded crates
* `validate`: use `from_utf8_unchecked` in creating json instances for performance

### Fixed
* `input`: Fixed line-skipping logic so CSV parsing is flexible - i.e. column count can change between records

## [0.41.0] - 2022-04-21
### Added
* `input`: add `--skip-lines` option in https://github.com/jqnatividad/qsv/pull/266

### Changed
* More verbose, matching START/END logging messages when `QSV_LOG_LEVEL` is enabled.
* Bump whatlang from 0.13.0 to 0.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/264
* Bump filetime from 0.2.15 to 0.2.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/263
* Bump uuid from 0.8 to 1 in https://github.com/jqnatividad/qsv/pull/267
* Minor documentation improvements
* `cargo update` bumped several other second-level dependencies

## [0.40.3] - 2022-04-14
### Changed
* Bump pyo3 from 0.16.3 to 0.16.4
* `stats`: renamed `--dates` option to `--infer-dates`
### Fixed
* `stats`: fixed panic caused by wrong type inference when `--infer-dates` option is on in https://github.com/jqnatividad/qsv/pull/256

## [0.40.2] - 2022-04-14
### Changed
* Datapusher tweaks, primarily to help with datapusher error-handling in https://github.com/jqnatividad/qsv/pull/255
* `excel`: exported count with `--human-readable` option
* use calamine fork to bump dependencies, and reduce binary size
* Bump rayon from 1.5.1 to 1.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/254
* Bump jql from 4.0.0 to 4.0.1

### Fixed
* removed unnecessary *.d dependency files from published binaries zip

## [0.40.1] - 2022-04-13
### Changed
* use performance tweaked forks of csv crate
* Made `this_error` dependency optional with `fetch` feature 
* Made `once_cell` dependency optional  with `apply` and `fetch` features

### Fixed
* Fixed qsv binary publishing. qsv binary was not built properly, it was built using a qsvlite profile

## [0.40.0] - 2022-04-12
### Added
* `excel` command in https://github.com/jqnatividad/qsv/pull/249 and https://github.com/jqnatividad/qsv/pull/252

### Changed
* Bump jql from 3.3.0 to 4.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/251
* Bump actions/setup-python from 3.1.1 to 3.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/250

## [0.39.1] - 2022-04-11
### Fixed
* added version to grex dependency as its required by crates.io, though we're still using the grex fork without the CLI components.

## [0.39.0] - 2022-04-10
### Added
* `QSV_AUTOINDEX` environment variable. When set, autoindexes csv files, autoupdates stale indices 
* `replace`: \<NULL\> `--replacement` option (https://github.com/jqnatividad/qsv/pull/244)
* qsv now automatically screens files for utf-8 encoding. Set `QSV_SKIPUTF8_CHECK` env var to skip encoding check. (https://github.com/jqnatividad/qsv/pull/245 and https://github.com/jqnatividad/qsv/pull/248)

### Changed
* `foreach`: refactored. (https://github.com/jqnatividad/qsv/pull/247)
* Bump jql from 3.2.3 to 3.3.0
* Bump actions/setup-python from 3.1.0 to 3.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/246
* use grex fork to remove unneeded CLI dependencies

## [0.38.0] - 2022-04-05
### Changed
* qsv **requires** UTF-8/ASCII encoded files. Doing so allows us to squeeze more performance by removing UTF-8 validation in https://github.com/jqnatividad/qsv/pull/239 and https://github.com/jqnatividad/qsv/pull/240
### Fixed
* fixed `--jobs` parameter parsing for multithreaded commands in https://github.com/jqnatividad/qsv/pull/236 and https://github.com/jqnatividad/qsv/pull/237

## [0.37.2] - 2022-04-03
### Fixed
* Handle/log self-update errors in https://github.com/jqnatividad/qsv/pull/233

## [0.37.1] - 2022-04-03
### Changed
* `fetch` and `apply`: use cheaper, faster lookup tables for dynamic formatting in https://github.com/jqnatividad/qsv/pull/231
* Cleanup - remove commented code; convert `match` to `if let`; more pedantic clippy recommendations, etc. in https://github.com/jqnatividad/qsv/pull/232

## [0.37.0] - 2022-04-02
### Added
* `enumerate`: added `--constant` <NULL> sentinel value in https://github.com/jqnatividad/qsv/pull/219
* `fetch`: added `--jqlfile` option in https://github.com/jqnatividad/qsv/pull/220
* `stats`: more perf tweaks in https://github.com/jqnatividad/qsv/pull/223

### Changed
* `fetch`: argument parsing refactor, removing need for dummy argument in https://github.com/jqnatividad/qsv/pull/222
* applied select pedantic clippy recommendations in https://github.com/jqnatividad/qsv/pull/224
* simplified multithreading - removed jobs div by three heuristic in https://github.com/jqnatividad/qsv/pull/225
* use qsv-dateparser fork of dateparser for increased performance of `stats`, `schema` and `apply` in https://github.com/jqnatividad/qsv/pull/230
* Bump actions/checkout from 2.3.3 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/228
* Bump actions/stale from 3 to 5 by @dependabot in https://github.com/jqnatividad/qsv/pull/227
* Bump actions/setup-python from 2 to 3.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/226

## [0.36.1] - 2022-03-26
### Changed
* `validate`: use user agent & compression settings when fetching jsonschema from a URL in https://github.com/jqnatividad/qsv/pull/207
* Build and publish smaller qsvlite binary in https://github.com/jqnatividad/qsv/pull/208, https://github.com/jqnatividad/qsv/pull/210 & https://github.com/jqnatividad/qsv/pull/213
* `sniff`: now works with stdin in https://github.com/jqnatividad/qsv/pull/211 and https://github.com/jqnatividad/qsv/pull/212
* `stats`: remove smartstring in https://github.com/jqnatividad/qsv/pull/214
* various performance tweaks in `stats` and `select`
### Fixed
* README: Installation - git:// is no longer supported by GitHub  by @harrybiddle in https://github.com/jqnatividad/qsv/pull/205
* README: Fixed wrong footnote for feature flags
* Silent error when an index file is not found is now logged (https://github.com/jqnatividad/qsv/commit/7f2fe7f3259fb74a8d76396dcc2aa71585967b9b)
* bumped self-update to 0.29. This partly addresses #167, as self-update had an indirect dependency to `time` 0.1.43.

## [0.36.0] - 2022-03-20
### Added
* `sniff`: new command to quickly detect CSV metadata in https://github.com/jqnatividad/qsv/pull/202
* auto-delimiter setting with `QSV_SNIFF_DELIMITER` environment variable in https://github.com/jqnatividad/qsv/pull/203
* `apply`: new `dynfmt` multi-column, dynamic formatting subcommand in https://github.com/jqnatividad/qsv/pull/200
* `fetch`: new multi-column dynamic formatting with --url-template option in https://github.com/jqnatividad/qsv/pull/196
### Changed
* `fetch`: --url-template safety tweaks in https://github.com/jqnatividad/qsv/pull/197
* `fetch`: automatically minify JSON responses. JSON can still be pretty-printed with --pretty option in https://github.com/jqnatividad/qsv/pull/198
* `fetch` is now an optional feature in https://github.com/jqnatividad/qsv/pull/201
* `sniff`: improved display in https://github.com/jqnatividad/qsv/pull/204
* slim down dev-dependencies
### Fixed:
* `py`: now checks if first character of a column is a digit, and replaces it with an underscore

## [0.35.2] - 2022-03-13
### Added
* README: Added datHere logo
### Changed
* `py`: ensure valid python variable names https://github.com/jqnatividad/qsv/pull/192
* `fetch`: dev-dependency actix upgrade (actix-governor from 0.2->0.3; actix-web from 3.3->4.0) https://github.com/jqnatividad/qsv/pull/193
* `lua`: replace hlua with mlua  https://github.com/jqnatividad/qsv/pull/194
* `stats`: refactor for performance - skip from_utf8 check as input is utf8 transcoded as necessary; smartstring https://github.com/jqnatividad/qsv/pull/195
* Whirlwind Tour: show country-continent.csv file with comment handling
* cargo bump update several dependencies

### Fixed
* `stats`: only compute quartiles/median for int/float fields - https://github.com/jqnatividad/qsv/pull/195

## [0.35.1] - 2022-03-08

### Changed
- README: note about `--output` option changing delimiter automatically based on file extension and UTF-8 encoding the file
- README: Windows usage note about UTF16-LE encoding and `--output` workaround
### Fixed
* upgraded regex to 1.5.5 which resolves the [GHSA-m5pq-gvj9-9vr8 security advisory](https://github.com/rust-lang/regex/security/advisories/GHSA-m5pq-gvj9-9vr8)

## [0.35.0] - 2022-03-08
### Added
* `count`: `--human-readable` option in https://github.com/jqnatividad/qsv/pull/184
* Automatic utf8 transcoding in https://github.com/jqnatividad/qsv/pull/187
* Added NYC School of Data 2022 presentation
* Added ahash 0.7 and encoding_rs_io 0.1 dependencies

### Changed
* Use ahash::AHashMap instead of std::collections::HashMap for performance in https://github.com/jqnatividad/qsv/pull/186
* Revamped Whirlwind Tour
* bumped several dependencies 
  * anyhow 1.0.55 to 1.0.56
  * ipnet 2.3.1 to 2.4.0
  * pyo3 0.16.0 to 0.16.1

### Fixed
* `py`: convert spaces to underscores for valid python variable names when Column names have embedded spaces in https://github.com/jqnatividad/qsv/pull/183
* docs: CSV Kit got a 10x improvement by @jpmckinney in https://github.com/jqnatividad/qsv/pull/180
* `fetch`: added jql selector to cache key
* Corrected README mixup re `join` hashmap indices and qsv indices

## New Contributors
* @jpmckinney made their first contribution in https://github.com/jqnatividad/qsv/pull/180

## [0.34.1] - 2022.03-04
### Added
* `stats`: added `--dates` option. This option turns on date/datetime data type inferencing, which is 
a very expensive operation. Only use this option when you have date/datetime fields and you want to 
compile the proper statistics for them (otherwise, they will be treated as "String" fields.)

## [0.34.0] - 2022.03-03
### Added
* added intentionally kitschy qsv logo :grin:
* `stats`: added `datetime` data type inferencing
* `fetch`: added optional Redis response caching
* `schema`: added `--strict-dates` option by @mhuang74 in https://github.com/jqnatividad/qsv/pull/177 
* `validate`: added more robust [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)-compliance checking when no jsonschema is provided
* added Redis to CI

### Changed
* bumped reverse-geocoder crate from 2.0.1 to 3.0.0 to modernize geonames reverse geocoder
* bumped cached crate from 0.30.0 to 0.33.0 to enable Redis response caching
* bumped various other dependencies to latest release

### Fixed
* removed invalid `--path` cargo install option in README
* `workdir.rs` was not properly cleaning up test files

## [0.33.0] - 2022.02-27
### Added
* `fetch`: add `--url-template` and `--redis` options in https://github.com/jqnatividad/qsv/pull/175
* `stats`: add `DateTime` data type (RFC3339 format) in https://github.com/jqnatividad/qsv/pull/176
* added Rust Beta to Github Actions CI

### Changed
* `validate`: improve performance and simplify error report format by @mhuang74 in https://github.com/jqnatividad/qsv/pull/172
* Addl `validate` performance tweaks in https://github.com/jqnatividad/qsv/pull/173
* changed MSRV to latest Rust stable - 1.59.0
* removed `num_cpus` crate and use new `std::thread::available_parallelism` stabilized in Rust 1.59.0
* use new cargo.toml `strip` option to strip binaries
* refactored GitHub Actions CI to make it faster

## [0.32.2] - 2022-02-20
### Changed
* `schema` (#60): pattern constraint for string types by @mhuang74 in https://github.com/jqnatividad/qsv/pull/168
* `validate`: improve performance by @mhuang74 in https://github.com/jqnatividad/qsv/pull/170
* `fetch`: Spell out k:v -> key:value in docopt usage text
* cargo update bump several dependencies

### Fixed
* `validate`: bug fix and refactor by @mhuang74 in https://github.com/jqnatividad/qsv/pull/171

## [0.32.1] - 2022-02-14
### Changed
* `fetch`: upgrade to jql 3.1.0 by @mhuang74 in https://github.com/jqnatividad/qsv/pull/160
* `schema`: refactor tests by @mhuang74 in https://github.com/jqnatividad/qsv/pull/161
* `schema`: support Enum constraint by @mhuang74 in https://github.com/jqnatividad/qsv/pull/162
* `schema`: default to include value constraints  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/166
* bumped `qsv-stats` to 0.3.6 for `stats` & `frequency` performance tweaks
* specify that `apply geocode` expects WGS84 coordinate system
* cargo update bump several dependencies
* changed CI to run clippy and rustfmt automatically

### Fixed
* `schema`: Fix bug with enum by @mhuang74 in https://github.com/jqnatividad/qsv/pull/163

## [0.32.0] - 2022-02-06
### Added
* `schema` POC by @mhuang74 in https://github.com/jqnatividad/qsv/pull/155
* `schema`: add value constraints via stats  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/158
* `schema`: update command description by @mhuang74 in https://github.com/jqnatividad/qsv/pull/159

### Changed
* `stats` data type inference changed to more straightforward "String" from "Unicode"
* changed CI/CD to use rust-cache GitHub Actions making it ~3x faster.
* always build and test with `--locked` flag. This allows us to use rust-cache and guarantee that
  builds are using the exact dependency versions qsv requires.
* bumped `qsv-stats` to 0.3.5 for `stats` performance tweaks  

### Fixed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/154

## [0.31.0] - 2022-01-31
### Changed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/151
* Python 3.8 (current stable version) is now required for the `py` command. Changed from Python 3.7.
* bumped jsonschema dependency to to 0.15.
* always build/publish with `--locked` flag in CI/CD.
* enclose environment variable values with double quotes when using `--envlist` option
* use more captured identifiers in format strings.

### Added
* added `--helper` option to `py` command. This allows users to load a python user helper script as a module named `qsv_uh`. [Example](https://github.com/jqnatividad/qsv/blob/78046d922e9a530c0887f18065fc325049b58687/tests/test_py.rs#L93) 
* added support for last N records in `slice` command by allowing negative values for the `slice --start` option.
* added progress bar to `py` command.

## [0.30.1] - 2022-01-23
### Changed
* convert more format strings to use captured identifiers
* bump jsonschema to 0.14.0. This will allow cross-compilation to work again as 
  we can explicitly use rustls for reqwest. This is required as cross no longer bundles openssl.

### Fixed
* fixed broken self-update ([#150](https://github.com/jqnatividad/qsv/issues/150))

## [0.30.0] - 2022-01-22
### Added
* `validate` command by @mhuang74 in https://github.com/jqnatividad/qsv/pull/145
* README: additional information on xsv fork differences

### Changed
* bumped MSRV to 1.58.1
* `validate` tweaks in https://github.com/jqnatividad/qsv/pull/148
* `validate` buffered jsonl error report in https://github.com/jqnatividad/qsv/pull/149

### Fixed
* fix `fetch` bugs by @mhuang74 in https://github.com/jqnatividad/qsv/pull/146
* README: added missing `--path` option in `cargo install`

## [0.29.1] - 2022-01-17
### Changed
* refactored `--update` to give update progress messages; run on `--help` as well
* updated README
  - remove bold formatting of commands
  - expanded descriptions of
      - fixlengths
      - foreach
      - jsonl
      - py
    - searchset
  - added reason why pre-built binaries on some platforms do not have the python feature installed.
  - drop use of "parallelism", just say "multithreading"
  - expanded Feature Flag section
* bump cached from 0.26 to 0.29
* added `update_cache_info!` macro to util.rs, replacing redundant code for progress indicators with cache info
* bump MSRV to Rust 1.58
* use new Rust 1.58 captured identifiers for format strings
* added `output_stderr` test helper to test for expected errors in CI
* added tests for invalid delimiter length; truncated comment char and unknown apply operators
* pointed documentation to Github README instead of doc.rs
* added `rustup update` to Github Actions publish workflow as Github's runners are still on Rust 1.57
* added Debian package build to publish workflow for `x86_64-unknown-linux-musl`

### Fixed
* corrected help text on job divisor is 3 not 4 for multithreaded commands (`frequency`, `split` and `stats`)
* corrected `stats` help text to state that multithreading requires an index

## [0.29.0] - 2022-01-08
### Changed
* `fetch`: enable cookies and storing error messages by @mhuang74 in https://github.com/jqnatividad/qsv/pull/141
* `fetch`: improve jql integration by @mhuang74 in https://github.com/jqnatividad/qsv/pull/139
* `--envlist` option now returns all qsv-relevant environment variables in https://github.com/jqnatividad/qsv/pull/140
* Move logging and update utility functions to util.rs in https://github.com/jqnatividad/qsv/pull/142
* `fetch`: support custom http headers by @mhuang74 in https://github.com/jqnatividad/qsv/pull/143
* bumped whatlang to 13.0 which supports Tagalog detection
* improved documentation of feature flags, environment variables & `stats` command

### Added
* added JSONL/NDJSON to Recognized File Formats (thru `jsonl` command)
* added CODE_OF_CONDUCT.md

### Deleted
* removed WIP indicator from `fetch` in README

## [0.28.0] - 2021-12-31
### Changed
* Fetch: support rate limiting by @mhuang74 in https://github.com/jqnatividad/qsv/pull/133
* Runtime minimum version check for Python 3.7 if `python` feature is enabled by @jqnatividad in https://github.com/jqnatividad/qsv/pull/138
* Fine-tuned GitHub Actions publish workflow for pre-built binaries
   * removed upx compression, as it was creating invalid binaries on certain platforms
   * enabled `python` feature on x86_64 platforms as we have access to the Python interpreter on GitHub's Action runners
   * include both `qsv` and `qsvlite` in the distribution zip file
* Formatted Cargo.toml with Even Better TOML VS code extension
* changed Cargo.toml categories and keywords
* removed patch version number from Cargo.toml dependencies. Let cargo do its semver dependency magic, and we include the Cargo.lock file anyway.

### Added
* added example of Python f-string formatting to `py` help text
* added Python f-string formatting test
* Added note in README about enabled features in pre-built binaries

### Deleted
* Removed _**NEW**_ and _**EXTENDED**_ indicators in README

## [0.27.1] - 2021-12-28
### Changed
* changed publish workflow for apple targets to use Xcode 12.5.1 from 12.4
* `jsonl` command now recognize and process JSON arrays
* `--version` option now shows binary name and enabled features
* Use upgraded [`qsv_currency`](https://crates.io/crates/qsv_currency) fork to power `apply currencytonum` operation. Now supports currency strings
  (e.g. USD, EUR, JPY, etc) in addition to currency symbols (e.g. $, €, ¥, etc)
* renamed `QSV_COMMENTS` environment variable to `QSV_COMMENT_CHAR` to make it clear that it clear that we're expecting
  a single character, not a boolean as the old name implies.

### Added
* added `create_from_string` helper function in workdir.rs
* compress select pre-built binaries with [UPX](https://upx.github.io/)
* `qsvlite` binary target, with all features disabled.
* `py` command. Evaluates a Python expression over CSV lines to transform, aggregate or filter them.

### Deleted
* removed Debian package publishing workflow, as the GH action for it
  does not support Rust 2021 edition

## [0.26.2] - 2021-12-21
## Added
* automatic self-update version check when the `--list` option is invoked.
* `QSV_NO_UPDATE` environment variable to prohibit self-update checks.
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.1] - 2021-12-21
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.0] - 2021-12-21
### Changed
* `fetch` refinements. Still WIP, but usable (See [#77](https://github.com/jqnatividad/qsv/issues/77))
  - add default user agent
  - `fetch` progress bar
  - `--jobs`, `--throttle`, `--header`, `--store-error` and `cookies` options still not functional.
* cargo update bump several crates to their latest releases. Of note are `test-data-generation`, 
`self_update` and `jql` where we worked with the crate maintainers directly with the update.

### Fixed
* `--update` bug fixed. It was not finding the binary to self update properly.


## [0.25.2-beta] - 2021-12-13
## Added
* `fetch` command by [@mhuang74](https://github.com/mhuang74). Note that the command is functional but still WIP, that's why this is a beta release.
* Download badge for GitHub pre-built binaries
* Compute hashes for pre-built binaries for verification

## Changed
* Additional helptext for `apply` NLP functions
* standardized on canonical way to suppress progress bars with `--quiet` option
* README: Mentioned `--frozen` option when installing/building qsv; wordsmithing
* rustfmt; clippy

## Deleted
* remove obsolete Makefile and .gitsubmodules
## [0.24.1] - 2021-12-06
### Changed
- changed selfupdate dependency to use pure Rust TLS implementation as cross no longer bundles OpenSSL, causing some binary builds using cross to fail.
## [0.24.0] - 2021-12-06
### Added
* Add logging by @mhuang74 in https://github.com/jqnatividad/qsv/pull/116
* Environment variables for logging - `QSV_LOG_LEVEL` and `QSV_LOG_DIR` - see [Logging](https://github.com/jqnatividad/qsv/blob/master/docs/Logging.md#logging) for more details.
* `sentiment` analysis `apply` operation by @jqnatividad in https://github.com/jqnatividad/qsv/pull/121
* `whatlang` language detection `apply` operation by @jqnatividad in https://github.com/jqnatividad/qsv/pull/122
* aarch64-apple-darwin prebuilt binary (Apple Silicon AKA M1)
* `--envlist` convenience option to list all environment variables with the `QSV_` prefix

### Changed
* changed `MAX_JOBS` heuristic logical processor divisor from 4 to 3
* `selfupdate` is no longer an optional feature

## New Contributors
* @mhuang74 made their first contribution in https://github.com/jqnatividad/qsv/pull/116
## [0.23.0] - 2021-11-29
### Added
- added `--update` option. This allows qsv to check and update itself if there are new release binaries published on GitHub.
- added `--envlist` option to show all environment variables with the `QSV_` prefix.
- `apply`, `generate`, `lua`, `foreach` and `selfupdate` are now optional features. `apply` and `generate` are marked optional since they have
large dependency trees; `lua` and `foreach` are very powerful commands that can be abused to issue system commands. Users now have the option exclude these features from their local builds.  Published binaries on GitHub still have `-all-features` enabled.
- added `QSV_COMMENTS` environment variable (contributed by [@jbertovic](https://github.com/jbertovic)). This allows qsv to ignore lines in the CSV (including headers) that start with the set character. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_comments.rs#L3)
- catch input empty condition when qsv's input is empty when using `select`.   
(e.g. `cat /dev/null | qsv select 1` will now show the error "Input is empty." instead of "Selector index 1 is out of bounds. Index must be >= 1 and <= 0.")
- added `--pad <arg>` option to `split` command to zero-pad the generated filename by the number of `<arg>` places. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_split.rs#L81)
- tests for `QSV_COMMENTS`, `split --pad`, `select` input empty condition, 
### Changed
- set Cargo.toml to Rust 2021 edition
- added "command-line-utilities" category to crates.io metadata
- cargo update bumped `mimalloc`, `serde_json`, `syn`, `anyhow` and `ryu`.
- GitHub Actions CI tests runs with `--all-features` enabled.
- published binaries on GitHub have `--all-features` enabled by default.
- made geocode caching a tad faster by making the transitional cache unbounded, and simplifying the key.
- `--version` now also shows the number of logical CPUs detected.
- project-wide rustfmt
- documentation for features, `QSV_COMMENTS` and `apply`
### Removed
- removed greetings.yml workflow from GitHub Actions.

## [0.22.1] - 2021-11-22
### Added
- added `lua` and `foreach` feature flags. These commands are very powerful and can be easily abused or get into "foot-shooting" scenarios.
They are now only enabled when these features are enabled during install/build.
- `censor` and `censor_check` now support the addition of custom profanities to screen for with the --comparand option.
### Changed
- removed `lazy_static` and used `once_cell` instead
- smaller stripped binaries for `x86_64-unknown-linux-gnu`, `i686-unknown-linux-gnu`, `x86_64-apple-darwin` targets
- expanded `apply` help text
- added more tests (currencytonum, censor, censor_check)

## [0.22.0] - 2021-11-15
### Added
- `generate` command. Generate test data by profiling a CSV using a [Markov decision process](https://docs.rs/test-data-generation).
- add `--no-headers` option to `rename` command (see [discussion #81](https://github.com/jqnatividad/qsv/discussions/81#discussioncomment-1599027))
- Auto-publish binaries for more platforms on release
- added combo-test for sort-dedup-sort (see [discussion #80](https://github.com/jqnatividad/qsv/discussions/80#discussioncomment-1610190))
- New environment variables galore
  - `QSV_DEFAULT_DELIMITER` - single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files, when not set. Note that this will also set the delimiter for qsv's output. Adapted from [xsv PR](https://github.com/BurntSushi/xsv/pull/94) by [@camerondavison](https://github.com/camerondavison).
  - `QSV_NO_HEADERS` - when set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`.
  - `QSV_MAX_JOBS` - number of jobs to use for parallelized commands (currently `frequency`, `split` and `stats`). If not set, max_jobs is set
to number of logical processors divided by four.  See [Parallelization](#parallelization) for more info.
  - `QSV_REGEX_UNICODE` - if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these
commands are not unicode-aware and will ignore unicode values when matching and will panic when unicode characters are used in the regex.
- Added parallelization heuristic (num_cpus/4), in connection with `QSV_MAX_JOBS`.
- Added more tests
  - `apply` (test for regex_replace, eudex, and lat/long parsing)
  - combo-test (see above) - for testing qsv command combinations
  - tests for `QSV_NO_HEADERS` environment variable
  - tests for `QSV_REGEX_UNICODE` environment variable in `search`, `searchset` and `replace` commands
  - tests for `QSV_DEFAULT_DELIMITER` environment variable
### Changed
- MSRV of Rust 1.56
- expanded `apply` help-text examples
- progress bar now only updates every 1% progress by default
- replaced English-specific soundex with multi-lingual eudex algorithm (see https://docs.rs/crate/eudex/0.1.1)
- refactored `apply geocode` subcommand to improve cache performance
- improved lat/long parsing - can now recognize embedded coordinates in text
- changed `apply operations regex_replace` behavior to do all matches in a field, instead of just the left-most one, to be consistent with the behavior of `apply operations replace`

## [0.21.0] - 2021-11-07
### Added
- added `apply geocode` caching, more than doubling performance in the geocode benchmark.
- added `--random` and `--seed` options to `sort` command from [@pjsier](https://github.com/pjsier).
- added qsv tab completion section to README.
- additional `apply operations` subcommands:
  * Match Trim operations - enables trimming of more than just whitespace, but also of multiple trim characters in one pass ([Example](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L214)):
    * mtrim: Trims `--comparand` matches left & right of the string ([trim_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_matches) wrapper)
    * mltrim: Left trim `--comparand` matches ([trim_start_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_start_matches) wrapper)
    * mrtrim: Right trim `--comparand` matches ([trim_end_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end_matches) wrapper)
  * replace: Replace all matches of a pattern (using `--comparand`)
      with a string (using `--replacement`) (Std::String [replace](https://doc.rust-lang.org/std/string/struct.String.html#method.replace) wrapper).
  * regex_replace: Replace the leftmost-first regex match with `--replacement` (regex [replace](https://docs.rs/regex/1.1.0/regex/struct.Regex.html#method.replace) wrapper).
  * titlecase - capitalizes English text using Daring Fireball titlecase style
      https://daringfireball.net/2008/05/title_case
  * censor_check: check if profanity is detected (boolean) [Examples](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L66)
  * censor: profanity filter
- added parameter validation to `apply operations` subcommands
- added more robust parameter validation to `apply` command by leveraging docopt
- added more tests
- added `rust-version` in Cargo.toml to specify MSRV of rust 1.56

### Changed
- revamped benchmark script:
  * allow binary to be changed, so users can benchmark xsv and other xsv forks by simply replacing the $bin shell variable
  * now uses a much larger data file - a 1M row, 512 mb, 41 column sampling of NYC's 311 data
  * simplified and cleaned-up script now that it's just using 1 data file
- Upgrade rand and quickcheck crates to latest releases (0.8.4 and 1.0.3 respectively), and modified code accordingly.
- `cargo update` bumped addr2line (0.16.0->0.17.0), backtrace (0.3.62->0.3.63), gimli (0.25.0->0.26.1) and anyhow (1.0.44->1.0.45)

### Removed
- removed `scramble` command as its function is now subsumed by the `sort` command with the `--random` and `--seed` options
- removed `num-format` crate which has a large dependency tree with several old crates; replaced with much smaller `thousands` crate.
- removed 1M row, 48mb, 7 column world_cities_pop_mil.csv as its no longer used by the revamped benchmark script.
- removed `build.rs` build dependency that was checking for MSRV of Rust >= "1.50". Instead, took advantage of new [`rust-version`](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html#cargo-rust-version) Cargo.toml option
introduced in Rust 1.56.

## [0.20.0] - 2021-10-31
### Added
- added string similarity operations to `apply` command:
  * simdl: Damerau-Levenshtein similarity
  * simdln: Normalized Damerau-Levenshtein similarity (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity (between 0.0 & 1.0)
  * simhm: Hamming distance. Number of positions where characters differ.
  * simod: OSA Distance.
  * soundex: sounds like (boolean)
- added progress bars to commands that may spawn long-running jobs - for this release,
`apply`, `foreach`, and `lua`. Progress bars can be suppressed with `--quiet` option.
- added progress bar helper functions to utils.rs.
- added `apply` to benchmarks.
- added sample NYC 311 data to benchmarks.
- added records per second (RECS_PER_SEC) to benchmarks

### Changed
- major refactoring of `apply` command:
  - to take advantage of docopt parsing/validation.
  - instead of one big command, broke down apply to several subcommands:
    - operations
    - emptyreplace
    - datefmt
    - geocode
- simplified lat/long regex validator to no longer validate range, as the underlying 
geocoder function validates it already - 18% geocode speedup.
- bumped docopt back up to 1.1.1.
- improved error message when specifying an invalid apply operation.

## [0.19.0] - 2021-10-24
### Added
- new `scramble` command. Randomly scrambles a CSV's records.
- read/write buffer capacity can now be set using environment variables
`QSV_RDR_BUFFER_CAPACITY` and `QSV_WTR_BUFFER_CAPACITY` (in bytes).
- added additional test for `apply datefmt`.

### Changed
- default read buffer doubled from 8k to 16k.
- default write buffer doubled from 32k to 64k.
- benchmark script revamped. Now produces aligned output onscreen,
while also creating a benchmark TSV file; downloads the sample file from GitHub;
benchmark more commands.
- version info now also returns memory allocator being used, and number of cpus detected.
- minor refactor of `enumerate`, `explode`, `fill` and `foreach` commands.

### Removed
- removed benchmark data from repository. Moved to GitHub wiki instead.

## [0.18.2] - 2021-10-21
## Changed
- use docopt v1.1.0 instead of docopt v.1.1.1 for docopt to support all regex features

## [0.18.1] - 2021-10-20
### Added
- added `mimalloc` feature flag. [mimalloc](https://github.com/microsoft/mimalloc) is Microsoft's performance-oriented memory allocator.
Earlier versions of qsv used mimalloc by default. Now it is only used when the feature is set.
- README: Added Performance section.
- README: Document how to enable `mimalloc` feature.

### Changed
- README: Explicitly show how to set environment variables on different platforms.
## [0.18.0] - 2021-10-18
### Added
- `stats` `mode` is now also multi-modal -i.e. returns multiples modes when detected. 
e.g. mode[1,1,2,2,3,4,6,6] will return [1,2,6].
It will continue to return one mode if there is only one detected.
- `stats` `quartile` now also computes IQR, lower/upper fences and skew ([using Pearson's median skewness](https://en.wikipedia.org/wiki/Skewness#Pearson's_second_skewness_coefficient_(median_skewness))). For code simplicity, calculated skew with quartile.
- `join` now also support `left-semi` and `left-anti` joins, the same way [Spark does](https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-join.html#semi-join).
- `search` `--flag` option now returns row number, not just '1'.
- `searchset` `--flag` option now returns row number, followed by a semi-colon, and a list of matching regexes.
- README: Added badges for Security Audit, Discussion & Docs
- README: Added FAQ link in fork note.

### Changed
- point to https://docs.rs/crate/qsv for documentation.
- README: `stats` and `join` section updated with new features.
- README: wordsmithing - replaced "CSV data" and "CSV file/s" with just "CSV".
- in `stats` changed `q2` column name to `q2_median`.
- removed debug symbols in release build for smaller binaries.
- minor refactoring of `search`, `searchset` & `stats`.

### Fixed
- README: fixed `flatten` example.

### Removed
- removed Rust badge.

## [0.17.3] - 2021-10-12
### Added
- added [sample regexset file](https://github.com/jqnatividad/qsv/commit/d209436b588b88b0f92cc133ebcada726f72a2bd) for PII-screening.

### Changed
- `apply geocode --formatstr` now accepts less US-centric format selectors.
- `searchset --flag` now shows which regexes match as a list (e.g. "[1, 3, 5]"), not just "1" or "0".
### Fixed
- `foreach` command now returns error message on Windows. `foreach` still doesn't work on 
Windows (yet), but at least it returns "foreach command does not work on Windows.".
- `apply geocode` was not accepting valid lat/longs below the equator. Fixed regex validator.
- more robust `searchset` error handling when attempting to load regexset files.
- `apply` link on README was off by one. 
## [0.17.2] - 2021-10-10

### Changed
- bumped `dateparser` to 0.1.6. This now allows `apply datefmt` to properly reformat
dates without a time component. Before, when reformatting a date like "July 4, 2020", 
qsv returns "2020-07-04T00:00:00+00:00". It now returns "2020-07-04".
- minor clippy refactoring
### Removed
- removed rust-stats submodule introduced in 0.17.1. It turns out
crates.io does not allow publishing of crates with local dependencies on submodules. 
Published the modified rust-stats fork as qsv-stats instead. This allows us to publish
qsv on crates.io
- removed unused `textwrap` dependency
## [0.17.1] - 2021-10-10
### Fixed
- explicitly specified embedded modified rust-stats version in Cargo.toml. 
## [0.17.0] - 2021-10-10
### Added
- added `searchset` command. Run **multiple regexes** over CSV data in a **single pass**.
- added `--unicode` flag to `search`, `searchset` and `replace` commands.
Previously, regex unicode support was on by default, which comes at the cost of performance.
And since `qsv` optimizes for performance ("q is for quick"), it is now off by default.
- added quartiles calculation to `stats`. Pulled in upstream
[pending](https://github.com/BurntSushi/rust-stats/pull/15) [PRs](https://github.com/BurntSushi/xsv/pull/273) 
from [@m15a](https://github.com/m15a) to implement.

### Changed
- changed variance algorithm. For some reason, the previous variance algorithm was causing
intermittent test failures on macOS. Pulled in [pending upstream PR](https://github.com/BurntSushi/rust-stats/pull/11)
from [@ruppertmillard](https://github.com/ruppertmillard).
- embedded [rust-stats fork](https://github.com/jqnatividad/rust-stats) submodule which implements 
quartile and new variance algorithm.
- changed GitHub Actions to pull in submodules.

### Fixed
- the project was not following semver properly, as several new features were released 
in the 0.16.x series that should have been MINOR version bumps, not PATCH bumps.

## [0.16.4] - 2021-10-08
### Added
- added `geocode` operation to `apply` command. It geocodes to the closest city given a column   
with coordinates in Location format ('latitude, longitude') using a static geonames lookup file.   
(see https://docs.rs/reverse_geocoder)
- added `currencytonum` operation to `apply` command.
- added `getquarter.lua` helper script to support `lua` example in [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).
- added `turnaroundtime.lua` helper script to compute turnaround time.
- added `nyc311samp.csv` to provide sample data for recipes.
- added several Date Enrichment and Geocoding recipes to [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).

### Fixed
- fixed `publish.yml` Github Action workflow to properly create platform specific binaries.
- fixed variance test to eliminate false positives in macOS.

## [0.16.3] - 2021-10-06
### Added
- added `docs` directory. For README reorg, and to add detailed examples per command in the future.
- added `emptyreplace` operation to `apply` command.
- added `datefmt` operation to `apply` command.
- added support for reading from stdin to `join` command.
- setup GitHub wiki to host [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook) and sundry docs to encourage collaborative editing.
- added footnotes to commands table in README.

### Changed
- changed GitHub Actions publish workflow so it adds the version to binary zip filename.
- changed GitHub Actions publish workflow so binary is no longer in `target/release` directory.
- reorganized README.
- moved whirlwind tour and benchmarks to `docs` directory.
- use zipped repo copy of worldcitiespop_mil.csv for benchmarks.

### Fixed
- fixed links to help text in README for `fixlengths` and `slice` cmds
- `exclude` not listed in commands table. Added to README.

### Removed
- Removed `empty0` and `emptyNA` operations in `apply` command.
Replaced with `emptyreplace`.

## [0.16.2] - 2021-09-30
### Changed
- changed Makefile to remove github recipe as we are now using GitHub Actions.
- Applied rustfmt to entire project [#56](https://github.com/jqnatividad/qsv/issues/56)
- Changed stats variance test as it was causing false positive test failures on macOS ([details](https://github.com/jqnatividad/qsv/commit/8c45c60de7598c7dc4cedd10ce7cb281ee34db46))
- removed `-amd64` suffix from binaries built by GitHub Actions.

### Fixed
- fixed publish Github Actions workflow to zip binaries before uploading.

### Removed 
- removed `.travis.yml` as we are now using GitHub Actions.
- removed scripts `build-release`, `github-release` and `github-upload` as we are now
 using GitHub Actions.
- removed `ci` folder as we are now using GitHub Actions.
- removed `py` command. [#58](https://github.com/jqnatividad/qsv/issues/58)

## [0.16.1] - 2021-09-28
### Fixed
- Bumped qsv version to 0.16.1. Inadvertently released 0.16.0 with qsv version still at 0.15.0.

## [0.16.0] - 2021-09-28
### Added
- Added a CHANGELOG.
- Added additional commands/options from [@Yomguithereal](https://github.com/Yomguithereal) 
[xsv fork](https://github.com/Yomguithereal/xsv).
  * `apply` - Apply series of string transformations to a CSV column.
  * `behead` - Drop headers from CSV file.
  * `enum` - Add a new column enumerating rows by adding a column of incremental or 
  uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.
  * `explode` - Explode rows into multiple ones by splitting a column value based on the given separator.
  * `foreach` - Loop over a CSV file to execute bash commands.
  * `jsonl` - Convert newline-delimited JSON to CSV.
  * `lua` - Execute a Lua script over CSV lines to transform, aggregate or filter them.
  * `pseudo` -  Pseudonymise the value of the given column by replacing them by an incremental identifier.
  * `py` - Evaluate a Python expression over CSV lines to transform, aggregate or filter them.
  * `replace` - Replace CSV data using a regex.
  * `sort` --uniq option - When set, identical consecutive lines will be dropped to keep only one line 
  per sorted value.
  * `search` --flag `column` option -  If given, the command will not filter rows but will instead flag 
  the found rows in a new column named `column`.

- Added conditional compilation logic for `foreach` command to only 
compile on `target_family=unix` as it has a dependency on 
`std::os::unix::ffi::OsStrExt` which only works in unix-like OSes.
- Added `empty0` and `emptyNA` operations to `apply` command with 
corresponding test cases.
- Added GitHub Actions to check builds on `ubuntu-latest`, 
`windows-latest` and `macos-latest`.
- Added GitHub Action to publish binaries on release.
- Added `build.rs` build-dependency to check that Rust is at least 
at version 1.50.0 and above.

### Changed
- reformatted README listing of commands to use a table, and to link to
corresponding help text.

### Removed
- Removed appveyor.yml as qsv now uses GitHub Actions.

## [0.15.0] - 2021-09-22
### Added
- `dedup` cmd from [@ronohm](https://github.com/ronohm).
- `table` cmd `--align` option from [@alex-ozdemir](https://github.com/alex-ozdemir).
- `fmt` cmd `--quote-never` option from [@niladic](https://github.com/niladic).
- `exclude` cmd from [@lalaithion](https://github.com/lalaithion)
- Added `--dupes-output` option to `dedup` cmd.
- Added datetime type detection to `stats` cmd.
- Added datetime `min/max` calculation to `stats` cmd.
- es-ES translation from [@ZeliosAriex](https://github.com/ZeliosAriex).

### Changed
- Updated benchmarks script.
- Updated whirlwind tour to include additional commands.
- Made whirlwind tour reproducible by using `sample` `--seed` option.

### Fixed
- Fixed `sample` percentage sampling to be always reproducible even if
sample size < 10% when using `--seed` option.
- Fixed BOM issue with tests, leveraging [unreleased xsv fix](https://github.com/BurntSushi/xsv/commit/a1165e0fe58e6e39f6ed8b1a67ca87dd966c0df3).
- Fixed count help text typo.

### Removed
- Removed `session.vim` file.

## [0.14.1] - 2021-09-15
### Changed
- Performance: enabled link-time optimization (`LTO="fat"`).
- Performance: used code generation units.
- Performance: used [mimalloc](https://docs.rs/mimalloc/0.1.26/mimalloc/) allocator.
- Changed benchmark to compare xsv 0.13.0 and qsv.
- Changed chart from png to svg.
- Performance: Added note in README on how to optimize local compile 
by setting `target-cpu=native`.

## [0.14.0] - 2021-09-14
### Changed
- Renamed fork to qsv.
- Revised highlight note explaining reason for qsv renamed fork in README.
- Added **(NEW)** and **(EXPANDED)** notations to command listing.
- Adapted to Rust 2018 edition.
- used serde derive feature.

## [0.13.1] - 2020-12-27
Initial fork from xsv.
### Added
- `rename` cmd from [@Kerollmops](https://github.com/Kerollmops).
- `fill` cmd from [@alexrudy](https://github.com/alexrudy).
- `transpose` cmd from [@mintyplanet](https://github.com/mintyplanet).
- `select` cmd regex support from [@sd2k](https://github.com/sd2k).
- `stats` cmd `--nullcount` option from [@scpike](https://github.com/scpike).
- added percentage sampling to `sample` cmd.

### Changed
- Updated README with additional commands.
