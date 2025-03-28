use pgx::{iter::TableIterator, *};

use crate::nmost::max_float::toolkit_experimental::*;
use crate::nmost::*;

use crate::{
    build, flatten,
    palloc::{Internal, InternalAsValue, ToInternal},
    pg_type, ron_inout_funcs,
};

use ordered_float::NotNan;
use std::cmp::Reverse;

type MaxByFloatTransType = NMostByTransState<Reverse<NotNan<f64>>>;

#[pg_schema]
pub mod toolkit_experimental {
    use super::*;

    pg_type! {
        #[derive(Debug)]
        struct MaxByFloats<'input> {
            values: MaxFloatsData<'input>,  // Nesting pg_types adds 8 bytes of header
            data: DatumStore<'input>,
        }
    }
    ron_inout_funcs!(MaxByFloats);

    impl<'input> From<MaxByFloatTransType> for MaxByFloats<'input> {
        fn from(item: MaxByFloatTransType) -> Self {
            let (capacity, val_ary, data) = item.into_sorted_parts();
            unsafe {
                flatten!(MaxByFloats {
                    values: build!(MaxFloats {
                        capacity: capacity as u32,
                        elements: val_ary.len() as u32,
                        values: val_ary
                            .into_iter()
                            .map(|x| f64::from(x.0))
                            .collect::<Vec<f64>>()
                            .into()
                    })
                    .0,
                    data,
                })
            }
        }
    }
}

#[pg_extern(schema = "toolkit_experimental", immutable, parallel_safe)]
pub fn max_n_by_float_trans(
    state: Internal,
    value: f64,
    data: AnyElement,
    capacity: i64,
    fcinfo: pg_sys::FunctionCallInfo,
) -> Option<Internal> {
    nmost_by_trans_function(
        unsafe { state.to_inner::<MaxByFloatTransType>() },
        Reverse(NotNan::new(value).unwrap()),
        data,
        capacity as usize,
        fcinfo,
    )
    .internal()
}

#[pg_extern(schema = "toolkit_experimental", immutable, parallel_safe)]
pub fn max_n_by_float_rollup_trans(
    state: Internal,
    value: toolkit_experimental::MaxByFloats<'static>,
    fcinfo: pg_sys::FunctionCallInfo,
) -> Option<Internal> {
    let values: Vec<Reverse<NotNan<f64>>> = value
        .values
        .values
        .clone()
        .into_iter()
        .map(|x| Reverse(NotNan::new(x).unwrap()))
        .collect();
    nmost_by_rollup_trans_function(
        unsafe { state.to_inner::<MaxByFloatTransType>() },
        &values,
        &value.data,
        value.values.capacity as usize,
        fcinfo,
    )
    .internal()
}

#[pg_extern(schema = "toolkit_experimental", immutable, parallel_safe)]
pub fn max_n_by_float_final(state: Internal) -> toolkit_experimental::MaxByFloats<'static> {
    unsafe { state.to_inner::<MaxByFloatTransType>().unwrap().clone() }.into()
}

#[pg_extern(
    schema = "toolkit_experimental",
    name = "into_values",
    immutable,
    parallel_safe
)]
pub fn max_n_by_float_to_values(
    agg: toolkit_experimental::MaxByFloats<'static>,
    _dummy: Option<AnyElement>,
) -> TableIterator<'static, (name!(value, f64), name!(data, AnyElement))> {
    TableIterator::new(
        agg.values
            .values
            .clone()
            .into_iter()
            .zip(agg.data.clone().into_anyelement_iter()),
    )
}

extension_sql!(
    "\n\
    CREATE AGGREGATE toolkit_experimental.max_n_by(\n\
        value double precision, data AnyElement, capacity bigint\n\
    ) (\n\
        sfunc = toolkit_experimental.max_n_by_float_trans,\n\
        stype = internal,\n\
        finalfunc = toolkit_experimental.max_n_by_float_final\n\
    );\n\
",
    name = "max_n_by_float",
    requires = [max_n_by_float_trans, max_n_by_float_final],
);

extension_sql!(
    "\n\
    CREATE AGGREGATE toolkit_experimental.rollup(\n\
        toolkit_experimental.MaxByFloats\n\
    ) (\n\
        sfunc = toolkit_experimental.max_n_by_float_rollup_trans,\n\
        stype = internal,\n\
        finalfunc = toolkit_experimental.max_n_by_float_final\n\
    );\n\
",
    name = "max_n_by_float_rollup",
    requires = [max_n_by_float_rollup_trans, min_n_by_float_final],
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use pgx_macros::pg_test;

    #[pg_test]
    fn max_by_float_correctness() {
        Spi::execute(|client| {
            client.select("SET timezone TO 'UTC'", None, None);
            client.select(
                "CREATE TABLE data(val DOUBLE PRECISION, category INT)",
                None,
                None,
            );

            for i in 0..100 {
                let i = (i * 83) % 100; // mess with the ordering just a little

                client.select(
                    &format!("INSERT INTO data VALUES ({}.0/128, {})", i, i % 4),
                    None,
                    None,
                );
            }

            // Test into_values
            let mut result =
                client.select("SELECT toolkit_experimental.into_values(toolkit_experimental.max_n_by(val, data, 3), NULL::data)::TEXT from data",
                    None, None,
                );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.7734375,\"(0.7734375,3)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.765625,\"(0.765625,2)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.7578125,\"(0.7578125,1)\")")
            );
            assert!(result.next().is_none());

            // Test rollup
            let mut result =
                client.select(
                    "WITH aggs as (SELECT category, toolkit_experimental.max_n_by(val, data, 5) as agg from data GROUP BY category)
                        SELECT toolkit_experimental.into_values(toolkit_experimental.rollup(agg), NULL::data)::TEXT FROM aggs",
                        None, None,
                    );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.7734375,\"(0.7734375,3)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.765625,\"(0.765625,2)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.7578125,\"(0.7578125,1)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.75,\"(0.75,0)\")")
            );
            assert_eq!(
                result.next().unwrap()[1].value(),
                Some("(0.7421875,\"(0.7421875,3)\")")
            );
            assert!(result.next().is_none());
        })
    }
}
