pub mod generic_data_types {
    pub mod generics_in_structs;
    pub mod generics_in_vec;
    pub mod in_enum_definitions;
    pub mod in_function_definitions;
    pub mod in_method_definitions;
    pub mod in_struct_definitions;
    pub mod intro;
    pub mod mixing_generic_types;
    pub mod performance_of_code_using_generics;
}

pub mod intro {
    pub mod code_without_duplications;
    pub mod computing_the_largest_number;
    pub mod duplicated_code_fragments;
    pub mod intro;
    pub mod removing_duplication_by_refactorings;
}

pub mod lifetimes {
    pub mod generic_lifetimes_in_functions;
    pub mod generic_type_parameters_trait_bounds_and_lifetimes_together;
    pub mod intro;
    pub mod lifetime_annotation_syntax;
    pub mod lifetime_annotations_in_function_signatures;
    pub mod lifetime_annotations_in_method_definitions;
    pub mod lifetime_annotations_in_struct_definitions;
    pub mod lifetime_elision;
    pub mod lifetime_going_out_of_scope;
    pub mod lifetimes_for_structs_with_references;
    pub mod pattern_binding;
    pub mod preventing_dangling_references_with_lifetimes;
    pub mod specify_lifetimes;
    pub mod summary;
    pub mod the_borrow_checker;
    pub mod the_static_lifetime;
    pub mod thinking_in_terms_of_lifetimes;
    pub mod unify_lifetimes;
}

pub mod traits {
    pub mod calling_methods_without_default_implementations;
    pub mod combine_traits;
    pub mod compare_licenses;
    pub mod default_implementations;
    pub mod defining_a_trait;
    pub mod fixing_the_largest_function_with_trait_bounds;
    pub mod implementing_a_trait_on_a_type;
    pub mod license_it;
    pub mod returning_types_that_implement_traits;
    pub mod string_trait;
    pub mod trait_bound_syntax;
    pub mod traits;
    pub mod traits_as_parameters;
    pub mod using_trait_bounds_to_conditionally_implement_methods;
    pub mod vec_trait;
}