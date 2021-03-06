pub use std::collections::HashSet;

pub use tdd_kata::thread_counting_kata::day_4::{ThreadCounter, Counter};

pub use expectest::prelude::be_equal_to;

describe! thread_counting_tests {

    it "should create a new thread counter" {
        ThreadCounter::new(3, 100);
    }

    it "should count till 100" {
        let thread_counter = ThreadCounter::new(3, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        let data = (1..).take(100).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should count till 200" {
        let thread_counter = ThreadCounter::new(3, 200);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        let data = (1..).take(200).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should count in 3 threads" {
        let thread_counter = ThreadCounter::new(3, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(3));
    }

    it "should count in 5 threads" {
        let thread_counter = ThreadCounter::new(5, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(5));
    }

    it "should count in its thread" {
        let thread_counter = ThreadCounter::new(3, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        let thread_data = (1..).take(100).filter(|n| n % 3 == 1).collect::<HashSet<i32>>();
        expect!(counter.thread_numbers("Thread-1")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|n| n % 3 == 2).collect::<HashSet<i32>>();
        expect!(counter.thread_numbers("Thread-2")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|n| n % 3 == 0).collect::<HashSet<i32>>();
        expect!(counter.thread_numbers("Thread-3")).to(be_equal_to(thread_data));
    }
}
