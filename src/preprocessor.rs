use crate::lexer_types::{LexerOutput, LexerToken, LexerBlock};

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
    remove_end_sentence_punctuation_in_quotes(lowercase_all_tokens(tokens))
}