pub fn measure_elapsed<F>(func: F)
where
    F: FnOnce() -> (),
{
    unsafe {
        let mut query = 0;
        gl::GenQueries(1, &mut query);
        gl::BeginQuery(gl::TIME_ELAPSED, query);

        use std::time::Instant;
        let now = Instant::now();

        func();

        let cpu_time = now.elapsed().as_micros() as f32 / 1000.;

        gl::EndQuery(gl::TIME_ELAPSED);
        let mut gpu_time = 0;
        gl::GetQueryObjectiv(query, gl::QUERY_RESULT, &mut gpu_time);
        gl::DeleteQueries(1, &query as *const _);

        println!(
            "CPU: {:.2} ms, GPU: {:.2} ms",
            cpu_time,
            gpu_time as f64 / 1_000_000.0
        );
    }
}
