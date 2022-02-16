// divides `number` into `into` integer parts the most evenly possible
pub fn split_evenly(number: usize, into: usize) -> Vec<usize> {
    // steps from 0 to `number`, separated by `number`/`into`
    // rounded to the nearest integer
    let mut steps: Vec<usize> = Vec::new();
    for i in 0..(into + 1) {
        steps.push((i as f32 * (number as f32 / into as f32)).round() as usize);
    }

    // what we want is the difference between each step
    let mut deltas = Vec::new();
    for i in 0..into {
        deltas.push(steps[i + 1] - steps[i]);
    }

    return deltas;
}
