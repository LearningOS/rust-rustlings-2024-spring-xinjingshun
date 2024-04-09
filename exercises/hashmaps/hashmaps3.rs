// hashmaps3.rs
// 给出了一场足球比赛的分数列表（每行一个分数）。每一行的格式为：“<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>”。例如：England,France,4,2（英格兰进了4个球，法国进了2个球）。
//
// 你需要构建一个包含球队名称、球队进球数和球队失球数的得分表。构建得分表的一种方法是使用HashMap。已经部分写出了使用HashMap的解决方案，请完成它以通过测试。
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

// I AM  DONE

use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() { //.lines()方法用于将字符串按行拆分成一个迭代器，每次迭代返回一个字符串表示的行
        let v: Vec<&str> = r.split(',').collect(); //将字符串 r 按逗号 , 分割成多个子字符串，并将这些子字符串放入一个 Vec<&str> 向量中
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: 用从当前行提取的细节填充得分表。请记住，由team_1进球将是team_2失球的数量，同样地，由team_2进球将是team_1失球的数量。
        if let Some(team) = scores.get_mut(&team_1_name) {
            // Update the team's goals scored and goals conceded
            team.goals_scored += team_1_score;
            team.goals_conceded += team_2_score;
        } else {
            // Insert a new record for the team
            scores.insert(team_1_name.clone(), Team { goals_scored: team_1_score, goals_conceded: team_2_score });
        }

        // Repeat the same process for team 2
        if let Some(team) = scores.get_mut(&team_2_name) {
            team.goals_scored += team_2_score;
            team.goals_conceded += team_1_score;
        } else {
            scores.insert(team_2_name.clone(), Team { goals_scored: team_2_score, goals_conceded: team_1_score });
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
