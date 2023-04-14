use std::thread;


pub fn split_work<T, R>(mut input: Vec<T>, f: fn(T) -> R, threshold: usize) -> Vec<R>
where
    T: Send + Clone + 'static,
    R: Send + 'static,
{
    println!("Split work called with vector of length {}", input.len());

    if input.len() == 0 {
        return vec![];
    }

    let len = input.len();

    if len <= threshold {
        input.into_iter().map(f).collect()
    } else {
        let num_threads = num_cpus::get();
        let chunk_size = (len + num_threads - 1) / num_threads;

        let handles: Vec<_> = (0..num_threads)
            .map(|i| {
                let chunk_start = i * chunk_size;
                let mut chunk = if chunk_start >= len {
                    Vec::new()
                } else {
                    input.split_off(chunk_start - i)
                };
                

                // проверяем, что вектор не пустой
                if i != 0 || !input.is_empty() {
                    chunk.splice(0..0, input.iter().cloned());
                }

                thread::spawn(move || chunk.into_iter().map(f).collect::<Vec<_>>())
            })
            .collect();

        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .flatten()
            .collect()
    }
}
