//! Numeric special functions: gamma, error function, incomplete gamma and beta, the normal
//! distribution and a generic monotone root finder. Written from the standard textbook
//! formulations (Lanczos approximation, series / continued-fraction expansions).

use std::f64::consts::PI;

const LANCZOS_G: f64 = 7.0;
const LANCZOS: [f64; 9] = [
    0.999_999_999_999_809_9,
    676.520_368_121_885_1,
    -1_259.139_216_722_402_8,
    771.323_428_777_653_1,
    -176.615_029_162_140_6,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_311_6e-7,
];

/// ln |Γ(x)|. NaN at non-positive integers.
pub(crate) fn ln_gamma(x: f64) -> f64 {
    if !x.is_finite() {
        return f64::NAN;
    }
    if x <= 0.0 && x == x.floor() {
        return f64::NAN;
    }
    if x < 0.5 {
        // Reflection.
        let s = (PI * x).sin().abs();
        return (PI / s).ln() - ln_gamma(1.0 - x);
    }
    if x < 15.0 && x == x.floor() {
        // Exact for small integers.
        let mut f = 1.0f64;
        let mut i = 2.0;
        while i < x {
            f *= i;
            i += 1.0;
        }
        return f.ln();
    }
    let x = x - 1.0;
    let mut a = LANCZOS[0];
    let t = x + LANCZOS_G + 0.5;
    for (i, c) in LANCZOS.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

/// Γ(x). NaN at non-positive integers, infinite on overflow.
pub(crate) fn gamma(x: f64) -> f64 {
    if !x.is_finite() || (x <= 0.0 && x == x.floor()) {
        return f64::NAN;
    }
    if x == x.floor() && x <= 171.0 {
        let mut f = 1.0f64;
        let mut i = 2.0;
        while i < x {
            f *= i;
            i += 1.0;
        }
        return f;
    }
    if x < 0.5 {
        return PI / ((PI * x).sin() * gamma(1.0 - x));
    }
    if x > 171.7 {
        return f64::INFINITY;
    }
    ln_gamma(x).exp()
}

/// The Stirling-series remainder δ(z) = ln Γ(z) − ((z − ½) ln z − z + ½ ln 2π), for z ≥ 10, where
/// eight terms leave an error below 1e-17.
fn stirling_remainder(z: f64) -> f64 {
    // B₂ₙ / (2n (2n − 1)) for n = 1..8.
    const C: [f64; 8] = [1.0 / 12.0, -1.0 / 360.0, 1.0 / 1260.0, -1.0 / 1680.0, 1.0 / 1188.0, -691.0 / 360_360.0, 1.0 / 156.0, -3617.0 / 122_400.0];
    let r = 1.0 / z;
    let r2 = r * r;
    C.iter().rev().fold(0.0, |s, c| s * r2 + c) * r
}

/// ln Γ(b) − ln Γ(a + b) for b ≥ 10. Taken as the difference of two log-gammas it would lose
/// about b·ln b·ε to cancellation (five digits at b = 5e9); through Stirling's series it is
/// −a ln(a + b) − (b − ½) ln(1 + a/b) + a + δ(b) − δ(a + b), with no large terms cancelling.
fn ln_gamma_ratio(a: f64, b: f64) -> f64 {
    let ab = a + b;
    -a * ab.ln() - (b - 0.5) * (a / b).ln_1p() + a + stirling_remainder(b) - stirling_remainder(ab)
}

/// ln B(a, b). With one parameter large, the two large log-gammas are taken as one ratio, so the
/// t, F and beta distributions keep their digits at large degrees of freedom.
pub(crate) fn ln_beta(a: f64, b: f64) -> f64 {
    let (small, large) = if a <= b { (a, b) } else { (b, a) };
    if large >= 10.0 && small > 0.0 && large.is_finite() {
        return ln_gamma(small) + ln_gamma_ratio(small, large);
    }
    ln_gamma(a) + ln_gamma(b) - ln_gamma(a + b)
}

/// ln(1 + d) − d, accurate where the two terms nearly cancel (small d).
fn ln_1p_minus(d: f64) -> f64 {
    if d.abs() >= 0.25 {
        return d.ln_1p() - d;
    }
    // −d²/2 + d³/3 − d⁴/4 + …
    let (mut power, mut sum) = (d, 0.0);
    for k in 2..80 {
        power *= -d;
        let term = power / k as f64;
        sum += term;
        if term.abs() <= 1e-17 * sum.abs() {
            break;
        }
    }
    sum
}

/// Regularised lower incomplete gamma P(a, x).
pub(crate) fn reg_gamma_p(a: f64, x: f64) -> f64 {
    if a.is_nan() || a <= 0.0 || x.is_nan() || x < 0.0 || !a.is_finite() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x.is_infinite() {
        return 1.0;
    }
    if x < a + 1.0 { gamma_series(a, x, front_of(a, x)) } else { 1.0 - gamma_cf(a, x, front_of(a, x)) }
}

/// Regularised upper incomplete gamma Q(a, x) = 1 − P(a, x).
pub(crate) fn reg_gamma_q(a: f64, x: f64) -> f64 {
    let front = if a > 0.0 && x > 0.0 && x.is_finite() { front_of(a, x) } else { 0.0 };
    reg_gamma_q_front(a, x, front)
}

/// Q(a, x) given the prefactor `e^-x x^a / Γ(a)` (the Poisson mass at a - 1, times x).
pub(crate) fn reg_gamma_q_front(a: f64, x: f64, front: f64) -> f64 {
    if a.is_nan() || a <= 0.0 || x.is_nan() || x < 0.0 || !a.is_finite() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    if x.is_infinite() {
        return 0.0;
    }
    if x < a + 1.0 { 1.0 - gamma_series(a, x, front) } else { gamma_cf(a, x, front) }
}

/// `e^-x x^a / Γ(a)`. For a ≥ 10 the large terms of −x + a ln x − ln Γ(a) are cancelled exactly
/// through Stirling's series: a (ln(1 + d) − d) + ½ ln(a / 2π) − δ(a), with d = (x − a) / a, so
/// CHISQ, GAMMA and POISSON keep their digits at large degrees of freedom or means.
pub(crate) fn front_of(a: f64, x: f64) -> f64 {
    if a >= 10.0 && a.is_finite() {
        let d = (x - a) / a;
        return (a * ln_1p_minus(d) + 0.5 * (a / (2.0 * PI)).ln() - stirling_remainder(a)).exp();
    }
    (-x + a * x.ln() - ln_gamma(a)).exp()
}

fn gamma_series(a: f64, x: f64, front: f64) -> f64 {
    let mut ap = a;
    let mut sum = 1.0 / a;
    let mut del = sum;
    for _ in 0..10_000 {
        ap += 1.0;
        del *= x / ap;
        sum += del;
        if del.abs() < sum.abs() * 1e-17 {
            break;
        }
    }
    sum * front
}

fn gamma_cf(a: f64, x: f64, front: f64) -> f64 {
    let tiny = 1e-300;
    let mut b = x + 1.0 - a;
    let mut c = 1.0 / tiny;
    let mut d = 1.0 / b;
    let mut h = d;
    for i in 1..10_000 {
        let an = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = an * d + b;
        if d.abs() < tiny {
            d = tiny;
        }
        c = b + an / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-17 {
            break;
        }
    }
    front * h
}

/// Error function.
pub(crate) fn erf(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x.abs() < 0.5 {
        // Taylor series: erf x = 2/√π Σ (-1)^n x^(2n+1) / (n! (2n+1)).
        let mut sum = x;
        let mut term = x;
        let x2 = x * x;
        for n in 1..60 {
            term *= -x2 / n as f64;
            let add = term / (2 * n + 1) as f64;
            sum += add;
            if add.abs() < 1e-18 * sum.abs() {
                break;
            }
        }
        return sum * 2.0 / PI.sqrt();
    }
    let p = reg_gamma_p(0.5, x * x);
    if x < 0.0 { -p } else { p }
}

/// Complementary error function, accurate in the tails.
pub(crate) fn erfc(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x.abs() < 0.5 {
        return 1.0 - erf(x);
    }
    if x > 0.0 { reg_gamma_q(0.5, x * x) } else { 1.0 + reg_gamma_p(0.5, x * x) }
}

/// Regularised incomplete beta I_x(a, b).
pub(crate) fn reg_inc_beta(x: f64, a: f64, b: f64) -> f64 {
    if x > 0.0 && x < 1.0 && a > 0.0 && b > 0.0 {
        let ln_bt = -ln_beta(a, b) + a * x.ln() + b * (-x).ln_1p();
        return reg_inc_beta_front(x, a, b, ln_bt.exp());
    }
    reg_inc_beta_front(x, a, b, 0.0)
}

/// 1 − I_x(a, b), computed on its own rather than subtracted from 1, so it keeps its digits both
/// where it is tiny and where x is so small that 1 − x (the argument of I_(1−x)(b, a)) would round.
pub(crate) fn reg_inc_beta_complement(x: f64, a: f64, b: f64) -> f64 {
    if a.is_nan() || a <= 0.0 || b.is_nan() || b <= 0.0 || !(0.0..=1.0).contains(&x) {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    if x == 1.0 {
        return 0.0;
    }
    let bt = (-ln_beta(a, b) + a * x.ln() + b * (-x).ln_1p()).exp();
    if x < (a + 1.0) / (a + b + 2.0) { 1.0 - bt * beta_cf(a, b, x) / a } else { bt * beta_cf(b, a, 1.0 - x) / b }
}

/// I_x(a, b) given the prefactor `bt = x^a (1-x)^b / B(a, b)` (symmetric in the two sides). A
/// caller that knows `bt` more accurately than the log-gamma route gives (the binomial mass, for
/// instance) passes it here.
pub(crate) fn reg_inc_beta_front(x: f64, a: f64, b: f64, bt: f64) -> f64 {
    if a.is_nan() || a <= 0.0 || b.is_nan() || b <= 0.0 || !(0.0..=1.0).contains(&x) {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    if x < (a + 1.0) / (a + b + 2.0) { bt * beta_cf(a, b, x) / a } else { 1.0 - bt * beta_cf(b, a, 1.0 - x) / b }
}

fn beta_cf(a: f64, b: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..10_000 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-16 {
            break;
        }
    }
    h
}

/// Standard normal density.
pub(crate) fn norm_pdf(z: f64) -> f64 {
    (-0.5 * z * z).exp() / (2.0 * PI).sqrt()
}

/// Standard normal cumulative distribution.
pub(crate) fn norm_cdf(z: f64) -> f64 {
    0.5 * erfc(-z / std::f64::consts::SQRT_2)
}

/// Inverse of the standard normal CDF (NaN outside (0, 1)).
pub(crate) fn norm_inv(p: f64) -> f64 {
    if !(p > 0.0 && p < 1.0) {
        return f64::NAN;
    }
    // Acklam's rational approximation, then one Halley refinement step.
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [f64; 5] =
        [-5.447_609_879_822_406e1, 1.615_858_368_580_409e2, -1.556_989_798_598_866e2, 6.680_131_188_771_972e1, -1.328_068_155_288_572e1];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [f64; 4] = [7.784_695_709_041_462e-3, 3.224_671_290_700_398e-1, 2.445_134_137_142_996, 3.754_408_661_907_416];
    let plow = 0.02425;
    let x = if p < plow {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - plow {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    let mut x = x;
    for _ in 0..2 {
        let e = norm_cdf(x) - p;
        let u = e * (2.0 * PI).sqrt() * (x * x / 2.0).exp();
        let nx = x - u / (1.0 + x * u / 2.0);
        if !nx.is_finite() {
            break;
        }
        x = nx;
    }
    x
}

/// Finds `x` in `[lo, hi]` with `f(x) = target` for a monotone `f` by bisection (expanding `hi`
/// up to `hi_max` while the target is not bracketed). Returns NaN when no root is bracketed.
pub(crate) fn invert(f: impl Fn(f64) -> f64, target: f64, lo: f64, hi: f64, hi_max: f64) -> f64 {
    let (mut lo, mut hi) = (lo, hi);
    let flo = f(lo) - target;
    let mut fhi = f(hi) - target;
    let mut guard = 0;
    while flo * fhi > 0.0 && hi < hi_max && guard < 200 {
        lo = hi;
        hi = (hi * 2.0).min(hi_max);
        fhi = f(hi) - target;
        guard += 1;
    }
    let flo = f(lo) - target;
    if flo == 0.0 {
        return lo;
    }
    if fhi == 0.0 {
        return hi;
    }
    if flo * fhi > 0.0 || flo.is_nan() || fhi.is_nan() {
        return f64::NAN;
    }
    let lo_neg = flo < 0.0;
    for _ in 0..300 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        let fm = f(mid) - target;
        if fm == 0.0 {
            return mid;
        }
        if (fm < 0.0) == lo_neg {
            lo = mid;
        } else {
            hi = mid;
        }
        if (hi - lo).abs() <= 1e-15 * mid.abs().max(1e-300) {
            break;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol * b.abs().max(1.0), "{a} vs {b}");
    }

    #[test]
    fn gamma_fn() {
        close(gamma(5.0), 24.0, 1e-15);
        close(gamma(0.5), PI.sqrt(), 1e-13);
        close(gamma(-1.5), 2.363_271_801_207_355, 1e-12);
        close(ln_gamma(10.0), 12.801_827_480_081_469, 1e-13);
        close(ln_gamma(101.0), 363.739_375_555_563_47, 1e-13);
        assert!(gamma(0.0).is_nan());
    }

    #[test]
    fn error_fn() {
        close(erf(1.0), 0.842_700_792_949_714_9, 1e-14);
        close(erf(0.3), 0.328_626_759_459_127_4, 1e-14);
        close(erf(-2.0), -0.995_322_265_018_952_7, 1e-14);
        close(erfc(3.0), 2.209_049_699_858_544e-5, 1e-12);
        close(erfc(-1.0), 1.842_700_792_949_715, 1e-14);
    }

    #[test]
    fn incomplete() {
        close(reg_gamma_p(2.0, 3.0), 0.800_851_726_528_544, 1e-13);
        close(reg_inc_beta(0.4, 2.0, 3.0), 0.5248, 1e-13);
        close(norm_cdf(1.96), 0.975_002_104_851_780, 1e-13);
        close(norm_inv(0.975), 1.959_963_984_540_054, 1e-12);
        close(norm_inv(1e-10), -6.361_340_902_404_056, 1e-10);
        close(invert(|x| x * x, 2.0, 0.0, 1.0, 10.0), 2f64.sqrt(), 1e-14);
    }
}
