use crate::types::{LexerOutput, LexerToken, LexerBlock};
use std::collections::HashSet;

fn lowercase_all_tokens(tokens: LexerOutput) -> LexerOutput {
    LexerOutput(tokens.0.into_iter().map(|block| {
        LexerBlock(block.0.into_iter().map(|token| {
            match token {
                LexerToken::Text(s) => LexerToken::Text(s.to_lowercase()),
                _ => token
            }
        }).collect::<Vec<_>>())
    }).collect::<Vec<_>>())
}

fn remove_articles_and_possessives(tokens: LexerOutput) -> LexerOutput {
    let articles_and_possessives = HashSet::from(["a", "an", "the", "my", "your", "his", "her", "its", "our", "their"]);

    LexerOutput(tokens.0.into_iter().map(|block| {
        LexerBlock(block.0.into_iter().filter(|token| {
            match token {
                LexerToken::Text(s) => !articles_and_possessives.contains(&s.as_str()),
                _ => true
            }
        }).collect::<Vec<_>>())
    }).collect::<Vec<_>>())
}

fn remove_end_sentence_punctuation_in_quotes(tokens: LexerOutput) -> LexerOutput {
    LexerOutput(tokens.0.into_iter().map(|block| {
        let mut in_quote = false;
        let mut updated_block = vec!();
        for token in block.0 {
            match token {
                LexerToken::Quote => {
                    in_quote = !in_quote;
                    updated_block.push(token);
                },
                LexerToken::Period | LexerToken::QuestionMark | LexerToken::ExclamationMark => {
                    if !in_quote {
                        updated_block.push(token);
                    }
                },
                _ => updated_block.push(token)
            }
        }
        LexerBlock(updated_block)
    }).collect::<Vec<_>>())
}

pub fn preprocess(tokens: LexerOutput) -> LexerOutput {
    remove_end_sentence_punctuation_in_quotes(remove_articles_and_possessives(lowercase_all_tokens(tokens)))
}