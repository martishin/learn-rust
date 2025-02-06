pub mod shared_state_concurrency {
    pub mod arc;
    pub mod atomic_reference_counting_with_arc;
    pub mod intro;
    pub mod multiple_ownership_with_multiple_threads;
    pub mod sharing_a_mutex_between_multiple_threads;
    pub mod similarities_between_refcell_rc_and_mutex_arc;
    pub mod the_api_of_mutex;
    pub mod threads_task;
    pub mod using_mutexes;
}

pub mod using_message_passing_to_transfer_data_between_threads {
    pub mod channels_and_ownership_transference;
    pub mod creating_a_channel;
    pub mod creating_multiple_producers_by_cloning_the_transmitter;
    pub mod intro;
    pub mod receiving_values;
    pub mod sending_multiple_values_and_seeing_the_receiver_waiting;
    pub mod sending_values;
    pub mod sum_all_numbers;
}

pub mod using_threads_to_run_code_simultaneously {
    pub mod creating_a_new_thread_with_spawn;
    pub mod intro;
    pub mod join_all_handles;
    pub mod using_move_closures_with_threads;
    pub mod waiting_for_all_threads_to_finish_using_join_handles;
}