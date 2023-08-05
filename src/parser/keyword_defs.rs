pub struct KeywordDefs {
    pub to_be: Vec<String>,
    pub positive_adjective: Vec<String>,
    pub negative_adjective: Vec<String>,
    pub said: Vec<String>,
    pub goto: Vec<String>,
    pub positive_comparative_adjective: Vec<String>,
    pub negative_comparative_adjective: Vec<String>
}

pub fn defs() -> KeywordDefs {
    fn to_strings(vector: Vec<&str>) -> Vec<String> {
        vector.into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
    }
    KeywordDefs {
        to_be: to_strings(vec!("was", "were", "is", "are", "wanted to be like", "wants to be like", "wanted to be like")),
        positive_adjective: to_strings(vec!("good", "great", "awesome", "amazing", "fantastic", "wonderful", "incredible", "nice", "cool", "happy", "joyful", "joyous", "glad", "delighted", "pleased", "satisfied", "content", "cheerful", "merry", "jolly", "jovial", "gleeful", "carefree", "sunny", "elated", "exhilarated", "ecstatic", "euphoric", "overjoyed", "exultant", "rapturous", "blissful", "radiant", "thrilled", "ravished")),
        negative_adjective: to_strings(vec!("bad", "terrible", "awful", "horrible", "dreadful", "unpleasant", "unlucky", "displeased", "miserable", "sad", "sorrowful", "dejected", "regretful", "depressed", "downcast", "despondent", "disconsolate", "desolate", "glum", "gloomy", "melancholic", "mournful", "forlorn", "crestfallen", "broken-hearted", "heartbroken", "grief-stricken", "disheartened", "dismayed", "dispirited", "discouraged", "hopeless")),
        said: to_strings(vec!("said", "stated", "exclaimed", "whispered", "shouted", "mumbled", "replied", "responded", "declared", "announced", "asserted", "acknowledged", "conveyed", "uttered", "ventured", "suggested", "disclosed", "protested", "objected", "interjected", "speculated", "greeted", "quoted", "noted", "mentioned", "alledged", "insisted", "confessed", "recited", "pleaded", "concluded", "inquired", "muttered")),
        goto: to_strings(vec!("go to", "goes to", "went to", "gone to", "going to")),
        positive_comparative_adjective: to_strings(vec!("better", "greater", "stronger", "larger")),
        negative_comparative_adjective: to_strings(vec!("worse", "less", "fewer", "smaller"))
    }
}