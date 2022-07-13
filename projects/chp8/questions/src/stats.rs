use std::collections::HashMap;
pub fn median(v: &mut Vec<i32>) -> f64{
    v.sort_unstable();
    let n = v.len();
    return {
        match n%2 {
            0 => (0.5 * (v[n/2] + v[n/2 - 1]) as f64),
            1 => (v[n/2] as f64),
            _ => 1.0,
        }
    }
}

pub fn mode(v: &mut Vec<i32>) -> (i32, i32){
    let mut map = HashMap::new();
    let mut ans = (0, 0);
    for n in v{
        let count = map.entry(*n).or_insert(0);
        *count += 1;
        if *count > ans.1 {
            ans = (*n, *count);
        }
    }
    return ans;
}
