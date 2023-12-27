use std::str::FromStr;
use regex::Match;
use regex_macro::regex;
use anyhow::{self, Context as AnyhowContext};
use crate::input::*;

#[derive(Copy, Clone, Default, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TokenPosition {
    #[default]
    OPENING,
    CLOSING,
    DISALLOWED
}


fn parse_technique_capture(capture: Option<Match>) -> anyhow::Result<Option<Technique>> {
    let capture_str = match capture {
        Some(c) => c.as_str()
            .strip_prefix('<').unwrap()
            .strip_suffix('>').unwrap(),
        None => return Ok(None)
    };

    match Technique::from_str(capture_str) {
        Ok(technique) => Ok(Some(technique)),
        Err(_e) => anyhow::bail!("Unknown technique: {capture_str}")
    }
}

fn parse_technique_captures(opening_capture: Option<Match>, closing_capture: Option<Match>, position: TokenPosition)
        -> anyhow::Result<Option<Technique>> {
    let opening_technique = parse_technique_capture(opening_capture)?;
    let closing_technique = parse_technique_capture(closing_capture)?;

    if opening_technique.is_some() && closing_technique.is_some() {
        anyhow::bail!("Technique defined twice for same entry: `{}` and `{}`",
            opening_technique.unwrap(), closing_technique.unwrap());
    } else if opening_technique.is_none() && closing_technique.is_none() {
        return Ok(None)
    }

    match position {
        TokenPosition::OPENING => match opening_technique {
            Some(technique) => Ok(Some(technique)),
            None => anyhow::bail!("Technique `{}` found on the right-side of entry. Expected in the opening position.",
                closing_technique.unwrap())
        },
        TokenPosition::CLOSING => match closing_technique {
            Some(technique) => Ok(Some(technique)),
            None => anyhow::bail!("Technique `{}` found on the left-side of entry. Expected in the closing position.",
                opening_technique.unwrap())
        },
        TokenPosition::DISALLOWED => {
            if opening_technique.is_some() {
                anyhow::bail!("Technique `{}` found on the left-side of entry. Techniques not allowed for this entry.",
                    opening_technique.unwrap())
            } else if closing_technique.is_some() {
                anyhow::bail!("Technique `{}` found on the right-side of entry. Techniques not allowed for this entry.",
                    closing_technique.unwrap())
            } else {
                Ok(None)
            }
        } 
    }
}

fn parse_note_capture(capture: Option<Match>) -> anyhow::Result<Option<String>> {
    match capture {
        Some(s) => Ok(Some(
            s.as_str()
                .strip_prefix('(').unwrap()
                .strip_suffix(')').unwrap()
                .trim()
                .to_string())),
        None => Ok(None)
    }
}

fn parse_note_captures<'h>(opening_capture: Option<Match>, closing_capture: Option<Match>)
        -> anyhow::Result<(Option<String>, Option<String>)> {
    let opening_note = parse_note_capture(opening_capture)?;
    let closing_note = parse_note_capture(closing_capture)?;
    Ok((opening_note, closing_note))
}

// This should be attempted first, as GroupEntries can enclose nested Sequences
fn try_parse_group_entry(rawstr: &str, symbol_lookup: &impl SymbolicLookup, context_lookup: &impl ContextLookup, recursion_depth: i32, technique_position: TokenPosition)
        -> anyhow::Result<Option<GroupEntry>> {
    // quick, though FALSE doesn't confirm that this *is* a GroupEntry.
    if !rawstr.contains('[') || !rawstr.contains(']') {
        return Ok(None);
    }

    // matches: (opening note) <opening technique> [ nested entries .. ] <closing technique> (closing note)
    // captures: 1. (optional) opening note 2. (optional) opening technique 3. nested entry string 4. (optional) closing technique 5. (optional) closing note
    // examples:
    //      [A,B,C]
    //      (quick)[A+B+C,D] (some note)
    //      [A,(inner note left)[B,C,D](tap),E+D](outter note.)
    let regex = regex!(r"^(\([^\(\)]+?\))?\s*(<.+?>)?\s*(\[.+\])\s*(<.+?>)?\s*(\([^\(\)]+?\))?$");
    let captures = match regex.captures(rawstr) {
        Some(c) => c,
        None => return Ok(None)
    };

    let (opening_note, closing_note) = parse_note_captures(captures.get(1), captures.get(5))?;
    let technique = parse_technique_captures(captures.get(2), captures.get(4), technique_position)?;
    let entries_str = captures.get(3).unwrap().as_str()
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .trim();
    let entries = parse_entries(entries_str, symbol_lookup, context_lookup, recursion_depth + 1, technique_position)?;

    Ok(Some(GroupEntry::new(
        entries,
        technique,
        opening_note,
        closing_note )))
}


// Expects a CSV token
// Ok(None) -> It isn't an InputEntry
// Ok(Some(_)) -> Success
// Err(_) -> It's an invalid InputEntry
fn try_parse_input_entry(rawstr: &str, nested: bool) -> anyhow::Result<Option<InputEntry>> {
    // matches: (opening note) <technique> symbol <technique> (closing note)
    // symbol must be alphanumeric
    // technique can only be specified once (left XOR right of symbol)
    // captures: 1. (optional) opening note 2. (optional) technique. 3. symbol 4. (optional) technique 5. (optional) closing note
    let regex = regex!(r"^(\([^\(\)]+?\))?\s*(<.+?>)?\s*([a-zA-Z0-9]+)\s*(<.+?>)?\s*(\([^\(\)]+?\))?$");
    let captures = match regex.captures(rawstr) {
        Some(c) => c,
        None => return Ok(None)
    };

    let symbol = captures.get(3).unwrap().as_str().to_string();
    let technique_position = if nested { TokenPosition::DISALLOWED} else { TokenPosition::default() };
    let technique = parse_technique_captures(captures.get(2), captures.get(4), technique_position )?;
    let (opening_note, closing_note) = parse_note_captures(captures.get(1), captures.get(5))?;

    Ok(Some(InputEntry::new(
        symbol,
        technique,
        opening_note,
        closing_note )))
}

fn try_parse_combination_entry(rawstr: &str, symbol_lookup: &impl SymbolicLookup, context_lookup: &impl ContextLookup, recursion_depth: i32)
        -> anyhow::Result<Option<CombinationEntry>> {
    // quick, though FALSE doesn't confirm that this *is* a GroupEntry.
    if !rawstr.contains('+') {
        return Ok(None);
    }

    // matches: (opening note) combination.. (closing note)
    // captures 1. opening note 2. opening technique 3. combination string 4. closing technique 5. closing note
    // examples:
    //      (note) A+B
    //      A + [B, C]
    let regex = regex!(r"^(\([^\(\)]+?\))?\s*(<.+?>)?\s*(.+?)\s*(<.+?>)?\s*(\([^\(\)]+?\))?$");
    let captures = match regex.captures(rawstr) {
        Some(c) => c,
        None => return Ok(None)
    };

    let (opening_note, closing_note) = parse_note_captures(captures.get(1), captures.get(5))?;
    let technique_position = if recursion_depth == 0 { TokenPosition::default() } else { TokenPosition::DISALLOWED };
    let technique = parse_technique_captures(captures.get(2), captures.get(4), technique_position)?;
    let combo_str = captures.get(3).unwrap().as_str();

    let mut items: Vec<CombinationItem> = Vec::new();
    let item_strings = split_char('+', false, combo_str)?;
    let mut i = 0;
    for item_str in item_strings {
        // closing if the first item, opening if the last
        let technique_position = if i == 0 { TokenPosition::CLOSING } else { TokenPosition::OPENING };
        i += 1;
        if let Some(entry) = try_parse_group_entry(item_str, symbol_lookup, context_lookup, recursion_depth, technique_position)? {
            items.push(CombinationItem::Group(entry));
            continue;
        } else if let Some(entry) = try_parse_input_entry(item_str, true)? {
            items.push(CombinationItem::Symbol(entry.symbol().to_owned()));
            continue;
         } else  {
            anyhow::bail!("Invalid or malformed combination entry: `{item_str}` :: {rawstr}");
         }
    }
    

    Ok(Some(CombinationEntry::new(
        items,
        technique,
        opening_note,
        closing_note )))
}

fn try_parse_note_entry(rawstr: &str) -> anyhow::Result<Option<String>> {
    if !rawstr.starts_with('(') || !rawstr.ends_with(')') {
        return Ok(None);
    }

    let note = rawstr
        .strip_prefix('(').unwrap()
        .strip_suffix(')').unwrap()
        .trim()
        .to_string();

    Ok(Some(note))
}

fn try_parse_context_entry(rawstr: &str, lookup: &impl ContextLookup) -> anyhow::Result<Option<Vec<Context>>> {
    if !rawstr.starts_with('{') || !rawstr.ends_with('}') {
        return Ok(None);
    }

    let context_strings  = rawstr
        .strip_prefix('{').unwrap()
        .strip_suffix('}').unwrap()
        .split(',')
        .map(|s| s.trim());

    let mut contexts: Vec<Context> = Vec::new();
    for context_string in context_strings {
        if let Ok(context) = Context::try_from(context_string) {
            contexts.push(context);
        } else {
            let context = lookup.lookup_custom_context(context_string)
                .context(format!("Unknown context: {context_string}"))?;
            contexts.push(context.to_owned());
        }
    }

    Ok(Some(contexts))
}

const MAX_RECURSION: i32 = 3;

pub(crate) fn parse_entries(
            rawstr: &str,
            symbol_lookup: &impl SymbolicLookup,
            context_lookup: &impl ContextLookup,
            recursion_depth: i32,
            technique_position: TokenPosition) -> anyhow::Result<Vec<Entry>> {
    if recursion_depth >= MAX_RECURSION {
        anyhow::bail!("Recursion limit reached while parsing input entry: {rawstr}");
    }

    let mut entries: Vec<Entry> = Vec::new();

    for entry_str in split_char(',', true, rawstr)? {
        if let Some(entry) = try_parse_note_entry(entry_str)? {
            entries.push(Entry::Note(entry));
            continue;
        } if let Some(contexts) = try_parse_context_entry(entry_str, context_lookup)? {
            entries.push(Entry::Context(contexts));
            continue;    
        } else if let Some(entry) = try_parse_group_entry(entry_str, symbol_lookup, context_lookup, recursion_depth, technique_position)? {
            entries.push(Entry::Group(entry));
            continue;    
        } else if let Some(entry) = try_parse_combination_entry(entry_str, symbol_lookup, context_lookup, recursion_depth)? {
            entries.push(Entry::Combination(entry));
            continue;
        } else if let Some(entry) = try_parse_input_entry(entry_str, recursion_depth > 0)? {
            entries.push(Entry::Input(entry));
            continue;
         } else  {
            anyhow::bail!("Invalid or malformed entry: `{entry_str}` :: {rawstr}");
         }
    }

    Ok(entries)
}

fn split_char(split_char: char, split_context: bool, rawstr: &str) -> anyhow::Result<Vec<&str>> {
    let mut entries: Vec<&str> = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let mut context_start = 0;
    let mut context_depth = 0;

    for (n, c) in rawstr.char_indices() {
        // this char is escaped
        match c {
            '{' => {
                if split_context {
                    if context_depth > 0 {
                        anyhow::bail!("Unable to split input characters. Nested context characters are invalid: {rawstr}");
                    }

                    if depth == 0 {
                        if n - start == 0 {
                            anyhow::bail!("Unable to split input entries. Empty entries are invalid: {rawstr}")
                        }

                        if start < n-1 {
                            entries.push(&rawstr[start .. n-1].trim());
                            start = n + 1;
                        }
                    }

                    context_depth = 1;
                    context_start = n;
                } else {
                    depth += 1;
                }
            },
            '}' => {
                if split_context {
                    if context_depth == 0 {
                        anyhow::bail!("Unable to split input characters. Illegal use of context characters: {rawstr}")
                    }

                    entries.push(&rawstr[context_start..n+1]);
                    context_depth = 0;
                    start = n + 1;
                } else {
                    depth -= 1;
                }
            },
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            _ => if c == split_char && context_depth == 0 {
                if depth == 0 {
                    if n - start == 0 {
                        anyhow::bail!("Unable to split input entries. Empty entries are invalid: {rawstr}")
                    }

                    entries.push(&rawstr[start .. n].trim());
                    start = n + 1;
                }
            }
        }
    }


    if depth != 0 {
        anyhow::bail!("Unable to split input entries. Illegal use of () or [] characters: {rawstr}");
    }

    let len = rawstr.len();
    if start < len {
        entries.push(&rawstr[start .. len].trim());
    }

    Ok(entries)
}

fn entry_trait_to_string(
            entry: &impl EntryTrait,
            _depth: i32,
            index_position: IndexPosition,
            contents: impl Fn() -> anyhow::Result<String>) -> anyhow::Result<String> {
    let technique_position = index_position.technique_position(); 
    let opening_note = match entry.opening_note() {
        Some(note) => format!("({note}) "),
        None => String::default() 
    };
    let closing_note = match entry.closing_note() {
        Some(note) => format!(" ({note})"),
        None => String::default()
    };
    let opening_technique = match technique_position {
        TokenPosition::OPENING => match entry.technique() {
            Some(technique) => format!("<{technique}> "),
            None => String::default()
        },
        _ => String::default()
    };
    let closing_technique = match technique_position {
        TokenPosition::CLOSING => match entry.technique() {
            Some(technique) => format!(" <{technique}>"),
            None => String::default()
        },
        _ => String::default()
    };

    let content = contents()?;
    let output = format!("{opening_note}{opening_technique}{content}{closing_technique}{closing_note}");
    Ok(output)
}

#[derive(PartialEq, Copy, Clone)]
enum IndexPosition {
    Only,
    First,
    Middle,
    Last
}

impl IndexPosition {
    pub fn determine(depth: i32, num_siblings: usize, index: usize) -> Self {
        if num_siblings == 0 || depth == 0 {
            IndexPosition::Only
        } else if index == 0 {
            IndexPosition::First
        } else if index == num_siblings-1 {
            IndexPosition::Last
        } else {
            IndexPosition::Middle
        }
    }

    pub fn technique_position(&self) -> TokenPosition {
        match self {
            Self::First => TokenPosition::CLOSING,
            _ => TokenPosition::OPENING
        }
    }
}

fn entry_to_string(entry: &Entry, depth: i32, index_position: IndexPosition) -> anyhow::Result<String> {
    let result = match entry {
        Entry::Note(note) => format!("({note})"),
        Entry::Input(input_entry) => entry_trait_to_string(input_entry, depth, index_position, || {
            Ok(input_entry.symbol.to_string())
        })?,
        Entry::Combination(combo_entry) => entry_trait_to_string(combo_entry, depth, index_position, || {
            let mut strings: Vec<String> = Vec::new();
            let mut i = 0;
            for combo_item in &combo_entry.items {
                match combo_item {
                    CombinationItem::Symbol(ref symbol) => strings.push(symbol.to_string()),
                    CombinationItem::Group(ref group_entry) => {
                        let index_position = IndexPosition::determine(depth + 1, combo_entry.items.len(), i);
                        let result = entry_trait_to_string(group_entry, depth + 1, index_position, || {
                            let entries_str = entries_to_string(&group_entry.entries, depth + 1)?;
                            Ok(format!("[ {entries_str} ]"))
                        })?;

                        strings.push(result);
                    }
                }

                i += 1;
            }

            Ok(strings.join(" + "))
        })?,
        Entry::Group(group_entry) => {
            entry_trait_to_string(group_entry, depth, index_position, || {
                let entries_str = entries_to_string(&group_entry.entries, depth + 1)?;
                Ok(format!("[ {entries_str} ]"))
            })?
        },
        Entry::Context(contexts) => {
            let contex_strings: Vec<String> = contexts.iter().map(|c| c.to_string()).collect();
            format!("{{{}}}", contex_strings.join(", "))
        }
    };

    Ok(result)
}

pub(crate) fn entries_to_string(entries: &Vec<Entry>, depth: i32) -> anyhow::Result<String> {
    let num_entries = entries.len();
    let mut entry_strings: Vec<String> = Vec::new();
    let mut i = 0;
    for entry in entries {
        let token_str = entry_to_string(entry, depth, IndexPosition::determine(depth, num_entries, i))?;
        entry_strings.push(token_str);
        i += 1;
    }


    let result = entry_strings
        .join(", ")
        .replace(", {", " {")
        .replace("}, ", "} ");
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{game, input::{self, Context}};

    use super::*;

    #[test]
    fn compare_displayed() {
        let game = game::test::game_fixture();
        let sequences_strings = vec![
            "(Air) B, F, P1",
            "D + P2 (hold P2)",
            "B + [ P1, P2, P3 ]"
        ];

        for sequence_string in sequences_strings {
            let sequence = input::Sequence::parse(sequence_string, &game, &game).unwrap();
            let actual = sequence.to_string();
            assert_eq!(sequence_string, actual);
        }
    }

    #[test]
    fn parse_sequence_and_displayed() {
        let game = game::test::game_fixture();
        let expected_sequences: &[(&'static str, Option<input::Sequence>)] = &[
            ("(Air) B, F, P1",
                Some(input::Sequence::new(vec![
                    input::Entry::Input(
                        input::InputEntry::new(
                            input::Symbol::from("B"),
                            None,
                            Some(String::from("Air")),
                            None
                    )),
                    input::InputEntry::for_symbol("F"),
                    input::InputEntry::for_symbol("P1"), ]))),
            ("(Air)B,F,P1",
                Some(input::Sequence::new(vec![
                    input::Entry::Input(
                        input::InputEntry::new(
                            input::Symbol::from("B"),
                            None,
                            Some(String::from("Air")),
                            None
                    )),
                    input::InputEntry::for_symbol("F"),
                    input::InputEntry::for_symbol("P1"), ]))),
            /*("(In air)",
                    None),*/
            ("D+P2 (hold P2)",
                Some(input::Sequence::new(vec![
                    input::Entry::Combination(
                        input::CombinationEntry::new(
                            vec![
                                input::CombinationItem::Symbol(input::Symbol::from("D")),
                                input::CombinationItem::Symbol(input::Symbol::from("P2")) ],
                            None,
                            None,
                            Some(String::from("hold P2"))
                        )
                    )
                ]))),
            ("F+K1+K2",
                Some(input::Sequence::new(vec![
                    input::Entry::Combination(
                        input::CombinationEntry::new(
                            vec![
                                input::CombinationItem::Symbol(input::Symbol::from("F")),
                                input::CombinationItem::Symbol(input::Symbol::from("K1")),
                                input::CombinationItem::Symbol(input::Symbol::from("K2")) ], 
                            None,
                            None,
                            None
                        )
                    )
                ]))
            ),
            ("B + K3",
                Some(input::Sequence::new(vec![
                    input::CombinationEntry::for_symbols(vec!["B","K3"])]))),
            ("U + 3K",
                Some(input::Sequence::new(vec![
                    input::CombinationEntry::for_symbols(vec!["U", "3K"])]))),
            ("F + P1 + P2",
                Some(input::Sequence::new(vec![
                    input::CombinationEntry::for_symbols(vec!["F", "P1", "P2"])]))),
            ("B+K2, ALT, (wait until Alt's last hit) D, F, P3",
                Some(input::Sequence::new(vec![
                    input::CombinationEntry::for_symbols(vec!["B","K2"]),
                    input::InputEntry::for_symbol("ALT"),
                    input::Entry::Input(input::InputEntry::new(
                        input::Symbol::from("D"),
                        None,
                        Some(String::from("wait until Alt's last hit")),
                        None)),
                    input::InputEntry::for_symbol("F"),
                    input::InputEntry::for_symbol("P3") ]))),
            ("QCF, P",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("QCF"),
                    input::InputEntry::for_symbol("P") ]))),
            ("QCF,2P",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("QCF"),
                    input::InputEntry::for_symbol("2P") ]))),
            ("QCB, 3P",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("QCB"),
                    input::InputEntry::for_symbol("3P") ]))),
            ("F,QCF, P2",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("F"),
                    input::InputEntry::for_symbol("QCF"),
                    input::InputEntry::for_symbol("P2") ]))),
            ("ZF, P1",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("ZF"),
                    input::InputEntry::for_symbol("P1") ]))),
            ("<charge> B, F, P",
                Some(input::Sequence::new(vec![
                    input::Entry::Input(input::InputEntry::new(
                        input::Symbol::from("B"),
                        Some(input::Technique::Charge),
                        None,
                        None )),
                    input::InputEntry::for_symbol("F"),
                    input::InputEntry::for_symbol("P") ]))),
            // technique is in the wrong position
            ("B <charge>, F, P",
                None),
            // unknown technique
            ("<error> B, F, P",
                None),
            ("B + [ P1, P2, P3 ]",
                Some(input::Sequence::new(vec![
                    input::Entry::Combination(input::CombinationEntry::new(
                        vec![
                            input::CombinationItem::Symbol(input::Symbol::from("B")),
                            input::CombinationItem::Group(input::GroupEntry::new(
                                vec![
                                    input::InputEntry::for_symbol("P1"),
                                    input::InputEntry::for_symbol("P2"),
                                    input::InputEntry::for_symbol("P3"),
                                ],
                                None,
                                None,
                                None
                            ))
                        ],
                        None,
                        None,
                        None 
                    ))
                ]))
            ),
            ("<quick> [ P2, K3, P1 ]",
                Some(input::Sequence::new(vec![
                    input::Entry::Group(input::GroupEntry::new(
                        vec![
                            input::InputEntry::for_symbol("P2"),
                            input::InputEntry::for_symbol("K3"),
                            input::InputEntry::for_symbol("P1") ],
                        Some(input::Technique::Quick),
                        None,
                        None
                    ))
                ]))
            ),
            ("QCF, P {juggle, exceptional} K3",
                Some(input::Sequence::new(vec![
                    input::InputEntry::for_symbol("QCF"),
                    input::InputEntry::for_symbol("P"),
                    input::Entry::Context(vec![
                        input::Context::Juggle,
                        input::Context::Custom(String::from("exceptional"))]),
                    input::InputEntry::for_symbol("K3")
                ]))
            ),

        ];

        for expected_sequence in expected_sequences {
                let result = input::Sequence::parse(expected_sequence.0, &game, &game);
                if let Some(fixed_sequence) = &expected_sequence.1 {
                    let actual_sequence = result.unwrap();
                    assert_eq!(fixed_sequence, &actual_sequence,
                        "Sequence::parse(`{}`) should match fixture :: {:#?}", expected_sequence.0, actual_sequence);
                } else if let Ok(sequence) = result {
                    assert!(false,
                        "Sequence::parse(`{}`) should have failed. Instead: {:#?} ", expected_sequence.0, sequence);
                }

                // convert to string and back
                if let Some(sequence) = expected_sequence.1.as_ref() {
                    let displayed_sequence = sequence.to_string();
                    let parsed_sequence = input::Sequence::parse(&displayed_sequence, &game, &game);
                    assert!(parsed_sequence.is_ok(),
                        "Unable to parse Display<Sequence>: {} -> {} :: {}", expected_sequence.0, displayed_sequence, parsed_sequence.err().unwrap());
                    let parsed_sequence = parsed_sequence.ok().unwrap();
                    assert_eq!(sequence, &parsed_sequence,
                        "Original Sequence and its parsed Display<> should match: `{}` vs `{}`", expected_sequence.0, parsed_sequence);
                }
        }
    }
 
    #[test]
    fn test_named_sequence_new() {
        let game = game::test::game_fixture();
        let named_sequence = input::NamedSequence::new(
            String::from("Sonic Something"),
            input::Sequence::new(vec![
                input::Entry::Input(input::InputEntry::new(
                    input::Symbol::from("B"),
                    Some(input::Technique::Charge),
                    None,
                    None )),
                input::InputEntry::for_symbol("F"),
                input::InputEntry::for_symbol("P") ]),
            game.find_input_category("Special Move").unwrap(),
            Some(String::from("This is hard to do!"))
        );

        assert_eq!(game.input_category(1), named_sequence.category(&game),
            "Categories should match");
    }

    #[test]
    fn test_split_char() {
        let tests = vec![
            // 0. input, 1. split_char, 2. split_context, 3. expected output
            ( "A,B,C", ',', true, vec![ "A", "B", "C" ] ),
            ( " A , B , C ", ',', true, vec![ "A", "B", "C" ] ),
            ( "A, B, {air} C", ',', true, vec![ "A", "B", "{air}", "C" ] ),
            ( "A, B, {air}", ',', true, vec![ "A", "B", "{air}" ] ),
            ( "A, B, {air}C", ',', true, vec![ "A", "B", "{air}", "C" ] ),
            ( " {air}A, B", ',', true, vec![ "{air}", "A", "B" ] ),
            ( "A, B, {air} C", ',', false, vec![ "A", "B", "{air} C" ] ),
            ( "A + B + {air} C", '+', false, vec![ "A", "B", "{air} C" ] ),
            ( "A {juggle, air} B", ',', true, vec![ "A", "{juggle, air}", "B" ] ),
        ];

        for test in tests {
            let actual = split_char(test.1, test.2, test.0).unwrap();
            assert_eq!(test.3, actual,
                "Actual output should match: {}", test.0);
        }
    }

    #[test]
    fn test_try_parse_context_entry() {
        let game = game::test::game_fixture();
        let tests = vec![
            ("bleh", None),
            ("{air}", Some(vec![Context::Air])),
            ("{air,close}", Some(vec![Context::Air, Context::Close])),
            ("{exceptional}", Some(vec![Context::Custom(String::from("exceptional"))])),
        ];
        let err_tests = vec![
            "{foo}"
        ];

        for test in tests {
            let actual = try_parse_context_entry(test.0, &game).unwrap();
            assert_eq!(test.1, actual,
                "Actual output should match: {}", test.0);
        }

        for err_test in err_tests {
            let actual = try_parse_context_entry(err_test, &game);
            assert!(actual.is_err(),
                "Actual result should have been an error: {} :: {:#?}", err_test, actual.ok().unwrap());
        }
    }
}