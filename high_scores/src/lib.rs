#[derive(Debug)]
pub struct HighScores{
    scores_: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores{
            // 将切片转换为vector
            scores_: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores_
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores_.is_empty(){
            return None;
        }
        return self.scores_.last().copied();
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores_.is_empty(){
            return None;
        }
        let mut best_score = 0;
        for score in &self.scores_{
            if *score > best_score{
                best_score = *score;
            }
        }
        Some(best_score)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut t = self.scores_.clone();
        t.sort_by(|a, b| b.cmp(a));
        let mut res:Vec<u32> = vec![];
        for i in 0..t.len(){
            if i >= 3{
                break;
            }
            res.push(t[i]);
        }
        return res;
    }
}