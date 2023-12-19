use common::maplit::hashmap;
use std::collections::HashMap;

static INPUT_PARTS: &str = include_str!("parts");
static INPUT_WORKFLOWS: &str = include_str!("workflows");

fn main() {
    let parts = parse_parts(INPUT_PARTS);
    let workflows = parse_workflows(INPUT_WORKFLOWS);

    let res1 = part_1(&parts, &workflows);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&workflows);
    println!("Part 2 result: {}", res2);
}

fn part_1(parts: &[Part], workflows: &Workflows) -> usize {
    fn is_accepted(part: &Part, workflows: &Workflows, workflow_label: &str) -> bool {
        let rule_result = workflows[workflow_label]
            .iter()
            .find(|r| match r.condition {
                RuleCondition::GreaterThan { category, value } => {
                    part.get_rating(&category) > value
                }
                RuleCondition::LessThan { category, value } => part.get_rating(&category) < value,
                RuleCondition::Default => true,
            })
            .map(|r| &r.result)
            .unwrap();
        match rule_result {
            RuleResult::Accepted => true,
            RuleResult::Rejected => false,
            RuleResult::ChainTo(label) => is_accepted(part, workflows, label),
        }
    }

    parts
        .iter()
        .map(|p| {
            if is_accepted(p, workflows, "in") {
                p.ratings.values().sum()
            } else {
                0
            }
        })
        .sum()
}

fn part_2(workflows: &Workflows) -> usize {
    fn recurs(workflows: &Workflows, workflow_label: &str, range: PartsRange) -> usize {
        let rules = &workflows[workflow_label];
        rules
            .iter()
            .fold((0, range), |(acc, remaining_range), rule| {
                match rule.condition {
                    RuleCondition::GreaterThan { category, value } => remaining_range
                        .0
                        .get(&category)
                        .map(|rr| {
                            let mut matched = remaining_range.clone();
                            if rr.1 > value {
                                matched
                                    .0
                                    .insert(category, RatingRange(rr.0.max(value + 1), rr.1));
                            } else {
                                matched.0.remove(&category);
                            };

                            let mut not_matched = remaining_range.clone();
                            if rr.0 > value {
                                not_matched.0.remove(&category);
                            } else {
                                not_matched
                                    .0
                                    .insert(category, RatingRange(rr.0, rr.1.min(value)));
                            }

                            match &rule.result {
                                RuleResult::Accepted => (acc + matched.size(), not_matched),
                                RuleResult::Rejected => (acc, not_matched),
                                RuleResult::ChainTo(label) => {
                                    (acc + recurs(workflows, &label, matched), not_matched)
                                }
                            }
                        })
                        .unwrap_or((0, remaining_range)),
                    RuleCondition::LessThan { category, value } => remaining_range
                        .0
                        .get(&category)
                        .map(|rr| {
                            let mut matched = remaining_range.clone();
                            if rr.0 < value {
                                matched
                                    .0
                                    .insert(category, RatingRange(rr.0, rr.1.min(value - 1)));
                            } else {
                                matched.0.remove(&category);
                            };

                            let mut not_matched = remaining_range.clone();
                            if rr.1 < value {
                                not_matched.0.remove(&category);
                            } else {
                                not_matched
                                    .0
                                    .insert(category, RatingRange(rr.0.max(value), rr.1));
                            }

                            match &rule.result {
                                RuleResult::Accepted => (acc + matched.size(), not_matched),
                                RuleResult::Rejected => (acc, not_matched),
                                RuleResult::ChainTo(label) => {
                                    (acc + recurs(workflows, &label, matched), not_matched)
                                }
                            }
                        })
                        .unwrap_or((0, remaining_range)),
                    RuleCondition::Default => match &rule.result {
                        RuleResult::Accepted => (acc + remaining_range.size(), PartsRange::empty()),
                        RuleResult::Rejected => (acc, PartsRange::empty()),
                        RuleResult::ChainTo(label) => (
                            acc + recurs(workflows, &label, remaining_range),
                            PartsRange::empty(),
                        ),
                    },
                }
            })
            .0
    }

    let initial_range = PartsRange(hashmap! {
        Category::X => RatingRange(1, 4000),
        Category::M => RatingRange(1, 4000),
        Category::A => RatingRange(1, 4000),
        Category::S => RatingRange(1, 4000),
    });

    recurs(workflows, "in", initial_range)
}

fn parse_parts(input: &str) -> Vec<Part> {
    workflow_parser::parts(input).unwrap()
}

fn parse_workflows(input: &str) -> Workflows {
    workflow_parser::workflows(input).unwrap()
}

peg::parser! {
    grammar workflow_parser() for str {
        rule label() -> String = l:$(['a'..='z']+) { l.to_owned() }
        rule ruleResult() -> RuleResult = "A" { RuleResult::Accepted } / "R" { RuleResult::Rejected } / l:label() { RuleResult::ChainTo(l) }
        rule category() -> Category = "x" { Category::X } / "m" { Category::M } / "a" { Category::A } / "s" { Category::S }
        rule value() -> usize = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }
        rule greaterThan() -> Rule = category:category() ">" value:value() ":" result:ruleResult() { Rule { condition: RuleCondition::GreaterThan { category, value }, result } }
        rule lessThan() -> Rule = category:category() "<" value:value() ":" result:ruleResult() { Rule { condition: RuleCondition::LessThan { category, value }, result } }
        rule default() -> Rule = result:ruleResult() { Rule { condition: RuleCondition::Default, result } }
        rule workflowRule() -> Rule = greaterThan() / lessThan() / default()
        rule rules() -> Vec<Rule> = (workflowRule() ** ",")
        rule workflow() -> (String, Vec<Rule>) = label:label() "{" rules:rules() "}" { ( label, rules ) }
        pub rule workflows() -> Workflows = w:(workflow() ** "\n") { w.into_iter().collect() }

        rule rating() -> (Category, usize) = c:category() "=" v:value() { (c, v) }
        rule part() -> Part = "{" r:(rating() ** ",") "}" { Part { ratings: r.into_iter().collect() } }
        pub rule parts() -> Vec<Part> = (part() ** "\n")
    }
}

impl RatingRange {
    fn len(&self) -> usize {
        self.1 - self.0 + 1
    }
}
#[derive(Copy, Clone, Debug)]
struct RatingRange(usize, usize); // Inclusive

impl PartsRange {
    fn empty() -> PartsRange {
        PartsRange(HashMap::new())
    }
    fn size(&self) -> usize {
        self.0.values().map(|r| r.len()).product()
    }
}
#[derive(Clone, Debug)]
struct PartsRange(HashMap<Category, RatingRange>);

type Workflows = HashMap<String, Vec<Rule>>;

#[derive(Eq, PartialEq, Debug)]
struct Rule {
    condition: RuleCondition,
    result: RuleResult,
}

#[derive(Eq, PartialEq, Debug)]
enum RuleCondition {
    GreaterThan { category: Category, value: usize },
    LessThan { category: Category, value: usize },
    Default,
}

#[derive(Eq, PartialEq, Debug)]
enum RuleResult {
    Accepted,
    Rejected,
    ChainTo(String),
}

impl Part {
    fn get_rating(&self, category: &Category) -> usize {
        match self.ratings.get(category) {
            Some(&v) => v,
            None => 0,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Part {
    ratings: HashMap<Category, usize>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_PARTS: &str = "{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    static TEST_WORKFLOWS: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";

    #[test]
    fn test() {
        println!("parts:");
        for p in parse_parts(TEST_PARTS) {
            println!("{:?}", p);
        }
        println!();
        println!("workflows:");
        for w in parse_workflows(TEST_WORKFLOWS) {
            println!("{:?}", w);
        }
        println!()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&parse_parts(TEST_PARTS), &parse_workflows(&TEST_WORKFLOWS)),
            19114
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_workflows(&TEST_WORKFLOWS)), 167409079868000);
    }
}
