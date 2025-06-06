// This file serves as the canonical database for what functionality Toolkit has
// stabilized and in which version they were stabilized. The file is consumed by
// by post-install as well as a number of tests that check if our stabilization
// guarantees are being upheld. These different usages require different views
// of the same info, so to avoid parsing issues the stabilization data is
// exposed as macros that're left to the other files to interpret.
//
// XXX this file is used as multiple modules. Search for `#[path = "..."]`
//     directives before adding new macros to make sure that all relevant usages
//     can handle it.

crate::functions_stabilized_at! {
    STABLE_FUNCTIONS
    "1.12.0" => {
        stats1d_tf_inv_trans(internal,double precision),
        stats1d_tf_final(internal),
        stats1d_tf_trans(internal,double precision),
        stats2d_tf_final(internal),
        stats2d_tf_trans(internal,double precision,double precision),
        stats2d_tf_inv_trans(internal,double precision,double precision),
    }
    "1.11.0" => {
        accessorfirsttime_in(cstring),
        accessorfirsttime_out(accessorfirsttime),
        accessorfirstval_in(cstring),
        accessorfirstval_out(accessorfirstval),
        accessorlasttime_in(cstring),
        accessorlasttime_out(accessorlasttime),
        accessorlastval_in(cstring),
        accessorlastval_out(accessorlastval),
        arrow_counter_agg_first_time(countersummary,accessorfirsttime),
        arrow_counter_agg_first_val(countersummary,accessorfirstval),
        arrow_counter_agg_last_time(countersummary,accessorlasttime),
        arrow_counter_agg_last_val(countersummary,accessorlastval),
        arrow_time_weight_first_time(timeweightsummary,accessorfirsttime),
        arrow_time_weight_first_val(timeweightsummary,accessorfirstval),
        arrow_time_weight_last_time(timeweightsummary,accessorlasttime),
        arrow_time_weight_last_val(timeweightsummary,accessorlastval),
        first_time(),
        first_time(countersummary),
        first_time(timeweightsummary),
        first_val(),
        first_val(countersummary),
        first_val(timeweightsummary),
        last_time(),
        last_time(countersummary),
        last_time(timeweightsummary),
        last_val(),
        last_val(countersummary),
        last_val(timeweightsummary),
        asap_final(internal),
        asap_smooth(timestamp with time zone,double precision,integer),
        asap_smooth(timevector_tstz_f64,integer),
        asap_trans(internal,timestamp with time zone,double precision,integer),
    }
    "1.9.0" => {
        accessorapproxpercentile_in(cstring),
        accessorapproxpercentile_out(accessorapproxpercentile),
        accessorapproxpercentilerank_in(cstring),
        accessorapproxpercentilerank_out(accessorapproxpercentilerank),
        accessoraverage_in(cstring),
        accessoraverage_out(accessoraverage),
        accessoraveragex_in(cstring),
        accessoraveragex_out(accessoraveragex),
        accessoraveragey_in(cstring),
        accessoraveragey_out(accessoraveragey),
        accessorcorr_in(cstring),
        accessorcorr_out(accessorcorr),
        accessorcounterzerotime_in(cstring),
        accessorcounterzerotime_out(accessorcounterzerotime),
        accessorcovar_in(cstring),
        accessorcovar_out(accessorcovar),
        accessordelta_in(cstring),
        accessordelta_out(accessordelta),
        accessordeterminationcoeff_in(cstring),
        accessordeterminationcoeff_out(accessordeterminationcoeff),
        accessordistinctcount_in(cstring),
        accessordistinctcount_out(accessordistinctcount),
        accessorerror_in(cstring),
        accessorerror_out(accessorerror),
        accessorextrapolateddelta_in(cstring),
        accessorextrapolateddelta_out(accessorextrapolateddelta),
        accessorextrapolatedrate_in(cstring),
        accessorextrapolatedrate_out(accessorextrapolatedrate),
        accessorideltaleft_in(cstring),
        accessorideltaleft_out(accessorideltaleft),
        accessorideltaright_in(cstring),
        accessorideltaright_out(accessorideltaright),
        accessorintercept_in(cstring),
        accessorintercept_out(accessorintercept),
        accessorirateleft_in(cstring),
        accessorirateleft_out(accessorirateleft),
        accessorirateright_in(cstring),
        accessorirateright_out(accessorirateright),
        accessorkurtosis_in(cstring),
        accessorkurtosis_out(accessorkurtosis),
        accessorkurtosisx_in(cstring),
        accessorkurtosisx_out(accessorkurtosisx),
        accessorkurtosisy_in(cstring),
        accessorkurtosisy_out(accessorkurtosisy),
        accessormaxval_in(cstring),
        accessormaxval_out(accessormaxval),
        accessormean_in(cstring),
        accessormean_out(accessormean),
        accessorminval_in(cstring),
        accessorminval_out(accessorminval),
        accessornumchanges_in(cstring),
        accessornumchanges_out(accessornumchanges),
        accessornumelements_in(cstring),
        accessornumelements_out(accessornumelements),
        accessornumresets_in(cstring),
        accessornumresets_out(accessornumresets),
        accessornumvals_in(cstring),
        accessornumvals_out(accessornumvals),
        accessorrate_in(cstring),
        accessorrate_out(accessorrate),
        accessorskewness_in(cstring),
        accessorskewness_out(accessorskewness),
        accessorskewnessx_in(cstring),
        accessorskewnessx_out(accessorskewnessx),
        accessorskewnessy_in(cstring),
        accessorskewnessy_out(accessorskewnessy),
        accessorslope_in(cstring),
        accessorslope_out(accessorslope),
        accessorstddev_in(cstring),
        accessorstddev_out(accessorstddev),
        accessorstddevx_in(cstring),
        accessorstddevx_out(accessorstddevx),
        accessorstddevy_in(cstring),
        accessorstddevy_out(accessorstddevy),
        accessorstderror_in(cstring),
        accessorstderror_out(accessorstderror),
        accessorsum_in(cstring),
        accessorsum_out(accessorsum),
        accessorsumx_in(cstring),
        accessorsumx_out(accessorsumx),
        accessorsumy_in(cstring),
        accessorsumy_out(accessorsumy),
        accessortimedelta_in(cstring),
        accessortimedelta_out(accessortimedelta),
        accessorunnest_in(cstring),
        accessorunnest_out(accessorunnest),
        accessorvariance_in(cstring),
        accessorvariance_out(accessorvariance),
        accessorvariancex_in(cstring),
        accessorvariancex_out(accessorvariancex),
        accessorvariancey_in(cstring),
        accessorvariancey_out(accessorvariancey),
        accessorwithbounds_in(cstring),
        accessorwithbounds_out(accessorwithbounds),
        accessorxintercept_in(cstring),
        accessorxintercept_out(accessorxintercept),
        approx_percentile(double precision),
        approx_percentile_rank(double precision),
        arrow_counter_agg_corr(countersummary,accessorcorr),
        arrow_counter_agg_delta(countersummary,accessordelta),
        arrow_counter_agg_extrapolated_delta(countersummary,accessorextrapolateddelta),
        arrow_counter_agg_extrapolated_rate(countersummary,accessorextrapolatedrate),
        arrow_counter_agg_idelta_left(countersummary,accessorideltaleft),
        arrow_counter_agg_idelta_right(countersummary,accessorideltaright),
        arrow_counter_agg_intercept(countersummary,accessorintercept),
        arrow_counter_agg_irate_left(countersummary,accessorirateleft),
        arrow_counter_agg_irate_right(countersummary,accessorirateright),
        arrow_counter_agg_num_changes(countersummary,accessornumchanges),
        arrow_counter_agg_num_elements(countersummary,accessornumelements),
        arrow_counter_agg_num_resets(countersummary,accessornumresets),
        arrow_counter_agg_rate(countersummary,accessorrate),
        arrow_counter_agg_slope(countersummary,accessorslope),
        arrow_counter_agg_time_delta(countersummary,accessortimedelta),
        arrow_counter_agg_with_bounds(countersummary,accessorwithbounds),
        arrow_counter_agg_zero_time(countersummary,accessorcounterzerotime),
        arrow_hyperloglog_count(hyperloglog,accessordistinctcount),
        arrow_hyperloglog_error(hyperloglog,accessorstderror),
        arrow_stats1d_average(statssummary1d,accessoraverage),
        arrow_stats1d_kurtosis(statssummary1d,accessorkurtosis),
        arrow_stats1d_num_vals(statssummary1d,accessornumvals),
        arrow_stats1d_skewness(statssummary1d,accessorskewness),
        arrow_stats1d_stddev(statssummary1d,accessorstddev),
        arrow_stats1d_sum(statssummary1d,accessorsum),
        arrow_stats1d_variance(statssummary1d,accessorvariance),
        arrow_stats2d_average_x(statssummary2d,accessoraveragex),
        arrow_stats2d_average_y(statssummary2d,accessoraveragey),
        arrow_stats2d_corr(statssummary2d,accessorcorr),
        arrow_stats2d_covar(statssummary2d,accessorcovar),
        arrow_stats2d_determination_coeff(statssummary2d,accessordeterminationcoeff),
        arrow_stats2d_intercept(statssummary2d,accessorintercept),
        arrow_stats2d_kurtosis_x(statssummary2d,accessorkurtosisx),
        arrow_stats2d_kurtosis_y(statssummary2d,accessorkurtosisy),
        arrow_stats2d_num_vals(statssummary2d,accessornumvals),
        arrow_stats2d_skewness_x(statssummary2d,accessorskewnessx),
        arrow_stats2d_skewness_y(statssummary2d,accessorskewnessy),
        arrow_stats2d_slope(statssummary2d,accessorslope),
        arrow_stats2d_stdddev_x(statssummary2d,accessorstddevx),
        arrow_stats2d_stdddev_y(statssummary2d,accessorstddevy),
        arrow_stats2d_sum_x(statssummary2d,accessorsumx),
        arrow_stats2d_sum_y(statssummary2d,accessorsumy),
        arrow_stats2d_variance_x(statssummary2d,accessorvariancex),
        arrow_stats2d_variance_y(statssummary2d,accessorvariancey),
        arrow_stats2d_x_intercept(statssummary2d,accessorxintercept),
        arrow_tdigest_approx_percentile(tdigest,accessorapproxpercentile),
        arrow_tdigest_approx_rank(tdigest,accessorapproxpercentilerank),
        arrow_tdigest_max(tdigest,accessormaxval),
        arrow_tdigest_mean(tdigest,accessormean),
        arrow_tdigest_min(tdigest,accessorminval),
        arrow_tdigest_num_vals(tdigest,accessornumvals),
        arrow_time_weighted_average_average(timeweightsummary,accessoraverage),
        arrow_timevector_unnest(timevector_tstz_f64,accessorunnest),
        arrow_uddsketch_approx_percentile(uddsketch,accessorapproxpercentile),
        arrow_uddsketch_approx_rank(uddsketch,accessorapproxpercentilerank),
        arrow_uddsketch_error(uddsketch,accessorerror),
        arrow_uddsketch_mean(uddsketch,accessormean),
        arrow_uddsketch_num_vals(uddsketch,accessornumvals),
        average(),
        average_x(),
        average_y(),
        corr(),
        counter_zero_time(),
        covariance(text),
        delta(),
        determination_coeff(),
        distinct_count(),
        error(),
        extrapolated_delta(text),
        extrapolated_rate(text),
        idelta_left(),
        idelta_right(),
        intercept(),
        irate_left(),
        irate_right(),
        kurtosis(text),
        kurtosis_x(text),
        kurtosis_y(text),
        lttb(timestamp with time zone,double precision,integer),
        lttb(timevector_tstz_f64,integer),
        lttb_final(internal),
        lttb_trans(internal,timestamp with time zone,double precision,integer),
        max_val(),
        mean(),
        min_val(),
        num_changes(),
        num_elements(),
        num_resets(),
        num_vals(),
        rate(),
        rollup(timevector_tstz_f64),
        skewness(text),
        skewness_x(text),
        skewness_y(text),
        slope(),
        stddev(text),
        stddev_x(text),
        stddev_y(text),
        stderror(),
        sum(),
        sum_x(),
        sum_y(),
        time_delta(),
        timevector(timestamp with time zone,double precision),
        timevector_combine(internal,internal),
        timevector_deserialize(bytea,internal),
        timevector_final(internal),
        timevector_serialize(internal),
        timevector_tstz_f64_compound_trans(internal,timevector_tstz_f64),
        timevector_tstz_f64_in(cstring),
        timevector_tstz_f64_out(timevector_tstz_f64),
        timevector_tstz_f64_trans(internal,timestamp with time zone,double precision),
        unnest(),
        unnest(timevector_tstz_f64),
        variance(text),
        variance_x(text),
        variance_y(text),
        with_bounds(tstzrange),
        x_intercept(),
        lttb(timestamp with time zone,double precision,integer),
        lttb(timevector_tstz_f64,integer),
        lttb_final(internal),
        lttb_trans(internal,timestamp with time zone,double precision,integer),
    }
    "1.8.0" => {
    }
    "1.7.0" => {
    }
    "1.6.0" => {
    }
    "1.5" => {
    }
    "prehistory" => {
        approx_percentile(double precision,uddsketch),
        approx_percentile_rank(double precision,uddsketch),
        error(uddsketch),
        mean(uddsketch),
        num_vals(uddsketch),
        percentile_agg(double precision),
        percentile_agg_trans(internal,double precision),
        uddsketch(integer,double precision,double precision),
        rollup(uddsketch),
        uddsketch_combine(internal,internal),
        uddsketch_compound_trans(internal,uddsketch),
        uddsketch_deserialize(bytea,internal),
        uddsketch_final(internal),
        uddsketch_in(cstring),
        uddsketch_out(uddsketch),
        uddsketch_serialize(internal),
        uddsketch_trans(internal,integer,double precision,double precision),
        approx_percentile(double precision,tdigest),
        approx_percentile_rank(double precision,tdigest),
        max_val(tdigest),
        min_val(tdigest),
        mean(tdigest),
        num_vals(tdigest),
        tdigest(integer,double precision),
        rollup(tdigest),
        tdigest_combine(internal,internal),
        tdigest_compound_combine(internal,internal),
        tdigest_compound_deserialize(bytea,internal),
        tdigest_compound_final(internal),
        tdigest_compound_serialize(internal),
        tdigest_compound_trans(internal,tdigest),
        tdigest_deserialize(bytea,internal),
        tdigest_final(internal),
        tdigest_in(cstring),
        tdigest_out(tdigest),
        tdigest_serialize(internal),
        tdigest_trans(internal,integer,double precision),
        average(timeweightsummary),
        time_weight(text,timestamp with time zone,double precision),
        rollup(timeweightsummary),
        time_weight_combine(internal,internal),
        time_weight_final(internal),
        time_weight_summary_trans(internal,timeweightsummary),
        time_weight_trans(internal,text,timestamp with time zone,double precision),
        time_weight_trans_deserialize(bytea,internal),
        time_weight_trans_serialize(internal),
        timeweightsummary_in(cstring),
        timeweightsummary_out(timeweightsummary),
        corr(countersummary),
        counter_agg(timestamp with time zone,double precision),
        counter_agg(timestamp with time zone,double precision,tstzrange),
        counter_agg_combine(internal,internal),
        counter_agg_final(internal),
        counter_agg_summary_trans(internal,countersummary),
        counter_agg_trans(internal,timestamp with time zone,double precision,tstzrange),
        counter_agg_trans_no_bounds(internal,timestamp with time zone,double precision),
        counter_summary_trans_deserialize(bytea,internal),
        counter_summary_trans_serialize(internal),
        counter_zero_time(countersummary),
        countersummary_in(cstring),
        countersummary_out(countersummary),
        delta(countersummary),
        extrapolated_delta(countersummary,text),
        extrapolated_rate(countersummary,text),
        idelta_left(countersummary),
        idelta_right(countersummary),
        intercept(countersummary),
        irate_left(countersummary),
        irate_right(countersummary),
        num_changes(countersummary),
        num_elements(countersummary),
        num_resets(countersummary),
        rate(countersummary),
        rollup(countersummary),
        slope(countersummary),
        time_delta(countersummary),
        with_bounds(countersummary,tstzrange),
        hyperloglog(integer,anyelement),
        hyperloglog_combine(internal,internal),
        hyperloglog_deserialize(bytea,internal),
        hyperloglog_final(internal),
        hyperloglog_in(cstring),
        hyperloglog_out(hyperloglog),
        hyperloglog_serialize(internal),
        hyperloglog_trans(internal,integer,anyelement),
        hyperloglog_union(internal,hyperloglog),
        rollup(hyperloglog),
        stderror(hyperloglog),
        average(statssummary1d),
        average_x(statssummary2d),
        average_y(statssummary2d),
        corr(statssummary2d),
        covariance(statssummary2d,text),
        determination_coeff(statssummary2d),
        intercept(statssummary2d),
        kurtosis(statssummary1d,text),
        kurtosis_x(statssummary2d,text),
        kurtosis_y(statssummary2d,text),
        num_vals(statssummary1d),
        num_vals(statssummary2d),
        rolling(statssummary1d),
        rolling(statssummary2d),
        rollup(statssummary1d),
        rollup(statssummary2d),
        skewness(statssummary1d,text),
        skewness_x(statssummary2d,text),
        skewness_y(statssummary2d,text),
        slope(statssummary2d),
        stats1d_combine(internal,internal),
        stats1d_final(internal),
        stats1d_inv_trans(internal,double precision),
        stats1d_summary_inv_trans(internal,statssummary1d),
        stats1d_summary_trans(internal,statssummary1d),
        stats1d_trans(internal,double precision),
        stats1d_trans_deserialize(bytea,internal),
        stats1d_trans_serialize(internal),
        stats2d_combine(internal,internal),
        stats2d_final(internal),
        stats2d_inv_trans(internal,double precision,double precision),
        stats2d_summary_inv_trans(internal,statssummary2d),
        stats2d_summary_trans(internal,statssummary2d),
        stats2d_trans(internal,double precision,double precision),
        stats2d_trans_deserialize(bytea,internal),
        stats2d_trans_serialize(internal),
        stats_agg(double precision),
        stats_agg(double precision,double precision),
        stats_agg_no_inv(double precision),
        stats_agg_no_inv(double precision,double precision),
        statssummary1d_in(cstring),
        statssummary1d_out(statssummary1d),
        statssummary2d_in(cstring),
        statssummary2d_out(statssummary2d),
        stddev(statssummary1d,text),
        stddev_x(statssummary2d,text),
        stddev_y(statssummary2d,text),
        sum(statssummary1d),
        sum_x(statssummary2d),
        sum_y(statssummary2d),
        variance(statssummary1d,text),
        variance_x(statssummary2d,text),
        variance_y(statssummary2d,text),
        x_intercept(statssummary2d),
        distinct_count(hyperloglog),
    }
}

crate::types_stabilized_at! {
    STABLE_TYPES
    "1.11.0" => {
        accessorfirsttime,
        accessorfirstval,
        accessorlasttime,
        accessorlastval,
    }
    "1.9.0" => {
        accessorapproxpercentile,
        accessorapproxpercentilerank,
        accessoraverage,
        accessoraveragex,
        accessoraveragey,
        accessorcorr,
        accessorcounterzerotime,
        accessorcovar,
        accessordelta,
        accessordeterminationcoeff,
        accessordistinctcount,
        accessorerror,
        accessorextrapolateddelta,
        accessorextrapolatedrate,
        accessorideltaleft,
        accessorideltaright,
        accessorintercept,
        accessorirateleft,
        accessorirateright,
        accessorkurtosis,
        accessorkurtosisx,
        accessorkurtosisy,
        accessormaxval,
        accessormean,
        accessorminval,
        accessornumchanges,
        accessornumelements,
        accessornumresets,
        accessornumvals,
        accessorrate,
        accessorskewness,
        accessorskewnessx,
        accessorskewnessy,
        accessorslope,
        accessorstddev,
        accessorstddevx,
        accessorstddevy,
        accessorstderror,
        accessorsum,
        accessorsumx,
        accessorsumy,
        accessortimedelta,
        accessorunnest,
        accessorvariance,
        accessorvariancex,
        accessorvariancey,
        accessorwithbounds,
        accessorxintercept,
        timevector_tstz_f64,
    }
    "1.8.0" => {
    }
    "1.7.0" => {
    }
    "1.6.0" => {
    }
    "1.5" => {
    }
    "prehistory" => {
        uddsketch,
        tdigest,
        timeweightsummary,
        countersummary,
        hyperloglog,
        statssummary1d,
        statssummary2d,
    }
}

crate::operators_stabilized_at! {
    STABLE_OPERATORS
    "1.11.0" => {
        "->"(countersummary,accessorfirsttime),
        "->"(countersummary,accessorfirstval),
        "->"(countersummary,accessorlasttime),
        "->"(countersummary,accessorlastval),
        "->"(timeweightsummary,accessorfirsttime),
        "->"(timeweightsummary,accessorfirstval),
        "->"(timeweightsummary,accessorlasttime),
        "->"(timeweightsummary,accessorlastval),
    }
    "1.9.0" => {
        "->"(countersummary,accessorcorr),
        "->"(countersummary,accessorcounterzerotime),
        "->"(countersummary,accessordelta),
        "->"(countersummary,accessorextrapolateddelta),
        "->"(countersummary,accessorextrapolatedrate),
        "->"(countersummary,accessorideltaleft),
        "->"(countersummary,accessorideltaright),
        "->"(countersummary,accessorintercept),
        "->"(countersummary,accessorirateleft),
        "->"(countersummary,accessorirateright),
        "->"(countersummary,accessornumchanges),
        "->"(countersummary,accessornumelements),
        "->"(countersummary,accessornumresets),
        "->"(countersummary,accessorrate),
        "->"(countersummary,accessorslope),
        "->"(countersummary,accessortimedelta),
        "->"(countersummary,accessorwithbounds),
        "->"(hyperloglog,accessordistinctcount),
        "->"(hyperloglog,accessorstderror),
        "->"(statssummary1d,accessoraverage),
        "->"(statssummary1d,accessorkurtosis),
        "->"(statssummary1d,accessornumvals),
        "->"(statssummary1d,accessorskewness),
        "->"(statssummary1d,accessorstddev),
        "->"(statssummary1d,accessorsum),
        "->"(statssummary1d,accessorvariance),
        "->"(statssummary2d,accessoraveragex),
        "->"(statssummary2d,accessoraveragey),
        "->"(statssummary2d,accessorcorr),
        "->"(statssummary2d,accessorcovar),
        "->"(statssummary2d,accessordeterminationcoeff),
        "->"(statssummary2d,accessorintercept),
        "->"(statssummary2d,accessorkurtosisx),
        "->"(statssummary2d,accessorkurtosisy),
        "->"(statssummary2d,accessornumvals),
        "->"(statssummary2d,accessorskewnessx),
        "->"(statssummary2d,accessorskewnessy),
        "->"(statssummary2d,accessorslope),
        "->"(statssummary2d,accessorstddevx),
        "->"(statssummary2d,accessorstddevy),
        "->"(statssummary2d,accessorsumx),
        "->"(statssummary2d,accessorsumy),
        "->"(statssummary2d,accessorvariancex),
        "->"(statssummary2d,accessorvariancey),
        "->"(statssummary2d,accessorxintercept),
        "->"(tdigest,accessorapproxpercentile),
        "->"(tdigest,accessorapproxpercentilerank),
        "->"(tdigest,accessormaxval),
        "->"(tdigest,accessormean),
        "->"(tdigest,accessorminval),
        "->"(tdigest,accessornumvals),
        "->"(timevector_tstz_f64,accessorunnest),
        "->"(timeweightsummary,accessoraverage),
        "->"(uddsketch,accessorapproxpercentile),
        "->"(uddsketch,accessorapproxpercentilerank),
        "->"(uddsketch,accessorerror),
        "->"(uddsketch,accessormean),
        "->"(uddsketch,accessornumvals),
    }
    "1.8.0" => {
    }
    "1.7.0" => {
    }
    "1.6.0" => {
    }
    "1.5" => {
    }
    "prehistory" => {
    }
}
