advent_of_code::solution!(2);

const POW10: [u64; 20] = [1,10,100,1_000,10_000,100_000,1_000_000,10_000_000,100_000_000,1_000_000_000,10_000_000_000,100_000_000_000,1_000_000_000_000,10_000_000_000_000,100_000_000_000_000,1_000_000_000_000_000,10_000_000_000_000_000,100_000_000_000_000_000,1_000_000_000_000_000_000,10_000_000_000_000_000_000];

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, lo, hi) = parse(input)?;
    Some(sum_over_ranges(&mirrored(lo, hi), &ranges))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, lo, hi) = parse(input)?;
    Some(sum_over_ranges(&periodic(lo, hi), &ranges))
}

fn parse(input: &str) -> Option<(Vec<(u64,u64)>, u64, u64)> {
    let s = input.trim(); if s.is_empty() { return None; }
    let mut lo = u64::MAX; let mut hi = 0;
    let ranges = s.split(',').map(|p| {
        let (a,b) = p.split_once('-').unwrap();
        let (mut l, mut r) = (a.parse().unwrap(), b.parse().unwrap());
        if l>r { std::mem::swap(&mut l, &mut r); }
        lo = lo.min(l); hi = hi.max(r); (l,r)
    }).collect();
    Some((ranges, lo, hi))
}

fn mirrored(min_v: u64, max_v: u64) -> Vec<u64> {
    let mut out = Vec::new(); let max_len = digits(max_v);
    for h in 1..=max_len/2 {
        let f = POW10[h]; let start = POW10[h-1];
        for half in start..f {
            let n = half*f + half;
            if n>=min_v && n<=max_v { out.push(n); }
        }
    }
    out.sort_unstable(); out
}

fn periodic(min_v: u64, max_v: u64) -> Vec<u64> {
    let max_len = digits(max_v); let mut out = Vec::with_capacity(120_000);
    for base_len in 1..=max_len/2 {
        let (base_lo, base_hi) = (POW10[base_len-1], POW10[base_len]-1);
        for rep in 2..=max_len/base_len {
            let f = repeat_factor(base_len, rep); if f==0 { continue; }
            let lo = base_lo.max(div_ceil(min_v, f)); let hi = base_hi.min(max_v / f);
            for b in lo..=hi { out.push(b * f); }
        }
    }
    out.sort_unstable(); out.dedup(); out
}

fn sum_over_ranges(values: &[u64], ranges: &[(u64,u64)]) -> u64 {
    if values.is_empty() { return 0; }
    let mut pref = Vec::with_capacity(values.len()+1); pref.push(0);
    for &v in values { pref.push(pref.last().unwrap()+v); }
    ranges.iter().map(|&(l,r)| {
        let a = values.partition_point(|&x| x < l);
        let b = values.partition_point(|&x| x <= r);
        pref[b]-pref[a]
    }).sum()
}

fn repeat_factor(base_len: usize, repeat: usize) -> u64 {
    let mut f=0u128; let base=POW10[base_len] as u128; let mut pow=1u128;
    for _ in 0..repeat { f+=pow; pow*=base; } f as u64
}
fn div_ceil(a:u64,b:u64)->u64{((a as u128+b as u128-1)/b as u128) as u64}
fn digits(x:u64)->usize{POW10.iter().position(|&p| x<p).unwrap_or(20)}
