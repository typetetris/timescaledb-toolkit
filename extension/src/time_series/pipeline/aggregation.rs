
use std::mem::replace;

use pgx::*;

use super::*;

use crate::{
    ron_inout_funcs, pg_type, build,
    stats_agg::{self, InternalStatsSummary1D, StatsSummary1D},
    counter_agg::{InternalCounterSummary, CounterSummary},
    hyperloglog::HyperLogLog
};


pg_type! {
    #[derive(Debug)]
    struct PipelineThenStatsAgg<'input> {
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenStatsAgg);

// hack to allow us to qualify names with "toolkit_experimental"
// so that pgx generates the correct SQL
pub mod toolkit_experimental {
    pub(crate) use super::*;
    pub(crate) use crate::accessors::*;
    varlena_type!(PipelineThenStatsAgg);
    varlena_type!(PipelineThenSum);
    varlena_type!(PipelineThenAverage);
    varlena_type!(PipelineThenNumVals);
    varlena_type!(PipelineThenCounterAgg);
    varlena_type!(PipelineThenHyperLogLog);
}

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn run_pipeline_then_stats_agg<'s, 'p>(
    mut timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenStatsAgg<'p>,
) -> StatsSummary1D<'static> {
    timevector = run_pipeline_elements(timevector, pipeline.elements.iter());
    let mut stats = InternalStatsSummary1D::new();
    for TSPoint{ val, ..} in timevector.iter() {
        stats.accum(val).expect("error while running stats_agg");
    }
    StatsSummary1D::from_internal(stats)
}

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn finalize_with_stats_agg<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_stats_agg: toolkit_experimental::PipelineThenStatsAgg<'e>,
) -> toolkit_experimental::PipelineThenStatsAgg<'e> {
    if then_stats_agg.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenStatsAgg {
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_stats_agg.elements.iter());
    build! {
        PipelineThenStatsAgg {
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    name="stats_agg",
    schema="toolkit_experimental"
)]
pub fn pipeline_stats_agg<'e>() -> toolkit_experimental::PipelineThenStatsAgg<'e> {
    build! {
        PipelineThenStatsAgg {
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

type Internal = usize;
#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_stats_agg_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenStatsAgg::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_stats_agg(old_pipeline, new_element).into_datum().unwrap()
    })
}

// using this instead of pg_operator since the latter doesn't support schemas yet
// FIXME there is no CREATE OR REPLACE OPERATOR need to update post-install.rs
//       need to ensure this works with out unstable warning
extension_sql!(r#"
ALTER FUNCTION toolkit_experimental."run_pipeline_then_stats_agg" SUPPORT toolkit_experimental.pipeline_stats_agg_support;

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."run_pipeline_then_stats_agg",
    LEFTARG=toolkit_experimental.Timevector,
    RIGHTARG=toolkit_experimental.PipelineThenStatsAgg
);

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."finalize_with_stats_agg",
    LEFTARG=toolkit_experimental.UnstableTimevectorPipeline,
    RIGHTARG=toolkit_experimental.PipelineThenStatsAgg
);
"#);

//
// SUM
//
pg_type! {
    #[derive(Debug)]
    struct PipelineThenSum<'input> {
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenSum);

#[pg_extern(
    immutable,
    parallel_safe,
    name="sum_cast",
    schema="toolkit_experimental"
)]
pub fn sum_pipeline_element<'p, 'e>(
    accessor: toolkit_experimental::AccessorSum<'p>,
) -> toolkit_experimental::PipelineThenSum<'e> {
    let _ = accessor;
    build ! {
        PipelineThenSum {
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

extension_sql!(r#"
    CREATE CAST (toolkit_experimental.AccessorSum AS toolkit_experimental.PipelineThenSum)
        WITH FUNCTION toolkit_experimental.sum_cast
        AS IMPLICIT;
"#);

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn arrow_pipeline_then_sum<'s, 'p>(
    timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenSum<'p>,
) -> Option<f64> {
    let pipeline = pipeline.0;
    let pipeline = build! {
        PipelineThenStatsAgg {
            num_elements: pipeline.num_elements,
            elements: pipeline.elements,
        }
    };
    let stats_agg = run_pipeline_then_stats_agg(timevector, pipeline);
    stats_agg::stats1d_sum(stats_agg)
}

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn finalize_with_sum<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_stats_agg: toolkit_experimental::PipelineThenSum<'e>,
) -> toolkit_experimental::PipelineThenSum<'e> {
    if then_stats_agg.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenSum {
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_stats_agg.elements.iter());
    build! {
        PipelineThenSum {
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_sum_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenSum::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_sum(old_pipeline, new_element).into_datum().unwrap()
    })
}

extension_sql!(r#"
ALTER FUNCTION "arrow_pipeline_then_sum" SUPPORT toolkit_experimental.pipeline_sum_support;
"#);


//
// AVERAGE
//
pg_type! {
    #[derive(Debug)]
    struct PipelineThenAverage<'input> {
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenAverage);

#[pg_extern(
    immutable,
    parallel_safe,
    name="average_cast",
    schema="toolkit_experimental"
)]
pub fn average_pipeline_element<'p, 'e>(
    accessor: toolkit_experimental::AccessorAverage<'p>,
) -> toolkit_experimental::PipelineThenAverage<'e> {
    let _ = accessor;
    build ! {
        PipelineThenAverage {
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

extension_sql!(r#"
    CREATE CAST (toolkit_experimental.AccessorAverage AS toolkit_experimental.PipelineThenAverage)
        WITH FUNCTION toolkit_experimental.average_cast
        AS IMPLICIT;
"#);

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn arrow_pipeline_then_average<'s, 'p>(
    timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenAverage<'p>,
) -> Option<f64> {
    let pipeline = pipeline.0;
    let pipeline = build! {
        PipelineThenStatsAgg {
            num_elements: pipeline.num_elements,
            elements: pipeline.elements,
        }
    };
    let stats_agg = run_pipeline_then_stats_agg(timevector, pipeline);
    stats_agg::stats1d_average(stats_agg)
}

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn finalize_with_average<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_stats_agg: toolkit_experimental::PipelineThenAverage<'e>,
) -> toolkit_experimental::PipelineThenAverage<'e> {
    if then_stats_agg.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenAverage {
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_stats_agg.elements.iter());
    build! {
        PipelineThenAverage {
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_average_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenAverage::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_average(old_pipeline, new_element).into_datum().unwrap()
    })
}

extension_sql!(r#"
ALTER FUNCTION "arrow_pipeline_then_average" SUPPORT toolkit_experimental.pipeline_average_support;
"#);


//
// NUM_VALS
//
pg_type! {
    #[derive(Debug)]
    struct PipelineThenNumVals<'input> {
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenNumVals);

#[pg_extern(
    immutable,
    parallel_safe,
    name="num_vals_cast",
    schema="toolkit_experimental"
)]
pub fn num_vals_pipeline_element<'p, 'e>(
    accessor: toolkit_experimental::AccessorNumVals<'p>,
) -> toolkit_experimental::PipelineThenNumVals<'e> {
    let _ = accessor;
    build ! {
        PipelineThenNumVals {
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

extension_sql!(r#"
    CREATE CAST (toolkit_experimental.AccessorNumVals AS toolkit_experimental.PipelineThenNumVals)
        WITH FUNCTION toolkit_experimental.num_vals_cast
        AS IMPLICIT;
"#);

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn arrow_pipeline_then_num_vals<'s, 'p>(
    timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenNumVals<'p>,
) -> i64 {
    run_pipeline_elements(timevector, pipeline.elements.iter())
        .num_vals() as _
}

#[pg_operator(immutable, parallel_safe)]
#[opname(->)]
pub fn finalize_with_num_vals<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_stats_agg: toolkit_experimental::PipelineThenNumVals<'e>,
) -> toolkit_experimental::PipelineThenNumVals<'e> {
    if then_stats_agg.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenNumVals {
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_stats_agg.elements.iter());
    build! {
        PipelineThenNumVals {
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_num_vals_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenNumVals::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_num_vals(old_pipeline, new_element).into_datum().unwrap()
    })
}

extension_sql!(r#"
ALTER FUNCTION "arrow_pipeline_then_num_vals" SUPPORT toolkit_experimental.pipeline_num_vals_support;
"#);

pg_type! {
    #[derive(Debug)]
    struct PipelineThenCounterAgg<'input> {
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenCounterAgg);

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn run_pipeline_then_counter_agg<'s, 'p>(
    mut timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenCounterAgg<'p>,
) -> Option<CounterSummary<'static>> {
    timevector = run_pipeline_elements(timevector, pipeline.elements.iter());
    if timevector.num_points() == 0 {
        return None
    }
    let mut it = timevector.iter();
    let mut summary = InternalCounterSummary::new(&it.next().unwrap(), None);
    for point in it {
        summary.add_point(&point).expect("error while running counter_agg");
    }
    Some(CounterSummary::from_internal_counter_summary(summary))
}

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn finalize_with_counter_agg<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_counter_agg: toolkit_experimental::PipelineThenCounterAgg<'e>,
) -> toolkit_experimental::PipelineThenCounterAgg<'e> {
    if then_counter_agg.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenCounterAgg {
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_counter_agg.elements.iter());
    build! {
        PipelineThenCounterAgg {
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    name="counter_agg",
    schema="toolkit_experimental"
)]
pub fn pipeline_counter_agg<'e>() -> toolkit_experimental::PipelineThenCounterAgg<'e> {
    build! {
        PipelineThenCounterAgg {
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_counter_agg_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenCounterAgg::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_counter_agg(old_pipeline, new_element).into_datum().unwrap()
    })
}

// using this instead of pg_operator since the latter doesn't support schemas yet
// FIXME there is no CREATE OR REPLACE OPERATOR need to update post-install.rs
//       need to ensure this works with out unstable warning
extension_sql!(r#"
ALTER FUNCTION toolkit_experimental."run_pipeline_then_counter_agg" SUPPORT toolkit_experimental.pipeline_counter_agg_support;

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."run_pipeline_then_counter_agg",
    LEFTARG=toolkit_experimental.Timevector,
    RIGHTARG=toolkit_experimental.PipelineThenCounterAgg
);

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."finalize_with_counter_agg",
    LEFTARG=toolkit_experimental.UnstableTimevectorPipeline,
    RIGHTARG=toolkit_experimental.PipelineThenCounterAgg
);
"#);

pg_type! {
    #[derive(Debug)]
    struct PipelineThenHyperLogLog<'input> {
        hll_size: u64,
        num_elements: u64,
        elements: [Element; self.num_elements],
    }
}

ron_inout_funcs!(PipelineThenHyperLogLog);

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn run_pipeline_then_hyperloglog<'s, 'p>(
    mut timevector: toolkit_experimental::Timevector<'s>,
    pipeline: toolkit_experimental::PipelineThenHyperLogLog<'p>,
) -> HyperLogLog<'static> {
    unsafe {
        timevector = run_pipeline_elements(timevector, pipeline.elements.iter());
        HyperLogLog::build_from(pipeline.hll_size as i32,
            PgBuiltInOids::FLOAT8OID as u32,
            None,
            timevector.iter().map(|point| point.val.into_datum().unwrap())
        )
    }
}

#[pg_extern(immutable, parallel_safe, schema="toolkit_experimental")]
pub fn finalize_with_hyperloglog<'p, 'e>(
    mut pipeline: toolkit_experimental::UnstableTimevectorPipeline<'p>,
    then_hyperloglog: toolkit_experimental::PipelineThenHyperLogLog<'e>,
) -> toolkit_experimental::PipelineThenHyperLogLog<'e> {
    if then_hyperloglog.num_elements == 0 {
        // flatten immediately so we don't need a temporary allocation for elements
        return unsafe {flatten! {
            PipelineThenHyperLogLog {
                hll_size: then_hyperloglog.hll_size,
                num_elements: pipeline.0.num_elements,
                elements: pipeline.0.elements,
            }
        }}
    }

    let mut elements = replace(pipeline.elements.as_owned(), vec![]);
    elements.extend(then_hyperloglog.elements.iter());
    build! {
        PipelineThenHyperLogLog {
            hll_size: then_hyperloglog.hll_size,
            num_elements: elements.len().try_into().unwrap(),
            elements: elements.into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    name="hyperloglog",
    schema="toolkit_experimental"
)]
pub fn pipeline_hyperloglog<'e>(size: i32) -> toolkit_experimental::PipelineThenHyperLogLog<'e> {
    build! {
        PipelineThenHyperLogLog {
            hll_size: size as u64,
            num_elements: 0,
            elements: vec![].into(),
        }
    }
}

#[pg_extern(
    immutable,
    parallel_safe,
    schema="toolkit_experimental"
)]
pub unsafe fn pipeline_hyperloglog_support(input: Internal)
-> Internal {
    pipeline_support_helper(input, |old_pipeline, new_element| unsafe {
        let new_element = PipelineThenHyperLogLog::from_datum(new_element, false, 0)
            .unwrap();
        finalize_with_hyperloglog(old_pipeline, new_element).into_datum().unwrap()
    })
}

// using this instead of pg_operator since the latter doesn't support schemas yet
// FIXME there is no CREATE OR REPLACE OPERATOR need to update post-install.rs
//       need to ensure this works with out unstable warning
extension_sql!(r#"
ALTER FUNCTION toolkit_experimental."run_pipeline_then_hyperloglog" SUPPORT toolkit_experimental.pipeline_hyperloglog_support;

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."run_pipeline_then_hyperloglog",
    LEFTARG=toolkit_experimental.Timevector,
    RIGHTARG=toolkit_experimental.PipelineThenHyperLogLog
);

CREATE OPERATOR -> (
    PROCEDURE=toolkit_experimental."finalize_with_hyperloglog",
    LEFTARG=toolkit_experimental.UnstableTimevectorPipeline,
    RIGHTARG=toolkit_experimental.PipelineThenHyperLogLog
);
"#);

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_stats_agg_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
                (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 25.0), \
                    ('2020-01-01 UTC'::TIMESTAMPTZ, 10.0), \
                    ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                    ('2020-01-02 UTC'::TIMESTAMPTZ, 15.0), \
                    ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> stats_agg())::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
            assert_eq!(val.unwrap(), "(version:1,n:5,sx:100,sx2:250,sx3:0,sx4:21250)");
        });
    }

    #[pg_test]
    fn test_stats_agg_pipeline_folding() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // `-> series()` should force materialization, but otherwise the
            // pipeline-folding optimization should proceed
            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> stats_agg() -> average();",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: (\
                run_pipeline_then_stats_agg(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethenstatsagg\
                ) -> '(version:1)'::accessoraverage)");
        });
    }


    #[pg_test]
    fn test_sum_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
                (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 25.0), \
                    ('2020-01-01 UTC'::TIMESTAMPTZ, 10.0), \
                    ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                    ('2020-01-02 UTC'::TIMESTAMPTZ, 15.0), \
                    ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> sum())::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
                assert_eq!(val.unwrap(), "100");
        });
    }

    #[pg_test]
    fn test_sum_pipeline_folding() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // `-> series()` should force materialization, but otherwise the
            // pipeline-folding optimization should proceed
            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> sum();",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: \
                arrow_pipeline_then_sum(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethensum\
                )");
        });
    }

    #[pg_test]
    fn test_average_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
                (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 25.0), \
                    ('2020-01-01 UTC'::TIMESTAMPTZ, 10.0), \
                    ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                    ('2020-01-02 UTC'::TIMESTAMPTZ, 15.0), \
                    ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> average())::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
            assert_eq!(val.unwrap(), "20");
        });
    }

    #[pg_test]
    fn test_average_pipeline_folding() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // `-> series()` should force materialization, but otherwise the
            // pipeline-folding optimization should proceed
            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> average();",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: \
                arrow_pipeline_then_average(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethenaverage\
                )");
        });
    }

    #[pg_test]
    fn test_num_vals_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
                (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 25.0), \
                    ('2020-01-01 UTC'::TIMESTAMPTZ, 10.0), \
                    ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                    ('2020-01-02 UTC'::TIMESTAMPTZ, 15.0), \
                    ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> num_vals())::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
            assert_eq!(val.unwrap(), "5");
        });
    }

    #[pg_test]
    fn test_num_vals_pipeline_folding() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // `-> series()` should force materialization, but otherwise the
            // pipeline-folding optimization should proceed
            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> num_vals();",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: \
                arrow_pipeline_then_num_vals(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethennumvals\
                )");
        });
    }

    #[pg_test]
    fn test_counter_agg_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
            (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 10.0), \
                ('2020-01-01 UTC'::TIMESTAMPTZ, 15.0), \
                ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                ('2020-01-02 UTC'::TIMESTAMPTZ, 25.0), \
                ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> sort() -> counter_agg())::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
            assert_eq!(val.unwrap(), "(version:1,stats:(n:5,sx:3156624000,sx2:74649600000,sx3:0,sx4:1894671345254400000000,sy:215,sy2:2280,sy3:6720.000000000007,sy4:1788960,sxy:12960000),first:(ts:\"2020-01-01 00:00:00+00\",val:15),second:(ts:\"2020-01-02 00:00:00+00\",val:25),penultimate:(ts:\"2020-01-04 00:00:00+00\",val:10),last:(ts:\"2020-01-05 00:00:00+00\",val:30),reset_sum:45,num_resets:2,num_changes:4,bounds:(is_present:0,has_left:0,has_right:0,padding:(0,0,0,0,0),left:None,right:None))");

            let val = client.select(
                &format!("SELECT series -> sort() -> counter_agg() -> with_bounds('[2020-01-01 UTC, 2020-02-01 UTC)') -> extrapolated_delta('prometheus') FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<f64>().unwrap();
            assert_eq!(val, 67.5);


            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> counter_agg();",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: \
                run_pipeline_then_counter_agg(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethencounteragg\
                )");
        })
    }

    #[pg_test]
    fn test_hyperloglog_finalizer() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            // using the search path trick for this test b/c the operator is
            // difficult to spot otherwise.
            let sp = client.select("SELECT format(' %s, toolkit_experimental',current_setting('search_path'))", None, None).first().get_one::<String>().unwrap();
            client.select(&format!("SET LOCAL search_path TO {}", sp), None, None);
            client.select("SET timescaledb_toolkit_acknowledge_auto_drop TO 'true'", None, None);

            // we use a subselect to guarantee order
            let create_series = "SELECT timevector(time, value) as series FROM \
            (VALUES ('2020-01-04 UTC'::TIMESTAMPTZ, 10.0), \
                ('2020-01-01 UTC'::TIMESTAMPTZ, 15.0), \
                ('2020-01-03 UTC'::TIMESTAMPTZ, 20.0), \
                ('2020-01-02 UTC'::TIMESTAMPTZ, 25.0), \
                ('2020-01-05 UTC'::TIMESTAMPTZ, 30.0), \
                ('2020-01-06 UTC'::TIMESTAMPTZ, 25.0), \
                ('2020-01-07 UTC'::TIMESTAMPTZ, 15.0), \
                ('2020-01-08 UTC'::TIMESTAMPTZ, 35.0), \
                ('2020-01-09 UTC'::TIMESTAMPTZ, 10.0), \
                ('2020-01-10 UTC'::TIMESTAMPTZ, 5.0)) as v(time, value)";

            let val = client.select(
                &format!("SELECT (series -> hyperloglog(100))::TEXT FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<String>();
            assert_eq!(val.unwrap(), "(version:1,log:Sparse(num_compressed:7,element_type:FLOAT8,collation:None,compressed_bytes:28,precision:7,compressed:[136,188,20,7,8,30,244,43,72,69,89,2,72,255,97,27,72,83,248,27,200,110,35,5,8,37,85,12]))");

            let val = client.select(
                &format!("SELECT series -> hyperloglog(100) -> distinct_count() FROM ({}) s", create_series),
                None,
                None
            )
                .first()
                .get_one::<i32>().unwrap();
            assert_eq!(val, 7);

            let output = client.select(
                "EXPLAIN (verbose) SELECT \
                timevector('1930-04-05'::timestamptz, 123.0) \
                -> ceil() -> abs() -> floor() \
                -> hyperloglog(100);",
                None,
                None
            ).skip(1)
                .next().unwrap()
                .by_ordinal(1).unwrap()
                .value::<String>().unwrap();
            assert_eq!(output.trim(), "Output: \
                run_pipeline_then_hyperloglog(\
                    timevector('1930-04-05 00:00:00+00'::timestamp with time zone, '123'::double precision), \
                    '(version:1,hll_size:100,num_elements:3,elements:[\
                        Arithmetic(function:Ceil,rhs:0),\
                        Arithmetic(function:Abs,rhs:0),\
                        Arithmetic(function:Floor,rhs:0)\
                    ])'::pipelinethenhyperloglog\
                )");
        })
    }
}