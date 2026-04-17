use rust_decimal::prelude::ToPrimitive;
use uuid::Uuid;

use super::candidate::MatcherCandidate;
use super::config::MatcherConfig;

/// Composite score per spec §5 (normalized tokens, rating, inverse distance), then **stable** sort:
/// higher score first; ties broken by `plumber_id` ascending (lexicographic UUID order).
pub fn rank_and_take_top(candidates: Vec<MatcherCandidate>, config: &MatcherConfig) -> Vec<Uuid> {
    if candidates.is_empty() {
        return vec![];
    }

    let max_token = candidates
        .iter()
        .map(|c| c.token_balance)
        .max()
        .unwrap_or(0)
        .max(1) as f64;

    let mut scored: Vec<(Uuid, f64)> = candidates
        .into_iter()
        .map(|c| {
            let norm_t = c.token_balance as f64 / max_token;
            let rating_f = c.rating_avg.to_f64().unwrap_or(0.0);
            let dist_term = 1.0 / (1.0 + c.distance_km);
            let score = config.w_token * norm_t
                + config.w_rating * rating_f
                + config.w_distance * dist_term;
            (c.plumber_id, score)
        })
        .collect();

    scored.sort_by(|a, b| {
        b.1.total_cmp(&a.1) // higher score first
            .then_with(|| a.0.cmp(&b.0))
    });

    scored
        .into_iter()
        .take(config.batch_size)
        .map(|(id, _)| id)
        .collect()
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use uuid::Uuid;

    use super::super::config::MatcherConfig;
    use super::super::candidate::MatcherCandidate;
    use super::rank_and_take_top;

    fn cand(id: Uuid, token: i32, rating: f64, dist: f64) -> MatcherCandidate {
        MatcherCandidate {
            plumber_id: id,
            token_balance: token,
            rating_avg: Decimal::from_f64_retain(rating).unwrap_or(Decimal::ZERO),
            distance_km: dist,
        }
    }

    #[test]
    fn stable_top_three_identical_runs() {
        let config = MatcherConfig::default();
        let ids = [
            Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap(),
            Uuid::parse_str("00000000-0000-4000-8000-000000000002").unwrap(),
            Uuid::parse_str("00000000-0000-4000-8000-000000000003").unwrap(),
            Uuid::parse_str("00000000-0000-4000-8000-000000000004").unwrap(),
        ];
        let v = vec![
            cand(ids[0], 10, 4.0, 1.0),
            cand(ids[1], 10, 4.0, 1.0),
            cand(ids[2], 10, 4.0, 1.0),
            cand(ids[3], 10, 4.0, 1.0),
        ];
        let a = rank_and_take_top(v.clone(), &config);
        let b = rank_and_take_top(v, &config);
        assert_eq!(a, b);
        assert_eq!(a.len(), 3);
        assert_eq!(a, vec![ids[0], ids[1], ids[2]]);
    }

    #[test]
    fn tie_breaks_by_plumber_id_ascending() {
        let config = MatcherConfig {
            batch_size: 2,
            ..MatcherConfig::default()
        };
        let hi = Uuid::parse_str("ffffffff-ffff-4fff-bfff-ffffffffffff").unwrap();
        let lo = Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap();
        let v = vec![cand(hi, 5, 3.0, 2.0), cand(lo, 5, 3.0, 2.0)];
        let got = rank_and_take_top(v, &config);
        assert_eq!(got, vec![lo, hi]);
    }

    #[test]
    fn higher_score_wins_over_tie_break() {
        let config = MatcherConfig::default();
        let a = Uuid::parse_str("00000000-0000-4000-8000-000000000001").unwrap();
        let b = Uuid::parse_str("00000000-0000-4000-8000-000000000002").unwrap();
        let v = vec![
            cand(a, 100, 5.0, 0.0), // dominates on tokens + rating + distance
            cand(b, 0, 0.0, 99.0),
        ];
        let got = rank_and_take_top(v, &config);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0], a);
    }
}
