pub mod closures {
    pub mod capturing_references_or_moving_ownership;
    pub mod capturing_the_environment_with_closures;
    pub mod closure_type_inference_and_annotation;
    pub mod example_sort_by_key;
    pub mod filter_a_sequence_with_a_closure;
    pub mod intro;
    pub mod moving_captured_values_out_of_the_closure_and_the_fn_traits;
    pub mod unwrap_or_else;
}

pub mod iterators {
    pub mod capitalize_first;
    pub mod count_progress;
    pub mod factorial;
    pub mod iterable_collection;
    pub mod iterator;
    pub mod loop_over_vector_with_iterators;
    pub mod methods_that_consume_the_iterator;
    pub mod methods_that_produce_other_iterators;
    pub mod processing_a_series_of_items_with_iterators;
    pub mod the_iterator_trait_and_the_next_method;
    pub mod using_closures_that_capture_their_environment;
}

pub mod smart_pointers {
    pub mod box_task;
    pub mod intro;
    pub mod switch_the_lamp;
    pub mod use_cow;
    pub mod use_rc;
    pub mod workers_in_the_engine;
}

pub mod type_conversions {
    pub mod asref_and_asmut;
    pub mod from_into;
    pub mod fromstr;
    pub mod intro;
    pub mod try_from_into;
    pub mod using_as;
}