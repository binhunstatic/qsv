static USAGE: &str = r#"
Compute summary statistics & infers data types for each column in a CSV.

Summary statistics includes sum, min/max/range, min/max length, mean, stddev, variance,
nullcount, sparsity, quartiles, interquartile range (IQR), lower/upper fences, skewness, median, 
cardinality, mode/s & antimode/s, and median absolute deviation (MAD). Note that some
statistics requires loading the entire file into memory, so they must be enabled explicitly. 

By default, the following statistics are reported for *every* column in the CSV data:
sum, min/max/range values, min/max length, mean, stddev, variance, nullcount & sparsity.
The default set of statistics corresponds to statistics that can be computed efficiently
on a stream of data (i.e., constant memory) and can work with arbitrarily large CSV files.

The following additional statistics require loading the entire file into memory:
cardinality, mode/antimode, median, MAD, quartiles and its related measures (IQR,
lower/upper fences & skewness).

"Antimode" is the least frequently occurring non-zero value and is the opposite of mode.
It returns "*ALL" if all the values are unique, and only returns a preview of the first
10 antimodes.

If you need all the antimode values of a column, run the `frequency` command with --limit set
to zero. The resulting frequency table will have all the antimode values.

Summary statistics for dates are also computed when --infer-dates is enabled, with DateTime
results in rfc3339 format and Date results in "yyyy-mm-dd" format in the UTC timezone.
Date range, stddev, MAD & IQR are returned in days, not timestamp milliseconds. Date variance
is currently not computed as the current streaming variance algorithm is not well suited to 
unix epoch timestamp values.

Each column's data type is also inferred (NULL, Integer, String, Float, Date & DateTime).
Unlike the sniff command, stats' data type inferences are GUARANTEED, as the entire file
is scanned, and not just sampled.

Note that the Date and DateTime data types are only inferred with the --infer-dates option 
as its an expensive operation to match a date candidate against 19 possible date formats,
with each format, having several variants.

The date formats recognized and its sub-variants along with examples can be found at 
https://github.com/jqnatividad/belt/tree/main/dateparser#accepted-date-formats.

Computing statistics on a large file can be made much faster if you create an index for it
first with 'qsv index' to enable multithreading.

For examples, see the "boston311" test files in https://github.com/jqnatividad/qsv/tree/master/resources/test
and https://github.com/jqnatividad/qsv/blob/f7f9c4297fb3dea685b5d0f631932b6b2ca4a99a/tests/test_stats.rs#L544.

Usage:
    qsv stats [options] [<input>]
    qsv stats --help

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will disable the use of indexing.
    --everything              Show all statistics available.
    --typesonly               Infer data types only and do not compute statistics.
                              Note that if you want to infer dates, you'll still need to use
                              the --infer-dates and --dates-whitelist options.
    --mode                    Show the mode/s & antimode/s. Multimodal-aware.
                              This requires loading all CSV data in memory.
    --cardinality             Show the cardinality.
                              This requires loading all CSV data in memory.
    --median                  Show the median.
                              This requires loading all CSV data in memory.
    --mad                     Shows the median absolute deviation (MAD).
                              This requires loading all CSV data in memory.
    --quartiles               Show the quartiles, the IQR, the lower/upper inner/outer
                              fences and skewness.
                              This requires loading all CSV data in memory.
    --round <decimal_places>  Round statistics to <decimal_places>. Rounding is done following
                              Midpoint Nearest Even (aka "Bankers Rounding") rule.
                              For dates - range, stddev & IQR are always at least 5 decimal places as
                              they are reported in days, and 5 places gives us millisecond precision.
                              [default: 4]
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.
    --infer-dates             Infer date/datetime datatypes. This is an expensive
                              option and should only be used when you know there
                              are date/datetime fields.
                              Also, if timezone is not specified in the data, it'll
                              be set to UTC.
    --dates-whitelist <list>  The case-insensitive patterns to look for when 
                              shortlisting fields for date inferencing.
                              i.e. if the field's name has any of these patterns,
                              it is shortlisted for date inferencing.
                              Set to "all" to inspect ALL fields for
                              date/datetime types. Ignored if --infer-dates is false.
                              [default: date,time,due,open,close,created]
    --prefer-dmy              Parse dates in dmy format. Otherwise, use mdy format.
                              Ignored if --infer-dates is false.
    -j, --jobs <arg>          The number of jobs to run in parallel.
                              This works only when the given CSV has an index.
                              Note that a file handle is opened for each job.
                              When not set, the number of jobs is set to the
                              number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

// DEVELOPER NOTE: stats is heavily optimized and makes extensive use of "unsafe" calls.
// It is a central command, that is used by `schema`/`validate`, `tojsonl` and Datapusher+.
//
// It was the primary reason I created the qsv fork as I needed to do GUARANTEED data type
// inferencing & to compile smart Data Dictionaries in the most performant way possible
// for Datapusher+ (https://github.com/dathere/datapusher-plus).
//
// It underpins the `schema` and `validate` commands - enabling the automatic creation of
// a JSONschema based on a CSV's summary statistics; and use the generated JSONschema to
// quickly validate complex CSVs (NYC's 311 data) at almost 300,000 records/sec.
//
// These "unsafe" calls primarily skip repetitive UTF-8 validation and unneeded bounds checking.
//
// To safeguard against undefined behavior, `stats` is the most extensively tested command,
// with ~470 tests.

use std::{
    borrow::ToOwned,
    default::Default,
    fmt, fs, io,
    iter::repeat,
    str::{self, FromStr},
    sync::atomic::{AtomicBool, Ordering},
};

use itertools::Itertools;
use once_cell::sync::OnceCell;
use qsv_dateparser::parse_with_preference;
use serde::Deserialize;
use stats::{merge_all, Commute, MinMax, OnlineStats, Unsorted};
use threadpool::ThreadPool;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};
use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util, CliResult,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:            Option<String>,
    pub flag_select:          SelectColumns,
    pub flag_everything:      bool,
    pub flag_typesonly:       bool,
    pub flag_mode:            bool,
    pub flag_cardinality:     bool,
    pub flag_median:          bool,
    pub flag_mad:             bool,
    pub flag_quartiles:       bool,
    pub flag_round:           u32,
    pub flag_nulls:           bool,
    pub flag_infer_dates:     bool,
    pub flag_dates_whitelist: String,
    pub flag_prefer_dmy:      bool,
    pub flag_jobs:            Option<usize>,
    pub flag_output:          Option<String>,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
}

static INFER_DATE_FLAGS: once_cell::sync::OnceCell<Vec<bool>> = OnceCell::new();
static DMY_PREFERENCE: AtomicBool = AtomicBool::new(false);
static RECORD_COUNT: once_cell::sync::OnceCell<u64> = OnceCell::new();

// number of milliseconds per day
const MS_IN_DAY: f64 = 86_400_000.0;
// number of decimal places when rounding days
// 5 decimal places give us millisecond precision
const DAY_DECIMAL_PLACES: u32 = 5;

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        args.flag_cardinality = false;
        args.flag_median = false;
        args.flag_quartiles = false;
        args.flag_mad = false;
    }

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let fconfig = args.rconfig();
    let record_count = RECORD_COUNT.get_or_init(|| util::count_rows(&fconfig).unwrap());
    log::info!("scanning {record_count} records...");
    let (headers, stats) = match fconfig.indexed()? {
        None => args.sequential_stats(&args.flag_dates_whitelist),
        Some(idx) => {
            if let Some(num_jobs) = args.flag_jobs {
                if num_jobs == 1 {
                    args.sequential_stats(&args.flag_dates_whitelist)
                } else {
                    args.parallel_stats(&args.flag_dates_whitelist, &idx)
                }
            } else {
                args.parallel_stats(&args.flag_dates_whitelist, &idx)
            }
        }
    }?;
    let stats = args.stats_to_records(stats);

    wtr.write_record(&args.stat_headers())?;
    let fields = headers.iter().zip(stats.into_iter());
    for (i, (header, stat)) in fields.enumerate() {
        let header = if args.flag_no_headers {
            i.to_string().into_bytes()
        } else {
            header.to_vec()
        };
        let stat = stat.iter().map(str::as_bytes);
        wtr.write_record(vec![&*header].into_iter().chain(stat))?;
    }
    wtr.flush()?;
    Ok(())
}

impl Args {
    pub fn sequential_stats(&self, whitelist: &str) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            whitelist,
        )?;

        let stats = self.compute(&sel, rdr.byte_records());
        Ok((headers, stats))
    }

    pub fn parallel_stats(
        &self,
        whitelist: &str,
        idx: &Indexed<fs::File, fs::File>,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx.count() == 0 {
            return self.sequential_stats(whitelist);
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            whitelist,
        )?;

        let chunk_size = util::chunk_size(idx.count() as usize, util::njobs(self.flag_jobs));
        let nchunks = util::num_of_chunks(idx.count() as usize, chunk_size);

        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let (send, recv) = channel::bounded(0);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || unsafe {
                let mut idx = args
                    .rconfig()
                    .indexed()
                    .unwrap_unchecked()
                    .unwrap_unchecked();
                idx.seek((i * chunk_size) as u64).unwrap_unchecked();
                let it = idx.byte_records().take(chunk_size);
                send.send(args.compute(&sel, it)).unwrap_unchecked();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
    }

    pub fn stats_to_records(&self, stats: Vec<Stats>) -> Vec<csv::StringRecord> {
        let round_places = self.flag_round;
        let mut records = Vec::with_capacity(stats.len());
        records.extend(repeat(csv::StringRecord::new()).take(stats.len()));
        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let mut results = Vec::with_capacity(stats.len());
        for mut stat in stats {
            let (send, recv) = channel::bounded(0);
            results.push(recv);
            pool.execute(move || {
                unsafe { send.send(stat.to_record(round_places)).unwrap_unchecked() };
            });
        }
        for (i, recv) in results.into_iter().enumerate() {
            records[i] = unsafe { recv.recv().unwrap_unchecked() };
        }
        records
    }

    #[inline]
    fn compute<I>(&self, sel: &Selection, it: I) -> Vec<Stats>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let mut stats = self.new_stats(sel.len());

        // amortize allocation
        #[allow(unused_assignments)]
        let mut record = csv::ByteRecord::with_capacity(1000, sel.len());
        it.for_each(|row| {
            record = unsafe { row.unwrap_unchecked() };
            sel.select(&record).enumerate().for_each(|(i, field)| {
                unsafe {
                    // we use unchecked here so we skip unnecessary bounds checking
                    stats
                        .get_unchecked_mut(i)
                        .add(field, *INFER_DATE_FLAGS.get_unchecked().get_unchecked(i));
                }
            });
        });
        stats
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?.clone();
        let sel = self.rconfig().selection(&headers)?;
        Ok((sel.select(&headers).collect(), sel))
    }

    pub fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    #[inline]
    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        let mut stats: Vec<Stats> = Vec::with_capacity(record_len);
        stats.extend(
            repeat(Stats::new(WhichStats {
                include_nulls: self.flag_nulls,
                sum:           !self.flag_typesonly,
                range:         !self.flag_typesonly,
                dist:          !self.flag_typesonly,
                cardinality:   self.flag_everything || self.flag_cardinality,
                median:        !self.flag_everything && self.flag_median && !self.flag_quartiles,
                mad:           self.flag_everything || self.flag_mad,
                quartiles:     self.flag_everything || self.flag_quartiles,
                mode:          self.flag_everything || self.flag_mode,
                typesonly:     self.flag_typesonly,
            }))
            .take(record_len),
        );
        stats
    }

    pub fn stat_headers(&self) -> csv::StringRecord {
        if self.flag_typesonly {
            return csv::StringRecord::from(vec!["field", "type"]);
        }

        // with --everything, we have 30 columns at most
        let mut fields = Vec::with_capacity(30);
        fields.extend_from_slice(&[
            "field",
            "type",
            "sum",
            "min",
            "max",
            "range",
            "min_length",
            "max_length",
            "mean",
            "stddev",
            "variance",
            "nullcount",
            "sparsity",
        ]);
        let all = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !all {
            fields.push("median");
        }
        if self.flag_mad || all {
            fields.push("mad");
        }
        if self.flag_quartiles || all {
            fields.extend_from_slice(&[
                "lower_outer_fence",
                "lower_inner_fence",
                "q1",
                "q2_median",
                "q3",
                "iqr",
                "upper_inner_fence",
                "upper_outer_fence",
                "skewness",
            ]);
        }
        if self.flag_cardinality || all {
            fields.push("cardinality");
        }
        if self.flag_mode || all {
            fields.push("mode");
            fields.push("mode_count");
            fields.push("mode_occurrences");
            fields.push("antimode");
            fields.push("antimode_count");
            fields.push("antimode_occurrences");
        }
        csv::StringRecord::from(fields)
    }
}

#[inline]
fn init_date_inference(
    infer_dates: bool,
    prefer_dmy: bool,
    headers: &csv::ByteRecord,
    flag_whitelist: &str,
) -> Result<(), String> {
    if infer_dates {
        let dmy_preferred = prefer_dmy || std::env::var("QSV_PREFER_DMY").is_ok();
        DMY_PREFERENCE.store(dmy_preferred, Ordering::Relaxed);

        let whitelist_lower = flag_whitelist.to_lowercase();
        log::info!("inferring dates with date-whitelist: {whitelist_lower}");

        if whitelist_lower == "all" {
            log::info!("inferring dates for ALL fields with DMY preference: {dmy_preferred}");
            if let Err(e) = INFER_DATE_FLAGS.set(vec![true; headers.len()]) {
                return fail_format!("Cannot init date inference flags for ALL fields: {e:?}");
            };
        } else {
            let whitelist = whitelist_lower
                .split(',')
                .map(|s| s.trim().to_string())
                .collect_vec();

            let mut infer_date_flags: Vec<bool> = Vec::with_capacity(headers.len());
            for header in headers {
                let header_str = from_bytes::<String>(header).to_lowercase();
                let mut date_found = false;
                for whitelist_item in &whitelist {
                    if header_str.contains(whitelist_item) {
                        date_found = true;
                        log::info!(
                            "inferring dates for {header_str} with DMY preference: {dmy_preferred}"
                        );
                        break;
                    }
                }
                infer_date_flags.push(date_found);
            }
            if let Err(e) = INFER_DATE_FLAGS.set(infer_date_flags) {
                return fail_format!("Cannot init date inference flags: {e:?}");
            };
        }
    // we're not inferring dates, set INFER_DATE_FLAGS to all false
    } else if let Err(e) = INFER_DATE_FLAGS.set(vec![false; headers.len()]) {
        return fail_format!("Cannot init empty date inference flags: {e:?}");
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WhichStats {
    include_nulls: bool,
    sum:           bool,
    range:         bool,
    dist:          bool,
    cardinality:   bool,
    median:        bool,
    mad:           bool,
    quartiles:     bool,
    mode:          bool,
    typesonly:     bool,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[derive(Clone)]
pub struct Stats {
    typ:       FieldType,
    sum:       Option<TypedSum>,
    minmax:    Option<TypedMinMax>,
    online:    Option<OnlineStats>,
    nullcount: u64,
    modes:     Option<Unsorted<Vec<u8>>>,
    median:    Option<Unsorted<f64>>,
    mad:       Option<Unsorted<f64>>,
    quartiles: Option<Unsorted<f64>>,
    which:     WhichStats,
}

fn timestamp_ms_to_rfc3339(timestamp: i64, typ: FieldType) -> String {
    use chrono::prelude::*;

    let date_val = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_millis(timestamp).unwrap_or_default(),
        Utc,
    )
    .to_rfc3339();

    // if type = Date, only return the date component
    // do not return the time component
    if typ == TDate {
        return date_val[..10].to_string();
    }
    date_val
}

impl Stats {
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut modes, mut median, mut quartiles, mut mad) =
            (None, None, None, None, None, None, None);
        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
        }
        if which.mode || which.cardinality {
            modes = Some(stats::Unsorted::default());
        }
        if which.quartiles {
            quartiles = Some(stats::Unsorted::default());
        } else if which.median {
            median = Some(stats::Unsorted::default());
        }
        if which.mad {
            mad = Some(stats::Unsorted::default());
        }
        Stats {
            typ: FieldType::default(),
            sum,
            minmax,
            online,
            nullcount: 0,
            modes,
            median,
            mad,
            quartiles,
            which,
        }
    }

    #[inline]
    fn add(&mut self, sample: &[u8], infer_dates: bool) {
        let (sample_type, timestamp_val) = FieldType::from_sample(infer_dates, sample, self.typ);
        self.typ.merge(sample_type);

        // we're inferring typesonly, don't add samples to compute statistics
        if self.which.typesonly {
            return;
        }

        let t = self.typ;
        if let Some(v) = self.sum.as_mut() {
            v.add(t, sample);
        };
        if let Some(v) = self.minmax.as_mut() {
            if let Some(ts_val) = timestamp_val {
                let mut buffer = itoa::Buffer::new();
                v.add(t, buffer.format(ts_val).as_bytes());
            } else {
                v.add(t, sample);
            }
        };
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec());
        };
        if sample_type == TNull {
            self.nullcount += 1;
        }
        match t {
            TNull => {
                if self.which.include_nulls {
                    if let Some(v) = self.online.as_mut() {
                        v.add_null();
                    };
                }
            }
            TFloat | TInteger => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                } else {
                    let n = from_bytes::<f64>(sample);
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(n);
                    }
                }
            }
            TDateTime | TDate => {
                if sample_type == TNull {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                // if ts_val.is_some() then we successfully inferred a date from the sample
                // and the timestamp value is not None
                } else if let Some(ts_val) = timestamp_val {
                    // calculate date statistics by adding date samples as timestamps to
                    // millisecond precision.
                    #[allow(clippy::cast_precision_loss)]
                    let n = ts_val as f64;
                    if let Some(v) = self.median.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.mad.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.quartiles.as_mut() {
                        v.add(n);
                    }
                    if let Some(v) = self.online.as_mut() {
                        v.add(n);
                    }
                }
            }
            // do nothing for String type
            TString => {}
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_record(&mut self, round_places: u32) -> csv::StringRecord {
        // we're doing typesonly
        if self.which.typesonly {
            return csv::StringRecord::from(vec![self.typ.to_string()]);
        }

        let typ = self.typ;
        // prealloc memory for performance
        // we have 30 columns at most with --everything
        let mut pieces = Vec::with_capacity(30);
        let empty = String::new;

        // type
        pieces.push(typ.to_string());

        // sum
        if let Some(sum) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
            if typ == FieldType::TFloat {
                if let Ok(f64_val) = sum.parse::<f64>() {
                    pieces.push(util::round_num(f64_val, round_places));
                } else {
                    pieces.push(format!("ERROR: Cannot convert {sum} to a float."));
                }
            } else {
                pieces.push(sum);
            }
        } else {
            pieces.push(empty());
        }

        // min/max/range
        if let Some(mm) = self
            .minmax
            .as_ref()
            .and_then(|mm| mm.show(typ, round_places))
        {
            pieces.push(mm.0);
            pieces.push(mm.1);
            pieces.push(mm.2);
        } else {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        }

        // min/max length
        if typ == FieldType::TDate || typ == FieldType::TDateTime {
            // returning min/max length for dates doesn't make sense
            // especially since we convert the date stats to rfc3339 format
            pieces.push(empty());
            pieces.push(empty());
        } else if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            pieces.push(mm.0);
            pieces.push(mm.1);
        } else {
            pieces.push(empty());
            pieces.push(empty());
        }

        // mean, stddev & variance
        if typ == TString || typ == TNull {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        } else if let Some(ref v) = self.online {
            if self.typ == TFloat || self.typ == TInteger {
                pieces.push(util::round_num(v.mean(), round_places));
                pieces.push(util::round_num(v.stddev(), round_places));
                pieces.push(util::round_num(v.variance(), round_places));
            } else {
                pieces.push(timestamp_ms_to_rfc3339(v.mean() as i64, typ));
                // instead of returning stdev in seconds, let's return it in
                // days as it easier to handle
                // Round to at least 5 decimal places, so we have millisecond precision
                pieces.push(util::round_num(
                    v.stddev() / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                // we don't know how to compute variance on timestamps
                // it appears the current algorithm we use is not suited to the large timestamp
                // values as the values we got during testing don't make sense, so
                // leave it empty for now
                // TODO: explore alternate algorithms for calculating variance
                // see https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
                pieces.push(empty());
            }
        } else {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        }

        // nullcount
        let mut buffer = itoa::Buffer::new();
        pieces.push(buffer.format(self.nullcount).to_owned());

        // sparsity
        // stats is also called by the `schema` and `tojsonl` commands to infer a schema,
        // sparsity is not required by those cmds and we don't necessarily have the
        // record_count when called by those cmds, so just set sparsity to nullcount
        // (div by 1) so we don't panic.
        #[allow(clippy::cast_precision_loss)]
        let sparsity: f64 = self.nullcount as f64 / *RECORD_COUNT.get().unwrap_or(&1) as f64;
        pieces.push(util::round_num(sparsity, round_places));

        // median
        let mut existing_median = None;
        if let Some(v) = self.median.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                existing_median = v.median();
                existing_median
            }
        }) {
            if typ == TDateTime || typ == TDate {
                pieces.push(timestamp_ms_to_rfc3339(v as i64, typ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.median {
            pieces.push(empty());
        }

        // median absolute deviation (MAD)
        if let Some(v) = self.mad.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else {
                v.mad(existing_median)
            }
        }) {
            if typ == TDateTime || typ == TDate {
                // like stddev, return MAD in days
                pieces.push(util::round_num(
                    v / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.mad {
            pieces.push(empty());
        }

        // quartiles
        match self.quartiles.as_mut().and_then(|v| match typ {
            TInteger | TFloat | TDate | TDateTime => v.quartiles(),
            _ => None,
        }) {
            None => {
                if self.which.quartiles {
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                }
            }
            Some((q1, q2, q3)) => {
                let iqr = q3 - q1;

                // use fused multiply add (mul_add) when possible
                // fused mul_add is more accurate & may be more performant if the
                // target architecture has a dedicated `fma` CPU instruction
                // https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add

                // lower_outer_fence = "q1 - (3.0 * iqr)"
                let lof = 3.0f64.mul_add(-iqr, q1);
                // lower_inner_fence = "q1 - (1.5 * iqr)"
                let lif = 1.5f64.mul_add(-iqr, q1);

                // upper inner fence = "q3 + (1.5 * iqr)"
                let uif = 1.5_f64.mul_add(iqr, q3);
                // upper_outer_fence = "q3 + (3.0 * iqr)"
                let uof = 3.0_f64.mul_add(iqr, q3);

                // calculate skewness using Quantile-based measures
                // https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures
                // https://blogs.sas.com/content/iml/2017/07/19/quantile-skewness.html
                // quantile skewness = ((q3 - q2) - (q2 - q1)) / iqr;
                // which is also (q3 - (2.0 * q2) + q1) / iqr
                // which in turn, is the basis of the fused multiply add version below
                let skewness = (2.0f64.mul_add(-q2, q3) + q1) / iqr;

                if typ == TDateTime || typ == TDate {
                    // casting from f64 to i64 is OK, per
                    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
                    // as values larger/smaller than what i64 can handle will automatically
                    // saturate to i64 max/min values.
                    pieces.push(timestamp_ms_to_rfc3339(lof as i64, typ));
                    pieces.push(timestamp_ms_to_rfc3339(lif as i64, typ));

                    pieces.push(timestamp_ms_to_rfc3339(q1 as i64, typ));
                    pieces.push(timestamp_ms_to_rfc3339(q2 as i64, typ)); // q2 = median
                    pieces.push(timestamp_ms_to_rfc3339(q3 as i64, typ));
                    // return iqr in days - there are 86,400,000 ms in a day
                    pieces.push(util::round_num(
                        (q3 - q1) / MS_IN_DAY,
                        u32::max(round_places, DAY_DECIMAL_PLACES),
                    ));

                    pieces.push(timestamp_ms_to_rfc3339(uif as i64, typ));
                    pieces.push(timestamp_ms_to_rfc3339(uof as i64, typ));
                } else {
                    pieces.push(util::round_num(lof, round_places));
                    pieces.push(util::round_num(lif, round_places));

                    pieces.push(util::round_num(q1, round_places));
                    pieces.push(util::round_num(q2, round_places)); // q2 = median
                    pieces.push(util::round_num(q3, round_places));
                    pieces.push(util::round_num(iqr, round_places));

                    pieces.push(util::round_num(uif, round_places));
                    pieces.push(util::round_num(uof, round_places));
                }
                pieces.push(util::round_num(skewness, round_places));
            }
        }

        // mode/modes & cardinality
        match self.modes.as_mut() {
            None => {
                if self.which.cardinality {
                    pieces.push(empty());
                }
                if self.which.mode {
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                }
            }
            Some(ref mut v) => {
                if self.which.cardinality {
                    let mut buffer = itoa::Buffer::new();
                    pieces.push(buffer.format(v.cardinality()).to_owned());
                }
                if self.which.mode {
                    // mode/s
                    let (modes_result, modes_count, mode_occurrences) = v.modes();
                    let modes_list = modes_result
                        .iter()
                        .map(|c| String::from_utf8_lossy(c))
                        .join(",");
                    pieces.push(modes_list);
                    pieces.push(modes_count.to_string());
                    pieces.push(mode_occurrences.to_string());

                    // antimode/s
                    if mode_occurrences == 0 {
                        // all the values are unique
                        // so instead of returning everything, just say *ALL
                        pieces.push("*ALL".to_string());
                        pieces.push("0".to_string());
                        pieces.push("1".to_string());
                    } else {
                        let (antimodes_result, antimodes_count, antimode_occurrences) =
                            v.antimodes();
                        let mut antimodes_list = String::new();

                        // We only store the first 10 antimodes
                        // so if antimodes_count > 10, add the "*PREVIEW: " prefix
                        if antimodes_count > 10 {
                            antimodes_list.push_str("*PREVIEW: ");
                        }

                        let antimodes_vals = &antimodes_result
                            .iter()
                            .map(|c| String::from_utf8_lossy(c))
                            .join(",");
                        if antimodes_vals.starts_with(',') {
                            antimodes_list.push_str("NULL");
                        }
                        antimodes_list.push_str(antimodes_vals);

                        // and truncate at 100 characters with an ellipsis
                        if antimodes_list.len() > 100 {
                            antimodes_list.truncate(100);
                            antimodes_list.push_str("...");
                        }

                        pieces.push(antimodes_list);
                        pieces.push(antimodes_count.to_string());
                        pieces.push(antimode_occurrences.to_string());
                    }
                }
            }
        }
        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.sum.merge(other.sum);
        self.minmax.merge(other.minmax);
        self.online.merge(other.online);
        self.nullcount += other.nullcount;
        self.modes.merge(other.modes);
        self.median.merge(other.median);
        self.quartiles.merge(other.quartiles);
        self.which.merge(other.which);
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FieldType {
    // The default - TNull, is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    #[default]
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
}

impl FieldType {
    // infer data type
    // infer_dates signals if date inference should be attempted
    // from a given sample & current type inference
    #[inline]
    pub fn from_sample(
        infer_dates: bool,
        sample: &[u8],
        current_type: FieldType,
    ) -> (FieldType, Option<i64>) {
        if sample.is_empty() {
            return (TNull, None);
        }
        // no need to do type checking if current_type is already a String
        if current_type == FieldType::TString {
            return (FieldType::TString, None);
        }

        // we skip utf8 validation since we say we only work with utf8
        let string = unsafe { str::from_utf8_unchecked(sample) };

        if current_type == FieldType::TFloat
            || current_type == FieldType::TInteger
            || current_type == FieldType::TNull
        {
            if let Ok(int_val) = string.parse::<i64>() {
                // leading zero, its a string
                if string.starts_with('0') && int_val != 0 {
                    return (TString, None);
                }
                return (TInteger, None);
            }

            if string.parse::<f64>().is_ok() {
                return (TFloat, None);
            }
        }

        if infer_dates
            && (current_type == FieldType::TDate
                || current_type == FieldType::TDateTime
                || current_type == FieldType::TNull)
        {
            if let Ok(parsed_date) =
                parse_with_preference(string, DMY_PREFERENCE.load(Ordering::Relaxed))
            {
                // get date in rfc3339 format, if it ends with "T00:00:00+00:00"
                // its a Date type, otherwise, its DateTime.
                let ts_val = parsed_date.timestamp_millis();
                if parsed_date.to_rfc3339().ends_with("T00:00:00+00:00") {
                    return (TDate, Some(ts_val));
                }
                return (TDateTime, Some(ts_val));
            }
        }
        (TString, None)
    }
}

impl Commute for FieldType {
    #[inline]
    #[allow(clippy::match_same_arms)]
    // we allow match_same_arms because we want are optimizing for
    // performance and not readability, as match arms are evaluated in order
    // so we want to put the most common cases first
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TString, TString) => TString,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // date data types
            (TDate, TDate) => TDate,
            (TDateTime | TDate, TDateTime) | (TDateTime, TDate) => TDateTime,
            // anything else is a String
            (_, _) => TString,
        };
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

/// `TypedSum` keeps a rolling sum of the data seen.
/// It sums integers until it sees a float, at which point it sums floats.
#[derive(Clone, Default)]
struct TypedSum {
    integer: i64,
    float:   Option<f64>,
}

impl TypedSum {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        if sample.is_empty() {
            return;
        }
        #[allow(clippy::cast_precision_loss)]
        match typ {
            TFloat => {
                let float: f64 = from_bytes::<f64>(sample);
                match self.float {
                    None => {
                        self.float = Some((self.integer as f64) + float);
                    }
                    Some(ref mut f) => {
                        *f += float;
                    }
                }
            }
            TInteger => {
                if let Some(ref mut float) = self.float {
                    *float += from_bytes::<f64>(sample);
                } else {
                    // so we don't panic on overflow/underflow, use saturating_add
                    self.integer = self.integer.saturating_add(from_bytes::<i64>(sample));
                }
            }
            _ => {}
        }
    }

    fn show(&self, typ: FieldType) -> Option<String> {
        match typ {
            TNull | TString | TDate | TDateTime => None,
            TInteger => {
                match self.integer {
                    // with saturating_add, if this is equal to i64::MAX or i64::MIN
                    // we overflowed/underflowed
                    i64::MAX => Some("OVERFLOW".to_string()),
                    i64::MIN => Some("UNDERFLOW".to_string()),
                    _ => {
                        let mut buffer = itoa::Buffer::new();
                        Some(buffer.format(self.integer).to_owned())
                    }
                }
            }
            TFloat => {
                let mut buffer = ryu::Buffer::new();
                Some(buffer.format(self.float.unwrap_or(0.0)).to_owned())
            }
        }
    }
}

impl Commute for TypedSum {
    #[inline]
    fn merge(&mut self, other: TypedSum) {
        #[allow(clippy::cast_precision_loss)]
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer = self.integer.saturating_add(other.integer),
        }
    }
}

/// `TypedMinMax` keeps track of minimum/maximum/range values for each possible type
/// where min/max/range makes sense.
#[derive(Clone, Default)]
struct TypedMinMax {
    strings:  MinMax<Vec<u8>>,
    str_len:  MinMax<usize>,
    integers: MinMax<i64>,
    floats:   MinMax<f64>,
    dates:    MinMax<i64>,
}

impl TypedMinMax {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        self.str_len.add(sample.len());
        if sample.is_empty() {
            return;
        }
        self.strings.add(sample.to_vec());
        // we can use unwrap_unchecked with confidence
        // below since we know the data type domains of the sample
        match typ {
            TString | TNull => {}
            TFloat => {
                let n = unsafe {
                    str::from_utf8_unchecked(sample)
                        .parse::<f64>()
                        .unwrap_unchecked()
                };

                self.floats.add(n);
                self.integers.add(n as i64);
            }
            TInteger => {
                let n = unsafe {
                    str::from_utf8_unchecked(sample)
                        .parse::<i64>()
                        .unwrap_unchecked()
                };
                self.integers.add(n);
                #[allow(clippy::cast_precision_loss)]
                self.floats.add(n as f64);
            }
            TDate | TDateTime => {
                let n = unsafe {
                    str::from_utf8_unchecked(sample)
                        .parse::<i64>()
                        .unwrap_unchecked()
                };
                self.dates.add(n);
            }
        }
    }

    fn len_range(&self) -> Option<(String, String)> {
        if let (Some(min), Some(max)) = (self.str_len.min(), self.str_len.max()) {
            let mut buffer = itoa::Buffer::new();
            Some((
                buffer.format(*min).to_owned(),
                buffer.format(*max).to_owned(),
            ))
        } else {
            None
        }
    }

    fn show(&self, typ: FieldType, round_places: u32) -> Option<(String, String, String)> {
        match typ {
            TNull => None,
            TString => {
                if let (Some(min), Some(max)) = (self.strings.min(), self.strings.max()) {
                    let min = String::from_utf8_lossy(min).to_string();
                    let max = String::from_utf8_lossy(max).to_string();
                    Some((min, max, String::new()))
                } else {
                    None
                }
            }
            TInteger => {
                if let (Some(min), Some(max)) = (self.integers.min(), self.integers.max()) {
                    let mut buffer = itoa::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                        buffer.format(*max - *min).to_owned(),
                    ))
                } else {
                    None
                }
            }
            TFloat => {
                if let (Some(min), Some(max)) = (self.floats.min(), self.floats.max()) {
                    let mut buffer = ryu::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                        util::round_num(*max - *min, round_places),
                    ))
                } else {
                    None
                }
            }
            TDateTime | TDate => {
                if let (Some(min), Some(max)) = (self.dates.min(), self.dates.max()) {
                    Some((
                        timestamp_ms_to_rfc3339(*min, typ),
                        timestamp_ms_to_rfc3339(*max, typ),
                        // return in days, not timestamp in milliseconds
                        #[allow(clippy::cast_precision_loss)]
                        util::round_num(
                            (*max - *min) as f64 / MS_IN_DAY,
                            u32::max(round_places, 5),
                        ),
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl Commute for TypedMinMax {
    #[inline]
    fn merge(&mut self, other: TypedMinMax) {
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
        self.integers.merge(other.integers);
        self.floats.merge(other.floats);
        self.dates.merge(other.dates);
    }
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn from_bytes<T: FromStr>(bytes: &[u8]) -> T {
    // we don't need to do UTF-8 validation as qsv requires UTF-8 encoding
    unsafe { str::from_utf8_unchecked(bytes).parse().unwrap_unchecked() }
}
