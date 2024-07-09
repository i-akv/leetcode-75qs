pub struct Solution{}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let vowels = "aeiouAEIOU";

        let mut lp = 0;
        let mut rp = s.len() -1;

        while lp<rp {
            let (l, r) = (chars[lp], chars[rp]);

            if let (true, true) = (vowels.contains(l), vowels.contains(r)) {
                chars[lp] = r;
                chars[rp] = l;

                lp+=1; rp-=1;
                continue;
            }
            if !vowels.contains(l) {lp+=1}
            if !vowels.contains(r) {rp-=1}
        }

        chars.into_iter().collect()
    }
}