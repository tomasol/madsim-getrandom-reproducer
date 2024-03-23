#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn madsim_getrandom_should_be_deterministic() {
        let rnd_fn = || async {
            let mut dst = [0];
            getrandom::getrandom(&mut dst).unwrap();
            println!("{dst:?}");
            dst
        };
        let builder = madsim::runtime::Builder::from_env();
        let seed = builder.seed;
        let set = (0..10)
            .map(|_| {
                madsim::runtime::Builder {
                    seed,
                    count: 1,
                    jobs: 1,
                    config: madsim::Config::default(),
                    time_limit: None,
                    check: false,
                }
                .run(rnd_fn)
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), 1);
    }
}
